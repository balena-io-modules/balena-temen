use chrono::{DateTime, NaiveDateTime, Utc};
use crate::engine::context::Context;
use crate::error::Result;
use serde_json::Value;
use slug;
use std::collections::HashMap;

pub type FilterFn = fn(&Value, &HashMap<String, Value>, &Context) -> Result<Value>;

pub(crate) fn lower(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`lower` filter accepts string only")?;
    Ok(Value::String(s.to_lowercase()))
}

pub(crate) fn upper(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    let s = value.as_str().ok_or_else(|| "`upper` filter accepts string only")?;
    Ok(Value::String(s.to_uppercase()))
}

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

pub(crate) fn trim(value: &Value, _args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    Ok(Value::String(
        value
            .as_str()
            .ok_or_else(|| "`trim` filter accepts string only")?
            .trim()
            .to_string(),
    ))
}

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
