use crate::error::{bail, Result};
use crate::parser::ast::*;
use serde_json::Value;

pub struct Context {
    #[allow(dead_code)]
    data: Value,
}

impl Context {
    pub fn new(data: Value) -> Context {
        Context { data }
    }

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
