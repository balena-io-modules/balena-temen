use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

pub(crate) fn trim(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    Ok(Value::String(
        value
            .as_str()
            .ok_or_else(|| "`trim` filter accepts string only")?
            .trim()
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{Context, trim};

    #[test]
    fn result_is_trimmed() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(trim(&json!("a    "), &args, &ctx).unwrap(), json!("a"));
        assert_eq!(trim(&json!("    a"), &args, &ctx).unwrap(), json!("a"));
        assert_eq!(trim(&json!("    a    "), &args, &ctx).unwrap(), json!("a"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert!(trim(&json!(1), &args, &ctx).is_err());
        assert!(trim(&json!(1.0), &args, &ctx).is_err());
        assert!(trim(&json!(true), &args, &ctx).is_err());
        assert!(trim(&json!(["a", "b"]), &args, &ctx).is_err());
        assert!(trim(&json!({"a": "b"}), &args, &ctx).is_err());
    }
}
