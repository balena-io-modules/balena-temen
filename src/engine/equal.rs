//! `serde_json::Value` implements `PartialEq`, but the implementation does
//! use `==` operator. It's enough for `i64`, `u64`, but definitely not for
//! `f64`.

use approx::Relative;
use serde_json::Number;

pub trait RelativeEq {
    fn relative_eq(&self, other: &Self) -> bool;

    fn relative_ne(&self, other: &Self) -> bool {
        !self.relative_eq(other)
    }
}

impl RelativeEq for Number {
    /// Check the equality of two numbers
    ///
    /// Standard `==` operator is used if both numbers are either `i64` or `u64`
    /// (both must of the same type).
    ///
    /// Numbers are converted to `f64` and `approx::Relative` is used for the
    /// comparison if types differ or at least number is `f64`.
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
