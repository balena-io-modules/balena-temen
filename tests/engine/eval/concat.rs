use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;
use serde_json::json;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

#[test]
fn test_strings() {
    test_eval_eq!("`a` ~ `b` ~ `c`", json!("abc"));
}

#[test]
fn test_string_with_integer() {
    test_eval_eq!("`a` ~ 10", json!("a10"));
}

#[test]
fn test_string_with_float() {
    test_eval_eq!("`a` ~ 9.9", json!("a9.9"));
}
