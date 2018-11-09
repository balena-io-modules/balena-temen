use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;

use crate::context::Context;
use crate::error::Result;

fn format_timestamp(filter: &str, value: &Value, args: &HashMap<String, Value>, default: &str) -> Result<Value> {
    let ts = value
        .as_i64()
        .ok_or_else(|| format!("`{}` accepts integer only", filter))?;

    let format = match args.get("format") {
        Some(x) => x
            .as_str()
            .ok_or_else(|| format!("`{}` format must be a string", filter))?,
        None => default,
    };

    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc);

    Ok(Value::String(dt.format(format).to_string()))
}

pub(crate) fn time(value: &Value, args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    format_timestamp("time", value, args, "%H:%M:%S")
}

pub(crate) fn date(value: &Value, args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    format_timestamp("date", value, args, "%Y-%m-%d")
}

pub(crate) fn datetime(value: &Value, args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    format_timestamp("datetime", value, args, "%Y-%m-%dT%H:%M:%S%:z")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::context::Context;

    use super::{date, datetime, format_timestamp, time};

    #[test]
    fn time_default_format() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(time(&json!(1541485381), &args, &ctx).unwrap(), json!("06:23:01"));
    }

    #[test]
    fn date_default_format() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(date(&json!(1541485381), &args, &ctx).unwrap(), json!("2018-11-06"));
    }

    #[test]
    fn datetime_default_format() {
        let args = HashMap::new();
        let ctx = Context::default();

        assert_eq!(
            datetime(&json!(1541485381), &args, &ctx).unwrap(),
            json!("2018-11-06T06:23:01+00:00")
        );
    }

    #[test]
    fn format_argument() {
        let mut args = HashMap::new();

        args.insert("format".to_string(), json!("%Y"));
        assert_eq!(
            format_timestamp("", &json!(1541485381), &args, "").unwrap(),
            json!("2018")
        );

        args.insert("format".to_string(), json!("%m"));
        assert_eq!(
            format_timestamp("", &json!(1541485381), &args, "").unwrap(),
            json!("11")
        );

        args.insert("format".to_string(), json!("%d"));
        assert_eq!(
            format_timestamp("", &json!(1541485381), &args, "").unwrap(),
            json!("06")
        );

        args.insert("format".to_string(), json!("%H"));
        assert_eq!(
            format_timestamp("", &json!(1541485381), &args, "").unwrap(),
            json!("06")
        );

        args.insert("format".to_string(), json!("%M"));
        assert_eq!(
            format_timestamp("", &json!(1541485381), &args, "").unwrap(),
            json!("23")
        );

        args.insert("format".to_string(), json!("%S"));
        assert_eq!(
            format_timestamp("", &json!(1541485381), &args, "").unwrap(),
            json!("01")
        );
    }

    #[test]
    fn fail_on_invalid_format_argument_type() {
        let mut args = HashMap::new();

        args.insert("format".to_string(), json!(1));
        assert!(format_timestamp("", &json!(1541485381), &args, "").is_err());

        args.insert("format".to_string(), json!(1.2));
        assert!(format_timestamp("", &json!(1541485381), &args, "").is_err());

        args.insert("format".to_string(), json!(true));
        assert!(format_timestamp("", &json!(1541485381), &args, "").is_err());

        args.insert("format".to_string(), json!(["a", "b"]));
        assert!(format_timestamp("", &json!(1541485381), &args, "").is_err());

        args.insert("format".to_string(), json!({"a": "b"}));
        assert!(format_timestamp("", &json!(1541485381), &args, "").is_err());
    }
}
