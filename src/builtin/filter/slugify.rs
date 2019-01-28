use serde_json::Value;
use slug;

use crate::context::Context;
use crate::error::*;

pub(crate) fn slugify(input: &Value, _args: &[Value], _context: &mut Context) -> Result<Value> {
    let s = input.as_str().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", "SLUGIFY")
            .context("expected", "string")
            .context("input", input.to_string())
    })?;

    if s.is_empty() {
        return Ok(input.clone());
    }

    let result = slug::slugify(s);
    if result.is_empty() {
        return Err(Error::with_message("empty result, unable to slugify input")
            .context("filter", "SLUGIFY")
            .context("input", input.to_string()));
    }

    Ok(Value::String(result))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{slugify, Context};

    #[test]
    fn result_is_trimmed() {
        let mut ctx = Context::default();

        assert_eq!(slugify(&json!("abc"), &[], &mut ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("abc--def"), &[], &mut ctx).unwrap(), json!("abc-def"));
        assert_eq!(
            slugify(&json!("ěščřžýáíé"), &[], &mut ctx).unwrap(),
            json!("escrzyaie")
        );
        assert_eq!(
            slugify(&json!("Robert & Cyryl"), &[], &mut ctx).unwrap(),
            json!("robert-cyryl")
        );
        assert_eq!(
            slugify(&json!("some white\tspace here"), &[], &mut ctx).unwrap(),
            json!("some-white-space-here")
        );
        assert_eq!(
            slugify(&json!("what about !@#$%^&*()"), &[], &mut ctx).unwrap(),
            json!("what-about")
        );
        assert_eq!(slugify(&json!("-abc"), &[], &mut ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("-abc-"), &[], &mut ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("abc-"), &[], &mut ctx).unwrap(), json!("abc"));
    }

    #[test]
    fn fail_on_invalid_input() {
        let mut ctx = Context::default();

        assert!(slugify(&json!("-"), &[], &mut ctx).is_err());
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let mut ctx = Context::default();

        assert!(slugify(&json!(1), &[], &mut ctx).is_err());
        assert!(slugify(&json!(1.0), &[], &mut ctx).is_err());
        assert!(slugify(&json!(true), &[], &mut ctx).is_err());
        assert!(slugify(&json!(["a", "b"]), &[], &mut ctx).is_err());
        assert!(slugify(&json!({"a": "b"}), &[], &mut ctx).is_err());
    }
}
