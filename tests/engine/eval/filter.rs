use balena_temen::engine::context::Context;
use balena_temen::engine::{Engine, EngineBuilder};
use serde_json::{json, Value};

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval(&$e.parse().unwrap(), &context).is_err());
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
fn test_filter_chain() {
    test_eval_eq!("`a` | lower | upper", json!("A"));
    test_eval_eq!("`A` | lower | upper", json!("A"));
}

#[test]
fn test_custom_filter() {
    let cf = |value: &Value, _: &Context| {
        if value.is_string() {
            Ok(Value::String(value.as_str().unwrap().replace("a", "b")))
        } else {
            Err("no string, no fun".into())
        }
    };

    let engine: Engine = EngineBuilder::default().filter("atob", cf).into();
    let ctx = Context::default();

    assert_eq!(
        engine.eval(&"`abc` | atob".parse().unwrap(), &ctx).unwrap(),
        json!("bbc")
    );
    assert!(engine.eval(&"true | atob".parse().unwrap(), &ctx).is_err());
}
