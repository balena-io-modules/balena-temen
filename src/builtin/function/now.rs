use std::collections::HashMap;

use chrono::{DateTime, Local, Utc};
use serde_json::{Number, Value};

use crate::context::Context;
use crate::error::*;

fn now_with_cached(cached: DateTime<Utc>, args: &HashMap<String, Value>) -> Result<Value> {
    let timestamp = args.get("timestamp").unwrap_or_else(|| &Value::Bool(false));

    let timestamp = timestamp.as_bool().ok_or_else(|| {
        Error::with_message("invalid argument type")
            .context("function", "now")
            .context("argument name", "timestamp")
            .context("argument value", timestamp.to_string())
            .context("expected", "boolean")
    })?;

    let utc = args.get("utc").unwrap_or_else(|| &Value::Bool(false));

    let utc = utc.as_bool().ok_or_else(|| {
        Error::with_message("invalid argument type")
            .context("function", "now")
            .context("argument name", "utc")
            .context("argument value", utc.to_string())
            .context("expected", "boolean")
    })?;

    match (utc, timestamp) {
        (true, true) => Ok(Value::Number(Number::from(cached.timestamp()))),
        (true, false) => Ok(Value::String(cached.to_rfc3339())),
        (false, true) => Ok(Value::Number(Number::from(cached.with_timezone(&Local).timestamp()))),
        (false, false) => Ok(Value::String(cached.with_timezone(&Local).to_rfc3339())),
    }
}

pub(crate) fn now(args: &HashMap<String, Value>, context: &mut Context) -> Result<Value> {
    now_with_cached(context.cached_now(), args)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::{DateTime, Local, NaiveDateTime, Utc};
    use serde_json::{json, Value};

    use crate::context::Context;

    use super::{now, now_with_cached};

    #[test]
    fn subsequent_call_must_return_same_value() {
        let mut ctx = Context::default();
        let args = HashMap::<String, Value>::new();

        assert_eq!(now(&args, &mut ctx).unwrap(), now(&args, &mut ctx).unwrap());
    }

    #[test]
    fn default_argument_values() {
        let mut ctx = Context::default();
        let empty_args = HashMap::<String, Value>::new();

        let mut args = HashMap::new();
        args.insert("utc".to_string(), json!(false));
        args.insert("timestamp".to_string(), json!(false));

        assert_eq!(now(&empty_args, &mut ctx).unwrap(), now(&args, &mut ctx).unwrap());
    }

    #[test]
    fn local_rfc3339_as_default() {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_541_485_381, 0), Utc);
        assert_eq!(
            now_with_cached(dt, &HashMap::new()).unwrap(),
            json!(dt.with_timezone(&Local).to_rfc3339())
        );
    }

    #[test]
    fn utc_argument() {
        let mut args = HashMap::<String, Value>::new();
        args.insert("utc".to_string(), json!(true));

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_541_485_381, 0), Utc);
        assert_eq!(now_with_cached(dt, &args).unwrap(), json!(dt.to_rfc3339()));
        assert_eq!(now_with_cached(dt, &args).unwrap(), json!("2018-11-06T06:23:01+00:00"));
    }

    #[test]
    fn timestamp_argument() {
        let mut args = HashMap::<String, Value>::new();
        args.insert("timestamp".to_string(), json!(true));

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_541_485_381, 0), Utc);
        assert_eq!(now_with_cached(dt, &args).unwrap(), json!(1_541_485_381));
    }

    #[test]
    fn utc_argument_does_not_modify_timestamp() {
        let mut args = HashMap::<String, Value>::new();
        args.insert("timestamp".to_string(), json!(true));

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_541_485_381, 0), Utc);
        assert_eq!(now_with_cached(dt, &args).unwrap(), json!(1_541_485_381));

        args.insert("utc".to_string(), json!(true));
        assert_eq!(now_with_cached(dt, &args).unwrap(), json!(1_541_485_381));
    }

    #[test]
    fn fail_on_invalid_utc_argument_type() {
        let mut ctx = Context::default();
        let mut args = HashMap::new();
        args.insert("utc".to_string(), json!(1));

        assert!(now(&args, &mut ctx).is_err());
    }

    #[test]
    fn fail_on_invalid_timestamp_argument_type() {
        let mut ctx = Context::default();
        let mut args = HashMap::new();
        args.insert("timestamp".to_string(), json!("some-string"));

        assert!(now(&args, &mut ctx).is_err());
    }
}
