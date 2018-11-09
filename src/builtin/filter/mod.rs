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

pub type FilterFn = fn(&Value, &HashMap<String, Value>, &Context) -> Result<Value>;
