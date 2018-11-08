use balena_temen::engine::context::Context;
use balena_temen::engine::{Engine, EngineBuilder};
use serde_json::{json, Value};
use std::collections::HashMap;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context, None).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval(&$e.parse().unwrap(), &context, None).is_err());
    }};
}

#[test]
fn test_upper() {
    test_eval_eq!("`a` | upper", json!("A"));
    test_eval_eq!("`A` | upper", json!("A"));

    test_eval_err!("1 | upper");
    test_eval_err!("1.0 | upper");
    test_eval_err!("true | upper");
}

#[test]
fn test_lower() {
    test_eval_eq!("`a` | lower", json!("a"));
    test_eval_eq!("`A` | lower", json!("a"));

    test_eval_err!("1 | lower");
    test_eval_err!("1.0 | lower");
    test_eval_err!("true | lower");
}

#[test]
fn test_trim() {
    test_eval_eq!("`a` | trim", json!("a"));
    test_eval_eq!("`a  ` | trim", json!("a"));
    test_eval_eq!("`   a` | trim", json!("a"));
    test_eval_eq!("`   a    ` | trim", json!("a"));

    test_eval_err!("1 | trim");
    test_eval_err!("1.0 | trim");
    test_eval_err!("true | trim");
}

#[test]
fn test_slugify() {
    test_eval_eq!("`abc` | slugify", json!("abc"));
    test_eval_eq!("`abc--def` | slugify", json!("abc-def"));
    test_eval_eq!("`ěščřžýáíé` | slugify", json!("escrzyaie"));
    test_eval_eq!("`Robert & Cyryl` | slugify", json!("robert-cyryl"));
    test_eval_eq!("`some white\tspace here` | slugify", json!("some-white-space-here"));
    test_eval_eq!("`what about !@#$%^&*()` | slugify", json!("what-about"));
    test_eval_eq!("`-abc` | slugify", json!("abc"));
    test_eval_eq!("`-abc-` | slugify", json!("abc"));
    test_eval_eq!("`abc-` | slugify", json!("abc"));

    test_eval_err!("`-` | slugify");

    test_eval_err!("1 | slugify");
    test_eval_err!("1.0 | slugify");
    test_eval_err!("true | slugify");
}

#[test]
fn test_time() {
    test_eval_eq!("1541485381 | time", json!("06:23:01"));
    test_eval_eq!("1541485381 | time == 1541485381 | time(format=`%H:%M:%S`)", json!(true));

    test_eval_eq!("1541485381 | time(format=`%H`)", json!("06"));
    test_eval_eq!("1541485381 | time(format=`%M`)", json!("23"));
    test_eval_eq!("1541485381 | time(format=`%S`)", json!("01"));

    test_eval_err!("1541485381 | time(format=1)");
    test_eval_err!("1541485381 | time(format=true)");
}

#[test]
fn test_date() {
    test_eval_eq!("1541485381 | date", json!("2018-11-06"));
    test_eval_eq!("1541485381 | date == 1541485381 | date(format=`%Y-%m-%d`)", json!(true));

    test_eval_eq!("1541485381 | date(format=`%Y`)", json!("2018"));
    test_eval_eq!("1541485381 | date(format=`%m`)", json!("11"));
    test_eval_eq!("1541485381 | date(format=`%d`)", json!("06"));

    test_eval_err!("1541485381 | date(format=1)");
    test_eval_err!("1541485381 | date(format=true)");
}

#[test]
fn test_date_time() {
    test_eval_eq!("1541485381 | datetime", json!("2018-11-06T06:23:01+00:00"));
    test_eval_eq!(
        "1541485381 | datetime == 1541485381 | datetime(format=`%Y-%m-%dT%H:%M:%S%:z`)",
        json!(true)
    );

    test_eval_eq!("1541485381 | datetime(format=`%Y`)", json!("2018"));
    test_eval_eq!("1541485381 | datetime(format=`%m`)", json!("11"));
    test_eval_eq!("1541485381 | datetime(format=`%d`)", json!("06"));
    test_eval_eq!("1541485381 | datetime(format=`%H`)", json!("06"));
    test_eval_eq!("1541485381 | datetime(format=`%M`)", json!("23"));
    test_eval_eq!("1541485381 | datetime(format=`%S`)", json!("01"));
    test_eval_eq!("1541485381 | datetime(format=`%:z`)", json!("+00:00"));

    test_eval_err!("1541485381 | datetime(format=1)");
    test_eval_err!("1541485381 | datetime(format=true)");
}

#[test]
fn test_filter_chain() {
    test_eval_eq!("`a` | lower | upper", json!("A"));
    test_eval_eq!("`A` | lower | upper", json!("A"));
}

#[test]
fn test_custom_filter() {
    let cf = |value: &Value, _: &HashMap<String, Value>, _: &Context| {
        if value.is_string() {
            Ok(Value::String(value.as_str().unwrap().replace("a", "b")))
        } else {
            Err("no string, no fun".into())
        }
    };

    let engine: Engine = EngineBuilder::default().filter("atob", cf).into();
    let ctx = Context::default();

    assert_eq!(
        engine.eval(&"`abc` | atob".parse().unwrap(), &ctx, None).unwrap(),
        json!("bbc")
    );
    assert!(engine.eval(&"true | atob".parse().unwrap(), &ctx, None).is_err());
}
