use crate::error::{bail, Result};
use serde_json::Value;

// TODO Sync filters with specification (add filters, add to spec, ...)

pub type FilterFn = Box<dyn Fn(&Value) -> Result<Value>>;

pub(crate) fn lower(value: &Value) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`lower` filter accepts string only")?;
    Ok(Value::String(s.to_lowercase()))
}

pub(crate) fn upper(value: &Value) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`upper` filter accepts string only")?;
    Ok(Value::String(s.to_uppercase()))
}

#[cfg(test)]
mod tests {
    use super::{lower, upper};
    use serde_json::{json, Value};

    macro_rules! test_filter {
        ($f:ident, $e:expr, $r:expr) => {{
            assert_eq!($f(&$e).unwrap(), $r);
        }};
    }

    macro_rules! test_filter_fail {
        ($f:ident, $e:expr) => {{
            assert!($f(&$e).is_err());
        }};
    }

    #[test]
    fn test_lower() {
        test_filter!(lower, json!("ABC"), json!("abc"));
        test_filter!(lower, json!("abc"), json!("abc"));

        test_filter_fail!(lower, json!(true));
        test_filter_fail!(lower, json!(10));
        test_filter_fail!(lower, json!(22.3));
        test_filter_fail!(lower, json!(null));
        test_filter_fail!(lower, json!(["a", "b"]));
        test_filter_fail!(lower, json!({"a": "b"}));
    }

    #[test]
    fn test_upper() {
        test_filter!(upper, json!("ABC"), json!("ABC"));
        test_filter!(upper, json!("abc"), json!("ABC"));

        test_filter_fail!(upper, json!(true));
        test_filter_fail!(upper, json!(10));
        test_filter_fail!(upper, json!(22.3));
        test_filter_fail!(upper, json!(null));
        test_filter_fail!(upper, json!(["a", "b"]));
        test_filter_fail!(upper, json!({"a": "b"}));
    }
}
