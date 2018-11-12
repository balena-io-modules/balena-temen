use chrono::{DateTime, Utc};

/// Evaluation context
pub struct Context {
    cached_now: Option<DateTime<Utc>>,
}

impl Context {
    pub fn new() -> Context {
        Context { cached_now: None }
    }
}

impl Context {
    /// Current date time
    ///
    /// # Warning
    ///
    /// The result is cached and subsequent calls return same value! This is used
    /// by the `now()` function, which must return same value within one context.
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
    fn default() -> Context {
        Context::new()
    }
}
