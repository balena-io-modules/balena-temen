use std::collections::HashMap;

use serde_json::Value;

use crate::engine::context::Context;
use crate::error::Result;

pub(crate) fn upper(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`upper` filter accepts string only")?;
    Ok(Value::String(s.to_uppercase()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{Context, upper};

    #[test]
    fn result_is_upper_cased() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(upper(&json!("aBc"), &args, &ctx).unwrap(), json!("ABC"));
        assert_eq!(upper(&json!("ABC"), &args, &ctx).unwrap(), json!("ABC"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert!(upper(&json!(1), &args, &ctx).is_err());
        assert!(upper(&json!(1.0), &args, &ctx).is_err());
        assert!(upper(&json!(true), &args, &ctx).is_err());
        assert!(upper(&json!(["a", "b"]), &args, &ctx).is_err());
        assert!(upper(&json!({"a": "b"}), &args, &ctx).is_err());
    }
}
