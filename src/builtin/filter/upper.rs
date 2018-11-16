use std::collections::HashMap;

use serde_json::Value;

use crate::context::Context;
use crate::error::*;

pub(crate) fn upper(input: &Value, _args: &HashMap<String, Value>, _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "trim")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;
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
        let mut ctx = Context::default();

        assert_eq!(upper(&json!("aBc"), &args, &mut ctx).unwrap(), json!("ABC"));
        assert_eq!(upper(&json!("ABC"), &args, &mut ctx).unwrap(), json!("ABC"));
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert!(upper(&json!(1), &args, &mut ctx).is_err());
        assert!(upper(&json!(1.0), &args, &mut ctx).is_err());
        assert!(upper(&json!(true), &args, &mut ctx).is_err());
        assert!(upper(&json!(["a", "b"]), &args, &mut ctx).is_err());
        assert!(upper(&json!({"a": "b"}), &args, &mut ctx).is_err());
    }
}
