use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

pub(crate) use self::now::now;
pub(crate) use self::uuidv4::uuidv4;

mod now;
mod uuidv4;

pub type FunctionFn = fn(args: &HashMap<String, Value>, context: &mut Context) -> Result<Value>;
