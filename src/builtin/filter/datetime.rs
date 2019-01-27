use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;

use crate::context::Context;
use crate::error::*;

fn format_timestamp(filter: &'static str, input: &Value, args: &[Value], default: &str) -> Result<Value> {
    let ts = input.as_i64().ok_or_else(|| {
        Error::with_message("invalid input type")
            .context("filter", filter)
            .context("expected", "i64")
            .context("input", input.to_string())
    })?;

    let format = match args.first() {
        Some(x) => x.as_str().ok_or_else(|| {
            Error::with_message("invalid argument type")
                .context("filter", filter)
                .context("argument name", "format")
                .context("argument value", x.to_string())
                .context("expected", "string")
        })?,
        None => default,
    };

    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc);

    Ok(Value::String(dt.format(format).to_string()))
}

pub(crate) fn time(input: &Value, args: &[Value], _context: &mut Context) -> Result<Value> {
    format_timestamp("TIME", input, args, "%H:%M:%S")
}

pub(crate) fn date(input: &Value, args: &[Value], _context: &mut Context) -> Result<Value> {
    format_timestamp("DATE", input, args, "%Y-%m-%d")
}

pub(crate) fn datetime(input: &Value, args: &[Value], _context: &mut Context) -> Result<Value> {
    format_timestamp("DATETIME", input, args, "%Y-%m-%dT%H:%M:%S%:z")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::context::Context;

    use super::{date, datetime, format_timestamp, time};

    #[test]
    fn time_default_format() {
        let mut ctx = Context::default();

        assert_eq!(time(&json!(1_541_485_381), &[], &mut ctx).unwrap(), json!("06:23:01"));
    }

    #[test]
    fn date_default_format() {
        let mut ctx = Context::default();

        assert_eq!(date(&json!(1_541_485_381), &[], &mut ctx).unwrap(), json!("2018-11-06"));
    }

    #[test]
    fn datetime_default_format() {
        let mut ctx = Context::default();

        assert_eq!(
            datetime(&json!(1_541_485_381), &[], &mut ctx).unwrap(),
            json!("2018-11-06T06:23:01+00:00")
        );
    }

    #[test]
    fn format_argument() {
        let args = vec![json!("%Y")];

        assert_eq!(
            format_timestamp("", &json!(1_541_485_381), &args, "").unwrap(),
            json!("2018")
        );

        let args = vec![json!("%m")];
        assert_eq!(
            format_timestamp("", &json!(1_541_485_381), &args, "").unwrap(),
            json!("11")
        );

        let args = vec![json!("%d")];
        assert_eq!(
            format_timestamp("", &json!(1_541_485_381), &args, "").unwrap(),
            json!("06")
        );

        let args = vec![json!("%H")];
        assert_eq!(
            format_timestamp("", &json!(1_541_485_381), &args, "").unwrap(),
            json!("06")
        );

        let args = vec![json!("%M")];
        assert_eq!(
            format_timestamp("", &json!(1_541_485_381), &args, "").unwrap(),
            json!("23")
        );

        let args = vec![json!("%S")];
        assert_eq!(
            format_timestamp("", &json!(1_541_485_381), &args, "").unwrap(),
            json!("01")
        );
    }

    #[test]
    fn fail_on_invalid_format_argument_type() {
        let args = vec![json!(1)];
        assert!(format_timestamp("", &json!(1_541_485_381), &args, "").is_err());

        let args = vec![json!(1.2)];
        assert!(format_timestamp("", &json!(1_541_485_381), &args, "").is_err());

        let args = vec![json!(true)];
        assert!(format_timestamp("", &json!(1_541_485_381), &args, "").is_err());

        let args = vec![json!(["a", "b"])];
        assert!(format_timestamp("", &json!(1_541_485_381), &args, "").is_err());

        let args = vec![json!({"a": "b"})];
        assert!(format_timestamp("", &json!(1_541_485_381), &args, "").is_err());
    }
}
