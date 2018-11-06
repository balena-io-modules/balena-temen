use chrono::{DateTime, Local, Utc};
use crate::engine::context::Context;
use crate::error::Result;
use serde_json::{Number, Value};
use std::collections::HashMap;
use uuid::Uuid;

pub type FunctionFn = fn(&HashMap<String, Value>, &Context) -> Result<Value>;

pub(crate) fn uuidv4(_args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    Ok(Value::String(Uuid::new_v4().to_hyphenated().to_string()))
}

fn now_with_cached(cached: DateTime<Utc>, args: &HashMap<String, Value>) -> Result<Value> {
    let timestamp = args
        .get("timestamp")
        .unwrap_or_else(|| &Value::Bool(false))
        .as_bool()
        .ok_or_else(|| "timestamp must be a boolean value")?;

    let utc = args
        .get("utc")
        .unwrap_or_else(|| &Value::Bool(false))
        .as_bool()
        .ok_or_else(|| "utc must be a boolean value")?;

    match (utc, timestamp) {
        (true, true) => Ok(Value::Number(Number::from(cached.timestamp()))),
        (true, false) => Ok(Value::String(cached.to_rfc3339())),
        (false, true) => Ok(Value::Number(Number::from(cached.with_timezone(&Local).timestamp()))),
        (false, false) => Ok(Value::String(cached.with_timezone(&Local).to_rfc3339())),
    }
}

pub(crate) fn now(args: &HashMap<String, Value>, context: &Context) -> Result<Value> {
    now_with_cached(context.cached_now(), args)
}

#[cfg(test)]
mod tests {
    use super::{now, now_with_cached, uuidv4, Context, HashMap};
    use chrono::{DateTime, Local, NaiveDateTime, Utc};
    use serde_json::{json, Value};
    use uuid::{Uuid, Version};

    macro_rules! hashmap(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(
                    m.insert($key.into(), $value);
                )+
                m
            }
         };
    );

    #[test]
    fn test_uuidv4() {
        // Just a simple test that checks if UUIDv4 is generated
        let uuid_value = uuidv4(&HashMap::new(), &Context::default()).unwrap();
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid = Uuid::parse_str(&uuid_str).unwrap();
        // Version::Random == UUIDv4
        assert_eq!(uuid.get_version(), Some(Version::Random));
    }

    #[test]
    fn test_now_invalid_argument_type() {
        let ctx = Context::default();

        let invalid_timestamp_arg = hashmap!("timestamp" => Value::String("hallo".to_string()));
        assert!(now(&invalid_timestamp_arg, &ctx).is_err());

        let invalid_utc_arg = hashmap!("utc" => Value::Null);
        assert!(now(&invalid_utc_arg, &ctx).is_err());
    }

    #[test]
    fn test_now() {
        let cached: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1541485380, 0), Utc);
        assert_eq!(
            now_with_cached(cached, &HashMap::new()).unwrap(),
            json!(cached.with_timezone(&Local).to_rfc3339())
        );
    }

    #[test]
    fn test_now_utc() {
        let cached: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1541485380, 0), Utc);
        assert_eq!(
            now_with_cached(cached, &hashmap!("utc" => Value::Bool(true))).unwrap(),
            json!(cached.with_timezone(&Utc).to_rfc3339())
        );
        assert_eq!(
            now_with_cached(cached, &hashmap!("utc" => Value::Bool(true))).unwrap(),
            json!("2018-11-06T06:23:00+00:00")
        );
    }

    #[test]
    fn test_now_timestamp() {
        // There's no such a thing as local timestamp, it's always unix timestamp (UTC)
        // If timestamp is true then utc is ignored
        let cached: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1541485380, 0), Utc);
        assert_eq!(
            now_with_cached(
                cached,
                &hashmap!("timestamp" => Value::Bool(true), "utc" => Value::Bool(true))
            )
            .unwrap(),
            json!(1541485380)
        );
        assert_eq!(
            now_with_cached(
                cached,
                &hashmap!("timestamp" => Value::Bool(true), "utc" => Value::Bool(false))
            )
            .unwrap(),
            json!(1541485380)
        );
    }

    #[test]
    fn test_now_defaults() {
        let cached: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1541485380, 0), Utc);
        assert_eq!(
            now_with_cached(cached, &HashMap::new()).unwrap(),
            now_with_cached(
                cached,
                &hashmap!("timestamp" => Value::Bool(false), "utc" => Value::Bool(false))
            )
            .unwrap()
        );
    }
}
