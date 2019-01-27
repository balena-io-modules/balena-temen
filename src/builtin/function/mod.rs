use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

pub(crate) use self::now::now;
pub(crate) use self::uuidv4::uuidv4;

pub(crate) mod math;
mod now;
mod uuidv4;

/// Evaluation engine function signature
///
/// You can register custom function with the [`function`] method.
///
/// # Arguments
///
/// * `args` - List of arguments
/// * `context` - An evaluation context
///
/// # Examples
///
/// `random()`:
///
/// * `random` - function name
/// * `args` - empty slice
///
/// `random(1024)`
///
/// * `random` - function name
/// * `args` - first slice element contains the `Value::Number(1024)` value
///
/// Visit [`function`] method documentation to see how to register custom function
/// and how it should look like.
///
/// [`Engine`]: struct.Engine.html
/// [`function`]: struct.EngineBuilder.html#method.function
pub type FunctionFn = fn(args: &[Value], context: &mut Context) -> Result<Value>;
