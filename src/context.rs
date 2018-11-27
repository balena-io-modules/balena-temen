use chrono::{DateTime, Utc};
#[cfg(target_arch = "wasm32")]
use chrono::NaiveDateTime;

#[cfg(not(target_arch = "wasm32"))]
fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

// chrono crate doesn't support wasm32 arch yet, workaround
#[cfg(target_arch = "wasm32")]
fn utc_now() -> DateTime<Utc> {
    let now = js_sys::Date::new_0();
    let millisecs_since_unix_epoch: u64 = now.get_time() as u64;
    let secs = millisecs_since_unix_epoch / 1000;
    let nanos = 1_000_000 * (millisecs_since_unix_epoch - 1000 * secs);
    let naive = NaiveDateTime::from_timestamp(secs as i64, nanos as u32);
    DateTime::from_utc(naive, Utc)
}

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

        let x = utc_now();
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
