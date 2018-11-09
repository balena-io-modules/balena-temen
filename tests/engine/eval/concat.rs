use serde_json::json;

use balena_temen::{Context, Engine};

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context, None).unwrap(), $r);
    }};
}

#[test]
fn multiple_strings() {
    test_eval_eq!("`a` ~ `b`", json!("ab"));
    test_eval_eq!("`a` ~ `b` ~ `c`", json!("abc"));
}

#[test]
fn string_with_integer() {
    test_eval_eq!("`a` ~ 10", json!("a10"));
}

#[test]
fn string_with_float() {
    test_eval_eq!("`a` ~ 9.9", json!("a9.9"));
}
