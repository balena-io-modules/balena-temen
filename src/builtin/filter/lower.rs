use serde_json::Value;

use crate::context::Context;
use crate::error::*;

pub(crate) fn lower(input: &Value, _args: &[Value], _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "LOWER")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;
    Ok(Value::String(s.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{lower, Context};

    #[test]
    fn result_is_lower_cased() {
        let mut ctx = Context::default();

        assert_eq!(lower(&json!("aBc"), &[], &mut ctx).unwrap(), json!("abc"));
        assert_eq!(lower(&json!("ABC"), &[], &mut ctx).unwrap(), json!("abc"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let mut ctx = Context::default();

        assert!(lower(&json!(1), &[], &mut ctx).is_err());
        assert!(lower(&json!(1.0), &[], &mut ctx).is_err());
        assert!(lower(&json!(true), &[], &mut ctx).is_err());
        assert!(lower(&json!(["a", "b"]), &[], &mut ctx).is_err());
        assert!(lower(&json!({"a": "b"}), &[], &mut ctx).is_err());
    }
}
