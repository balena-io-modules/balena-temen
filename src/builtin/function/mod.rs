use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

pub(crate) use self::now::now;
pub(crate) use self::uuidv4::uuidv4;

mod now;
mod uuidv4;

/// Evaluation engine function signature
///
/// You can register custom function with the [`function`] method.
///
/// # Arguments
///
/// * `args` - A function arguments
/// * `context` - An evaluation context
///
/// # Examples
///
/// `random()`:
///
/// * `random` - function name
/// * `args` - empty map
///
/// `random(max=1024)`
///
/// * `random` - function name
/// * `args` - map contains `max` key with the `Value::Number(1024)` value
///
/// [`Engine`]: struct.Engine.html
/// [`function`]: struct.EngineBuilder.html#method.function
pub type FunctionFn = fn(args: &HashMap<String, Value>, context: &mut Context) -> Result<Value>;
