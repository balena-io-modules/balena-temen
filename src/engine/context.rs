use chrono::{DateTime, Utc};
use crate::error::{bail, Result};
use crate::parser::ast::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Internal {
    cached_now: Option<DateTime<Utc>>,
}

impl Internal {
    fn cached_now(&mut self) -> DateTime<Utc> {
        if let Some(x) = self.cached_now {
            return x;
        }

        let x = Utc::now();
        self.cached_now = Some(x);
        x
    }
}

struct Lookup<'a> {
    root: &'a Value,
    values: Vec<&'a Value>,
}

impl<'a> Lookup<'a> {
    fn new(root: &'a Value) -> Lookup<'a> {
        Lookup {
            root,
            values: vec![root],
        }
    }

    fn lookup_with_identifier_value(
        &mut self,
        identifier: &IdentifierValue,
        position: Option<&Identifier>,
    ) -> Result<()> {
        let last_value = self
            .values
            .last()
            .ok_or_else(|| "unable to lookup variable: not last value = invalid identifier")?;

        match identifier {
            IdentifierValue::Name(ref name) | IdentifierValue::StringIndex(ref name) => {
                let new_value = last_value
                    .as_object()
                    .ok_or_else(|| "unable to lookup variable: not an object")
                    .and_then(|x| {
                        x.get(name)
                            .ok_or_else(|| "unable to lookup variable: key does not exist")
                    })?;
                self.values.push(new_value);
            }
            IdentifierValue::This => {
                // Do nothing, this refers to self
            }
            IdentifierValue::Super => {
                self.values
                    .pop()
                    .ok_or_else(|| "unable to lookup variable: can't go back")?;
            }
            IdentifierValue::IntegerIndex(idx) => {
                let new_value = last_value
                    .as_array()
                    .ok_or_else(|| "unable to lookup variable: not an array")
                    .and_then(|x| {
                        let mut index = *idx;

                        if index < 0 {
                            index += x.len() as isize
                        }

                        if index < 0 {
                            bail!("unable to lookup variable: index out of bounds")
                        }

                        x.get(index as usize)
                            .ok_or_else(|| "unable to lookup variable: index out of bounds")
                    })?;
                self.values.push(new_value);
            }
            IdentifierValue::IdentifierIndex(ref identifier) => {
                match Lookup::lookup_identifier(self.root, identifier, position)? {
                    Value::String(ref x) => {
                        self.lookup_with_identifier_value(&IdentifierValue::StringIndex(x.to_string()), position)?
                    }
                    Value::Number(ref x) => {
                        let idx = x
                            .as_i64()
                            .ok_or_else(|| "unable to lookup variable: invalid integer index")?;

                        self.lookup_with_identifier_value(&IdentifierValue::IntegerIndex(idx as isize), position)?;
                    }
                    _ => bail!("unable to lookup variable: result of indirect lookup is not a number / string"),
                };
            }
        };

        Ok(())
    }

    fn lookup_identifier(root: &'a Value, identifier: &Identifier, position: Option<&Identifier>) -> Result<&'a Value> {
        let mut lookup = Lookup::new(root);

        if let Some(first_value) = identifier.values.first() {
            if first_value == &IdentifierValue::This || first_value == &IdentifierValue::Super {
                if let Some(position) = position {
                    for position_value in position.values.iter() {
                        lookup.lookup_with_identifier_value(position_value, Some(&position))?;
                    }
                } else {
                    bail!("lookup_identifier: unable to lookup relative identifier with position");
                }
            }
        } else {
            bail!("lookup_identifier: empty identifier, nothing to lookup");
        }

        for identifier_value in identifier.values.iter() {
            lookup.lookup_with_identifier_value(identifier_value, position)?;
        }

        Ok(lookup.values.last().ok_or_else(|| "no value?")?)
    }
}

pub struct Context {
    data: Value,
    position: Option<Identifier>,
    internal: Arc<Mutex<Internal>>,
}

impl Context {
    pub fn new(data: Value) -> Context {
        Context {
            data,
            position: None,
            internal: Arc::new(Mutex::new(Internal::default())),
        }
    }

    pub fn new_with_position(data: Value, position: Identifier) -> Context {
        Context {
            data,
            position: Some(position),
            internal: Arc::new(Mutex::new(Internal::default())),
        }
    }
}

impl Context {
    /// Get current date time
    ///
    /// # Warning
    ///
    /// The result is cached and subsequent calls return same value!
    pub(crate) fn cached_now(&self) -> DateTime<Utc> {
        self.internal.lock().unwrap().cached_now()
    }
}

impl Context {
    pub(crate) fn lookup_variable(&self, identifier: &Identifier) -> Result<&Value> {
        Lookup::lookup_identifier(&self.data, identifier, self.position.as_ref())
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new(Value::Null)
    }
}
