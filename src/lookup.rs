use serde_json::Value;

use crate::ast::*;
use crate::error::{bail, Result};

/// Lookup identifier
pub trait Lookup {
    /// Lookup identifier (variable) value
    ///
    /// # Arguments
    ///
    /// * `identifier` - An identifier (variable) to lookup
    /// * `position` - An initial position for relative lookups
    fn lookup_identifier(&self, identifier: &Identifier, position: &Identifier) -> Result<&Value>;
}

impl Lookup for Value {
    fn lookup_identifier(&self, identifier: &Identifier, position: &Identifier) -> Result<&Value> {
        let mut lookup = LookupStack::new(self);

        if identifier.is_relative() {
            // Update stack with initial position
            for position_value in position.values.iter() {
                lookup.update_with_identifier_value(position_value, position)?;
            }
        }

        // Update stack with either relative / absolute identifier, stack is prepared for both
        for identifier_value in identifier.values.iter() {
            lookup.update_with_identifier_value(identifier_value, position)?;
        }

        Ok(lookup
            .stack
            .last()
            .ok_or_else(|| "lookup_identifier: unable to lookup identifier, empty stack")?)
    }
}

/// Provide a way to lookup an identifier (variable) value
pub struct LookupStack<'a> {
    /// Whole structure (JSON) with variable values
    root: &'a Value,
    /// Stack of values for every identifier component (variable name, array index, ...)
    stack: Vec<&'a Value>,
}

impl<'a> LookupStack<'a> {
    pub fn new(root: &'a Value) -> LookupStack<'a> {
        LookupStack {
            root,
            stack: vec![root],
        }
    }

    /// Update stack with next identifier value
    ///
    /// `position` is required for relative identifier values only (`This`, `Super`).
    /// This argument is ignored if an identifier is absolute.
    ///
    /// # Arguments
    ///
    /// * `identifier_value` - Next identifier component to lookup
    /// * `position` - Initial position for relative lookup
    pub fn update_with_identifier_value(
        &mut self,
        identifier_value: &IdentifierValue,
        position: &Identifier,
    ) -> Result<()> {
        let last_value = self
            .stack
            .last()
            .ok_or_else(|| "update_with_identifier_value: invalid identifier")?;

        match identifier_value {
            IdentifierValue::Name(ref name) | IdentifierValue::StringIndex(ref name) => {
                // Name (networks) and StringIndex (["networks"]) equals
                let new_value = last_value
                    .as_object()
                    .ok_or_else(|| "update_with_identifier_value: not an object".to_string())
                    .and_then(|x| {
                        x.get(name)
                            .ok_or_else(|| format!("update_with_identifier_value: key `{}` does not exist", name))
                    })?;
                self.stack.push(new_value);
            }
            IdentifierValue::This => {
                // Do nothing, `this` refers to self
            }
            IdentifierValue::Super => {
                // Pop the last stack value, `super` refers to parent
                self.stack
                    .pop()
                    .ok_or_else(|| "update_with_identifier_value: invalid `super` usage, no parent object")?;
            }
            IdentifierValue::IntegerIndex(idx) => {
                // Array index
                let new_value = last_value
                    .as_array()
                    .ok_or_else(|| "update_with_identifier_value: not an array")
                    .and_then(|x| {
                        let mut index = *idx;

                        // Normalize negative index where -1 means last element, etc.
                        if index < 0 {
                            index += x.len() as isize
                        }

                        if index < 0 {
                            bail!("update_with_identifier_value: index out of bounds")
                        }

                        x.get(index as usize)
                            .ok_or_else(|| "update_with_identifier_value: index out of bounds")
                    })?;
                self.stack.push(new_value);
            }
            IdentifierValue::IdentifierIndex(ref identifier) => {
                // IdentifierIndex is like indirect lookup, identifier within identifier
                // people[boss.id].name - boss.id = IdentifierIndex
                //
                // We have to create new Lookup structure and lookup this identifier
                // from scratch to avoid existing stack modifications
                match self.root.lookup_identifier(identifier, position)? {
                    // If we were able to lookup the value, treat it as an String or Number index
                    Value::String(ref x) => {
                        self.update_with_identifier_value(&IdentifierValue::StringIndex(x.to_string()), position)?
                    }
                    Value::Number(ref x) => {
                        let idx = x
                            .as_i64()
                            .ok_or_else(|| "update_with_identifier_value: invalid integer index")?;

                        self.update_with_identifier_value(&IdentifierValue::IntegerIndex(idx as isize), position)?;
                    }
                    _ => bail!("update_with_identifier_value: result of indirect lookup is not a number / string"),
                };
            }
        };

        Ok(())
    }
}
