use std::collections::HashMap;

use serde_json::Value;
use slug;

use crate::context::Context;
use crate::error::*;

pub(crate) fn slugify(input: &Value, _args: &HashMap<String, Value>, _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "slugify")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;

    if s.is_empty() {
        return Ok(input.clone());
    }

    let result = slug::slugify(s);
    if result.is_empty() {
        return Err(Error::with_message("empty result, unable to slugify input")
            .context("filter", "slugify")
            .context("input", input.to_string()));
    }

    Ok(Value::String(result))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{slugify, Context};

    #[test]
    fn result_is_trimmed() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert_eq!(slugify(&json!("abc"), &args, &mut ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("abc--def"), &args, &mut ctx).unwrap(), json!("abc-def"));
        assert_eq!(
            slugify(&json!("ěščřžýáíé"), &args, &mut ctx).unwrap(),
            json!("escrzyaie")
        );
        assert_eq!(
            slugify(&json!("Robert & Cyryl"), &args, &mut ctx).unwrap(),
            json!("robert-cyryl")
        );
        assert_eq!(
            slugify(&json!("some white\tspace here"), &args, &mut ctx).unwrap(),
            json!("some-white-space-here")
        );
        assert_eq!(
            slugify(&json!("what about !@#$%^&*()"), &args, &mut ctx).unwrap(),
            json!("what-about")
        );
        assert_eq!(slugify(&json!("-abc"), &args, &mut ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("-abc-"), &args, &mut ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("abc-"), &args, &mut ctx).unwrap(), json!("abc"));
    }

    #[test]
    fn fail_on_invalid_input() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert!(slugify(&json!("-"), &args, &mut ctx).is_err());
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let mut ctx = Context::default();

        assert!(slugify(&json!(1), &args, &mut ctx).is_err());
        assert!(slugify(&json!(1.0), &args, &mut ctx).is_err());
        assert!(slugify(&json!(true), &args, &mut ctx).is_err());
        assert!(slugify(&json!(["a", "b"]), &args, &mut ctx).is_err());
        assert!(slugify(&json!({"a": "b"}), &args, &mut ctx).is_err());
    }
}
