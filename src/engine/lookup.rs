use std::borrow::Cow;

use serde_json::Value;

use crate::ast::*;
use crate::error::*;

/// Provide a way to lookup an identifier (variable) value
pub struct Lookup<'a> {
    /// Whole structure (JSON) with variable values
    data: &'a Value,
    /// Stack of values for every identifier component (variable name, array index, ...)
    stack: Vec<&'a Value>,
}

/// Checks that the lookup does not end up with an object containing `eval_keyword`
fn validate_not_for_evaluation(value: &Value, eval_keyword: &str) -> Result<()> {
    if let Value::Object(object) = value {
        if object.contains_key(eval_keyword) {
            return Err(Error::with_message("unable to lookup identifier"));
        }
    }
    Ok(())
}

impl<'a> Lookup<'a> {
    pub fn new(data: &'a Value) -> Lookup<'a> {
        Lookup {
            data,
            stack: vec![data],
        }
    }

    /// Lookup identifier (variable) value
    ///
    /// # Arguments
    ///
    /// * `data` - Variable values (whole JSON)
    /// * `identifier` - An identifier (variable) to lookup
    /// * `position` - An initial position for relative lookups
    /// * `eval_keyword` - An evaluation keyword
    pub fn lookup_identifier<'b>(
        data: &'b Value,
        identifier: &Identifier,
        position: &Identifier,
        eval_keyword: &str,
    ) -> Result<Cow<'b, Value>> {
        let mut lookup = Lookup::new(data);

        let canonical = identifier.canonicalize(position)?;

        for identifier_value in canonical.values.iter() {
            lookup.update_with_identifier_value(identifier_value, position, eval_keyword)?;
        }

        let result = Cow::Borrowed(lookup.stack.pop().ok_or_else(|| {
            Error::with_message("unable to lookup identifier").context("reason", "empty stack = invalid identifier")
        })?);
        Ok(result)
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
    /// * `eval_keyword` - An evaluation keyword
    pub fn update_with_identifier_value(
        &mut self,
        identifier_value: &IdentifierValue,
        position: &Identifier,
        eval_keyword: &str,
    ) -> Result<()> {
        let last_value = self.stack.last().ok_or_else(|| {
            Error::with_message("unable to lookup identifier").context("reason", "empty stack = invalid identifier")
        })?;

        match identifier_value {
            IdentifierValue::Name(ref name) => {
                let new_value = last_value
                    .as_object()
                    .ok_or_else(|| {
                        Error::with_message("unable to lookup identifier")
                            .context("reason", "parent value is not an object")
                            .context("name", name.to_string())
                    })
                    .and_then(|x| {
                        x.get(name).ok_or_else(|| {
                            Error::with_message("unable to lookup identifier")
                                .context("reason", "field does not exist")
                                .context("name", name.to_string())
                                .context("object", format!("{:?}", x))
                        })
                    })?;
                validate_not_for_evaluation(new_value, eval_keyword)?;
                self.stack.push(new_value);
            }
            IdentifierValue::This => {
                // Do nothing, `this` refers to self
            }
            IdentifierValue::Super => {
                // Pop the last stack value, `super` refers to parent
                self.stack.pop().ok_or_else(|| {
                    Error::with_message("unable to lookup identifier")
                        .context("reason", "super must not be used in the root")
                })?;
            }
            IdentifierValue::Index(idx) => {
                // Array index
                let new_value = last_value
                    .as_array()
                    .ok_or_else(|| {
                        Error::with_message("unable to lookup identifier")
                            .context("reason", "parent value is not an array")
                            .context("index", format!("{}", idx))
                    })
                    .and_then(|x| {
                        let mut index = *idx;

                        // Normalize negative index where -1 means last element, etc.
                        if index < 0 {
                            index += x.len() as isize
                        }

                        if index < 0 {
                            return Err(Error::with_message("unable to lookup identifier")
                                .context("reason", "invalid index")
                                .context("index", format!("{}", index)));
                        }

                        x.get(index as usize).ok_or_else(|| {
                            Error::with_message("unable to lookup identifier")
                                .context("reason", "index out of bounds")
                                .context("index", format!("{}", index))
                                .context("array", format!("{:?}", x))
                        })
                    })?;
                validate_not_for_evaluation(&new_value, eval_keyword)?;
                self.stack.push(new_value);
            }
            IdentifierValue::Identifier(ref identifier) => {
                // Identifier is like indirect lookup, identifier within identifier
                // people[boss.id].name - boss.id = Identifier to lookup
                //
                // We have to create new Lookup structure and lookup this identifier
                // from scratch to avoid existing stack modifications
                match Lookup::lookup_identifier(self.data, identifier, position, eval_keyword)?.as_ref() {
                    // If we were able to lookup the value, treat it as an String or Number index
                    Value::String(ref x) => self.update_with_identifier_value(
                        &IdentifierValue::Name(x.to_string()),
                        position,
                        eval_keyword,
                    )?,
                    Value::Number(ref x) => {
                        let idx = x.as_i64().ok_or_else(|| {
                            Error::with_message("unable to lookup identifier")
                                .context("reason", "invalid index")
                                .context("index", format!("{:?}", x))
                        })?;

                        self.update_with_identifier_value(
                            &IdentifierValue::Index(idx as isize),
                            position,
                            eval_keyword,
                        )?;
                    }
                    _ => {
                        return Err(Error::with_message("unable to lookup identifier")
                            .context("reason", "identifier does not point to an integer / string")
                            .context("identifier", format!("{:?}", identifier))
                            .context("position", format!("{:?}", position)))
                    }
                };
            }
        };

        Ok(())
    }
}
