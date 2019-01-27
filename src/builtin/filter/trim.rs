use serde_json::Value;

use crate::context::Context;
use crate::error::*;

pub(crate) fn trim(input: &Value, _args: &[Value], _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "TRIM")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;

    Ok(Value::String(s.trim().to_string()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{trim, Context};

    #[test]
    fn result_is_trimmed() {
        let mut ctx = Context::default();

        assert_eq!(trim(&json!("a    "), &[], &mut ctx).unwrap(), json!("a"));
        assert_eq!(trim(&json!("    a"), &[], &mut ctx).unwrap(), json!("a"));
        assert_eq!(trim(&json!("    a    "), &[], &mut ctx).unwrap(), json!("a"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let mut ctx = Context::default();

        assert!(trim(&json!(1), &[], &mut ctx).is_err());
        assert!(trim(&json!(1.0), &[], &mut ctx).is_err());
        assert!(trim(&json!(true), &[], &mut ctx).is_err());
        assert!(trim(&json!(["a", "b"]), &[], &mut ctx).is_err());
        assert!(trim(&json!({"a": "b"}), &[], &mut ctx).is_err());
    }
}
