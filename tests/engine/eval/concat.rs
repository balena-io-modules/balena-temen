use balena_template::engine::context::Context;
use balena_template::engine::Engine;
use serde_json::json;

macro_rules! test_eval {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

#[test]
fn test_strings() {
    test_eval!("`a` ~ `b` ~ `c`", json!("abc"));
}

#[test]
fn test_string_with_integer() {
    test_eval!("`a` ~ 10", json!("a10"));
}

#[test]
fn test_string_with_float() {
    test_eval!("`a` ~ 9.9", json!("a9.9"));
}
