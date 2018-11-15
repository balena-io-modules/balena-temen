//! Error handling
//!
//! A module allowing us to create more detailed, context aware, errors.
//!
//! # Examples
//!
//! Invalid UTC argument type:
//!
//! ```rust
//! use balena_temen::{
//!     ast::Identifier,
//!     Engine, Context, Value,
//!     error::*
//! };
//! use std::collections::HashMap;
//!
//! let engine: Engine = Engine::default();
//! let mut ctx = Context::default();
//! let position = Identifier::default();
//! let data = Value::Null;
//!
//! eprintln!("{}", engine.eval("now(utc=`yes`)", &position, &data, &mut ctx).err().unwrap());
//! ```
//!
//! Printed error:
//!
//! ```text
//! temen: invalid argument type
//!  └ context:
//!     ├ function = now
//!     ├ argument name = utc
//!     ├ argument value = "yes"
//!     └ expected = boolean
//! ```
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::result;

/// Standard library result wrapper
pub type Result<T> = result::Result<T, Error>;

/// Context key type
pub type Key = Cow<'static, str>;
/// Context value type
pub type Value = Cow<'static, str>;

/// Result extension
pub trait ResultExt<T> {
    /// Appends key, value pair to the context
    ///
    /// # Arguments
    ///
    /// * `k` - A key
    /// * `v` - A value
    fn context<K, V>(self, k: K, v: V) -> Result<T>
    where
        K: Into<Key>,
        V: Into<Value>;
}

impl<T> ResultExt<T> for Result<T> {
    fn context<K, V>(self, k: K, v: V) -> Result<T>
    where
        K: Into<Key>,
        V: Into<Value>,
    {
        self.map_err(|e| e.context(k, v))
    }
}

/// Error type
pub struct Error {
    inner: Box<Inner>,
}

impl Error {
    /// Creates new error with message
    ///
    /// # Arguments
    ///
    /// * `message` - An error message
    pub fn with_message(message: &'static str) -> Error {
        let inner = Inner::new(message);
        Error { inner: Box::new(inner) }
    }

    /// Adds key, value pair to the error's context
    ///
    /// # Arguments
    ///
    /// * `k` - A key
    /// * `v` - A value
    pub fn context<K, V>(mut self, k: K, v: V) -> Error
    where
        K: Into<Key>,
        V: Into<Value>,
    {
        self.inner.push(k, v);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "temen: {}", self.inner.message)?;
        if !self.inner.context.is_empty() {
            writeln!(f, " └ context:")?;
            let last_index = self.inner.context().len() - 1;
            for (idx, (k, v)) in self.inner.context().iter().enumerate() {
                if idx == last_index {
                    writeln!(f, "    └ {} = {}", k, v)?;
                } else {
                    writeln!(f, "    ├ {} = {}", k, v)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(error::Error + 'static)> {
        None
    }
}

struct Inner {
    message: &'static str,
    context: Vec<(Key, Value)>,
}

impl Inner {
    fn new(message: &'static str) -> Inner {
        Inner {
            message,
            context: vec![],
        }
    }

    fn push<K, V>(&mut self, k: K, v: V)
    where
        K: Into<Key>,
        V: Into<Value>,
    {
        self.context.push((k.into(), v.into()))
    }

    fn context(&self) -> &[(Key, Value)] {
        self.context.as_ref()
    }
}
