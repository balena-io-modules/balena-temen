use std::collections::HashMap;

use serde_json::json;

use balena_temen::{Context, Engine, EngineBuilder, Value};

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context, None).unwrap(), $r);
    }};
}

#[test]
fn default_filters_are_registered() {
    // All filters have unit tests and it's enough to test if they're called / registered / work
    test_eval_eq!("1541485381 | time", json!("06:23:01"));
    test_eval_eq!("1541485381 | date", json!("2018-11-06"));
    test_eval_eq!("1541485381 | datetime", json!("2018-11-06T06:23:01+00:00"));
    test_eval_eq!("`A` | lower", json!("a"));
    test_eval_eq!("`A` | slugify", json!("a"));
    test_eval_eq!("`A` | trim", json!("A"));
    test_eval_eq!("`a` | upper", json!("A"));
}

#[test]
fn filter_chain() {
    test_eval_eq!("`a` | lower | upper", json!("A"));
    test_eval_eq!("`A` | lower | upper", json!("A"));
}

#[test]
fn fail_on_unknown_filter() {
    let engine = Engine::default();
    let context = Context::default();

    assert!(engine
        .eval(
            &"1 | filterdoesnotexistoratleastitshouldnot".parse().unwrap(),
            &context,
            None
        )
        .is_err());
}

#[test]
fn custom_filter() {
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
