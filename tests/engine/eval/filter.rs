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

macro_rules! test_eval_fail {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval(&$e.parse().unwrap(), &context).is_err());
    }};
}

#[test]
fn test_upper() {
    test_eval!("`a` | upper", json!("A"));
    test_eval!("`A` | upper", json!("A"));

    test_eval_fail!("1 | upper");
    test_eval_fail!("1.0 | upper");
    test_eval_fail!("true | upper");
}

#[test]
fn test_lower() {
    test_eval!("`a` | lower", json!("a"));
    test_eval!("`A` | lower", json!("a"));

    test_eval_fail!("1 | lower");
    test_eval_fail!("1.0 | lower");
    test_eval_fail!("true | lower");
}

#[test]
fn test_filter_chain() {
    test_eval!("`a` | lower | upper", json!("A"));
    test_eval!("`A` | lower | upper", json!("A"));
}
