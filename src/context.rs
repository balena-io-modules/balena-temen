use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::error::Result;
use crate::lookup::Lookup;
use crate::parser::ast::*;

/// Internal context structure
///
/// It's in a separate structure because these data are mutable and
/// the whole structure is behind `Arc` & `Mutex`. Then we can
/// mutate it even if it's behind immutable `Context`.
///
/// Not sure if it's a good idea yet, because the whole project is in
/// early stage, evolving pretty quickly, ...
#[derive(Default)]
struct Internal {
    cached_now: Option<DateTime<Utc>>,
}

impl Internal {
    /// Generate current date time or return cached one
    ///
    /// Subsequent calls to this function return same date time.
    fn cached_now(&mut self) -> DateTime<Utc> {
        if let Some(x) = self.cached_now {
            return x;
        }

        let x = Utc::now();
        self.cached_now = Some(x);
        x
    }
}

/// Evaluation context
pub struct Context {
    /// Variable values, whole JSON
    data: Value,
    /// Internal data structure
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
    /// Current date time
    ///
    /// # Warning
    ///
    /// The result is cached and subsequent calls return same value! This is used
    /// by the `now()` function, which must return same value within one context.
    pub(crate) fn cached_now(&self) -> DateTime<Utc> {
        self.internal.lock().unwrap().cached_now()
    }
}

impl Context {
    /// Lookup identifier (variable) value
    ///
    /// # Arguments
    ///
    /// * `identifier` - An identifier to lookup value for
    /// * `position` - An position for relative identifier lookup (ignored for absolute identifier)
    pub(crate) fn lookup_identifier(&self, identifier: &Identifier, position: Option<&Identifier>) -> Result<&Value> {
        Lookup::lookup_identifier(&self.data, identifier, position)
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new(Value::Null)
    }
}
