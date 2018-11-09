use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

pub(crate) fn lower(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`lower` filter accepts string only")?;
    Ok(Value::String(s.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{Context, lower};

    #[test]
    fn result_is_lower_cased() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(lower(&json!("aBc"), &args, &ctx).unwrap(), json!("abc"));
        assert_eq!(lower(&json!("ABC"), &args, &ctx).unwrap(), json!("abc"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert!(lower(&json!(1), &args, &ctx).is_err());
        assert!(lower(&json!(1.0), &args, &ctx).is_err());
        assert!(lower(&json!(true), &args, &ctx).is_err());
        assert!(lower(&json!(["a", "b"]), &args, &ctx).is_err());
        assert!(lower(&json!({"a": "b"}), &args, &ctx).is_err());
    }
}
