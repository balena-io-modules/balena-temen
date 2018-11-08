use std::collections::HashMap;

use serde_json::Value;
use slug;

use crate::engine::context::Context;
use crate::error::Result;

pub(crate) fn slugify(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`slugify` filter accepts string only")?;

    if s.is_empty() {
        return Ok(value.clone());
    }

    let result = slug::slugify(s);
    if result.is_empty() {
        return Err(format!("unable to slugify `{}`", s).into());
    }

    Ok(Value::String(result))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{Context, slugify};

    #[test]
    fn result_is_trimmed() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(slugify(&json!("abc"), &args, &ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("abc--def"), &args, &ctx).unwrap(), json!("abc-def"));
        assert_eq!(
            slugify(&json!("ěščřžýáíé"), &args, &ctx).unwrap(),
            json!("escrzyaie")
        );
        assert_eq!(
            slugify(&json!("Robert & Cyryl"), &args, &ctx).unwrap(),
            json!("robert-cyryl")
        );
        assert_eq!(
            slugify(&json!("some white\tspace here"), &args, &ctx).unwrap(),
            json!("some-white-space-here")
        );
        assert_eq!(
            slugify(&json!("what about !@#$%^&*()"), &args, &ctx).unwrap(),
            json!("what-about")
        );
        assert_eq!(slugify(&json!("-abc"), &args, &ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("-abc-"), &args, &ctx).unwrap(), json!("abc"));
        assert_eq!(slugify(&json!("abc-"), &args, &ctx).unwrap(), json!("abc"));
    }

    #[test]
    fn fail_on_invalid_input() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert!(slugify(&json!("-"), &args, &ctx).is_err());
    }

    #[test]
    fn fail_on_invalid_input_type() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert!(slugify(&json!(1), &args, &ctx).is_err());
        assert!(slugify(&json!(1.0), &args, &ctx).is_err());
        assert!(slugify(&json!(true), &args, &ctx).is_err());
        assert!(slugify(&json!(["a", "b"]), &args, &ctx).is_err());
        assert!(slugify(&json!({"a": "b"}), &args, &ctx).is_err());
    }
}
