use crate::error::Result;
use serde_json::Value;

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
    use serde_json::json;

    macro_rules! test_filter_eq {
        ($f:ident, $e:expr, $r:expr) => {{
            assert_eq!($f(&$e).unwrap(), $r);
        }};
    }

    macro_rules! test_filter_err {
        ($f:ident, $e:expr) => {{
            assert!($f(&$e).is_err());
        }};
    }

    #[test]
    fn test_lower() {
        test_filter_eq!(lower, json!("ABC"), json!("abc"));
        test_filter_eq!(lower, json!("abc"), json!("abc"));

        test_filter_err!(lower, json!(true));
        test_filter_err!(lower, json!(10));
        test_filter_err!(lower, json!(22.3));
        test_filter_err!(lower, json!(null));
        test_filter_err!(lower, json!(["a", "b"]));
        test_filter_err!(lower, json!({"a": "b"}));
    }

    #[test]
    fn test_upper() {
        test_filter_eq!(upper, json!("ABC"), json!("ABC"));
        test_filter_eq!(upper, json!("abc"), json!("ABC"));

        test_filter_err!(upper, json!(true));
        test_filter_err!(upper, json!(10));
        test_filter_err!(upper, json!(22.3));
        test_filter_err!(upper, json!(null));
        test_filter_err!(upper, json!(["a", "b"]));
        test_filter_err!(upper, json!({"a": "b"}));
    }
}
