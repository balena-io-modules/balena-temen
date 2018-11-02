// Value implements PartialEq, but the implementation does use == operator.
// It's not enough for doubles (f64), especially when templating engine
// does math operations as well. We have to compare doubles in a better way.

use approx::Relative;
use serde_json::Number;

pub trait RelativeEq {
    fn relative_eq(&self, other: &Self) -> bool;

    fn relative_ne(&self, other: &Self) -> bool {
        !self.relative_eq(other)
    }
}

impl RelativeEq for Number {
    /// Check equality of two numbers
    ///
    /// # Notes
    ///
    /// Operator `==` is used for `i64` & `u64` types. `Relative` (from `approx` crate)
    /// is used for `f64` comparison.
    ///
    /// # Arguments
    ///
    /// * `other` - A number to compare `self` with
    fn relative_eq(&self, other: &Number) -> bool {
        if self.is_i64() && other.is_i64() {
            return self.as_i64().unwrap() == other.as_i64().unwrap();
        }

        if self.is_u64() && other.is_u64() {
            return self.as_u64().unwrap() == other.as_u64().unwrap();
        }

        Relative::default().eq(&self.as_f64().unwrap(), &other.as_f64().unwrap())
    }
}
