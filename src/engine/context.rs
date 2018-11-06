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

pub struct Context {
    data: Value,
    internal: Arc<Mutex<Internal>>,
}

impl Context {
    pub fn new(data: Value) -> Context {
        Context {
            data,
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
    fn lookup_variable_value<'a>(&self, value: &'a Value, identifier: &IdentifierValue) -> Result<&'a Value> {
        match identifier {
            IdentifierValue::Name(ref id) | IdentifierValue::StringIndex(ref id) => value
                .as_object()
                .ok_or_else(|| "unable to lookup variable: not an object".into())
                .and_then(|x| {
                    x.get(id)
                        .ok_or_else(|| "unable to lookup variable: key does not exist".into())
                }),
            IdentifierValue::IntegerIndex(idx) => value
                .as_array()
                .ok_or_else(|| "unable to lookup variable: not an array".into())
                .and_then(|x| {
                    let mut index = *idx;

                    if index < 0 {
                        index += x.len() as isize
                    }

                    if index < 0 {
                        bail!("unable to lookup variable: index out of bounds")
                    }

                    x.get(index as usize)
                        .ok_or_else(|| "unable to lookup variable: index out of bounds".into())
                }),
            IdentifierValue::IdentifierIndex(ref id) => match self.lookup_variable(id)? {
                Value::String(ref x) => self.lookup_variable_value(value, &IdentifierValue::StringIndex(x.to_string())),
                Value::Number(ref x) => x
                    .as_i64()
                    .ok_or_else(|| "unable to lookup variable: invalid integer index".into())
                    .and_then(|idx| self.lookup_variable_value(value, &IdentifierValue::IntegerIndex(idx as isize))),
                _ => bail!("unable to lookup variable: result of indirect lookup is not a number / string"),
            },
            IdentifierValue::This => unimplemented!(),
            IdentifierValue::Super => unimplemented!(),
        }
    }

    pub(crate) fn lookup_variable(&self, identifier: &Identifier) -> Result<&Value> {
        let mut result = &self.data;

        for iv in &identifier.values {
            result = self.lookup_variable_value(result, iv)?
        }

        Ok(result)
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new(Value::Null)
    }
}
