use chrono::{DateTime, Utc};

/// An evaluation context
///
/// Context is required for a template evaluation. It holds cached values
/// like date time for the `now()` function, which must return same value
/// in case the same evaluation context is used.
pub struct Context {
    cached_now: Option<DateTime<Utc>>,
}

impl Context {
    /// Current date time
    ///
    /// # Warning
    ///
    /// The result is cached and subsequent calls return same value! This is used
    /// by the `now()` function for example.
    pub(crate) fn cached_now(&mut self) -> DateTime<Utc> {
        if let Some(x) = self.cached_now {
            return x;
        }

        let x = Utc::now();
        self.cached_now = Some(x);
        x
    }
}

impl Default for Context {
    /// Creates new, empty, context
    fn default() -> Context {
        Context { cached_now: None }
    }
}
