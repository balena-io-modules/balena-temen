use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

pub(crate) use self::datetime::{date, datetime, time};
pub(crate) use self::lower::lower;
pub(crate) use self::slugify::slugify;
pub(crate) use self::trim::trim;
pub(crate) use self::upper::upper;

mod datetime;
mod lower;
mod slugify;
mod trim;
mod upper;

/// Evaluation engine filter signature
///
/// You can register custom filter with the [`filter`] method.
///
/// # Arguments
///
/// * `input` - An input value
/// * `args` - A function arguments
/// * `context` - An evaluation context
///
/// # Examples
///
/// `"hallo" | upper`:
///
/// * `"hallo"` - filter `input`
/// * `upper` - filter name
/// * `args` - empty map
///
/// `"hallo" | upper(trim=true)`
///
/// * `"hallo"` - filter `input`
/// * `upper` - filter name
/// * `args` - map contains `trim` key with the `Value::Boolean(true)` value
///
/// Visit [`filter`] method documentation to see how to register custom function
/// and how it should look like.
///
/// [`Engine`]: struct.Engine.html
/// [`filter`]: struct.EngineBuilder.html#method.filter
pub type FilterFn = fn(input: &Value, args: &HashMap<String, Value>, context: &mut Context) -> Result<Value>;
