use serde_json::Value;

use crate::context::Context;
use crate::error::*;

pub(crate) fn upper(input: &Value, _args: &[Value], _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "UPPER")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;
    Ok(Value::String(s.to_uppercase()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{upper, Context};

    #[test]
    fn result_is_upper_cased() {
        let mut ctx = Context::default();

        assert_eq!(upper(&json!("aBc"), &[], &mut ctx).unwrap(), json!("ABC"));
        assert_eq!(upper(&json!("ABC"), &[], &mut ctx).unwrap(), json!("ABC"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let mut ctx = Context::default();

        assert!(upper(&json!(1), &[], &mut ctx).is_err());
        assert!(upper(&json!(1.0), &[], &mut ctx).is_err());
        assert!(upper(&json!(true), &[], &mut ctx).is_err());
        assert!(upper(&json!(["a", "b"]), &[], &mut ctx).is_err());
        assert!(upper(&json!({"a": "b"}), &[], &mut ctx).is_err());
    }
}
