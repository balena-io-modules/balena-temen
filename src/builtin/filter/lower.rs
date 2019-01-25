use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::*;

pub(crate) fn lower(input: &Value, _args: &HashMap<String, Value>, _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "lower")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;
    Ok(Value::String(s.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{lower, Context};

    #[test]
    fn result_is_lower_cased() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert_eq!(lower(&json!("aBc"), &args, &mut ctx).unwrap(), json!("abc"));
        assert_eq!(lower(&json!("ABC"), &args, &mut ctx).unwrap(), json!("abc"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert!(lower(&json!(1), &args, &mut ctx).is_err());
        assert!(lower(&json!(1.0), &args, &mut ctx).is_err());
        assert!(lower(&json!(true), &args, &mut ctx).is_err());
        assert!(lower(&json!(["a", "b"]), &args, &mut ctx).is_err());
        assert!(lower(&json!({"a": "b"}), &args, &mut ctx).is_err());
    }
}
