use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::*;

pub(crate) fn trim(input: &Value, _args: &HashMap<String, Value>, _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "trim")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;

    Ok(Value::String(s.trim().to_string()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{trim, Context};

    #[test]
    fn result_is_trimmed() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert_eq!(trim(&json!("a    "), &args, &mut ctx).unwrap(), json!("a"));
        assert_eq!(trim(&json!("    a"), &args, &mut ctx).unwrap(), json!("a"));
        assert_eq!(trim(&json!("    a    "), &args, &mut ctx).unwrap(), json!("a"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert!(trim(&json!(1), &args, &mut ctx).is_err());
        assert!(trim(&json!(1.0), &args, &mut ctx).is_err());
        assert!(trim(&json!(true), &args, &mut ctx).is_err());
        assert!(trim(&json!(["a", "b"]), &args, &mut ctx).is_err());
        assert!(trim(&json!({"a": "b"}), &args, &mut ctx).is_err());
    }
}
