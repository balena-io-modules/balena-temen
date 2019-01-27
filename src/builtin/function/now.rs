use chrono::{DateTime, Utc};
use serde_json::{Number, Value};

use crate::context::Context;
use crate::error::*;

fn now_with_cached(cached: DateTime<Utc>, args: &[Value]) -> Result<Value> {
    let timestamp = args.get(0).unwrap_or_else(|| &Value::Bool(false));

    let timestamp = timestamp.as_bool().ok_or_else(|| {
        Error::with_message("invalid argument type")
            .context("function", "NOW")
            .context("argument", "1st")
            .context("argument value", timestamp.to_string())
            .context("expected", "boolean")
    })?;

    if timestamp {
        Ok(Value::Number(Number::from(cached.timestamp())))
    } else {
        Ok(Value::String(cached.to_rfc3339()))
    }
}

pub(crate) fn now(args: &[Value], context: &mut Context) -> Result<Value> {
    now_with_cached(context.cached_now(), args)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde_json::json;

    use crate::context::Context;

    use super::{now, now_with_cached};

    #[test]
    fn subsequent_call_must_return_same_value() {
        let mut ctx = Context::default();
        assert_eq!(now(&[], &mut ctx).unwrap(), now(&[], &mut ctx).unwrap());
    }

    #[test]
    fn default_argument_values() {
        let mut ctx = Context::default();
        let args = vec![json!(false)];
        assert_eq!(now(&[], &mut ctx).unwrap(), now(&args, &mut ctx).unwrap());
    }

    #[test]
    fn utc_rfc3339_as_default() {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_541_485_381, 0), Utc);
        assert_eq!(now_with_cached(dt, &[]).unwrap(), json!(dt.to_rfc3339()));
        assert_eq!(now_with_cached(dt, &[]).unwrap(), json!("2018-11-06T06:23:01+00:00"));
    }

    #[test]
    fn timestamp_argument() {
        let args = vec![json!(true)];

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_541_485_381, 0), Utc);
        assert_eq!(now_with_cached(dt, &args).unwrap(), json!(1_541_485_381));
    }

    #[test]
    fn fail_on_invalid_timestamp_argument_type() {
        let mut ctx = Context::default();
        let args = vec![json!("foo")];
        assert!(now(&args, &mut ctx).is_err());
    }
}
