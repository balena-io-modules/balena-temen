use crate::error::{bail, Result};

/// Check that the float is not infinite or NaN
///
/// It's guaranteed that the result of this function can be used with
/// `Number::from_64(...).unwrap()` without panic.
///
/// # Arguments
///
/// * `number` - A number to check
pub fn validate_f64(number: f64) -> Result<f64> {
    if number.is_nan() {
        bail!("parse_f64: NaN not supported");
    }

    if number.is_infinite() {
        bail!("parse_f64: infinite not supported");
    }

    Ok(number)
}
