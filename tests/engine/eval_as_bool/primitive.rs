use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval_as_bool(&$e.parse().unwrap(), &context, None).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval_as_bool(&$e.parse().unwrap(), &context, None).is_err());
    }};
}

#[test]
fn test_boolean() {
    test_eval_eq!("true", true);
    test_eval_eq!("false", false);
}

#[test]
fn test_string() {
    test_eval_err!("\"\"");
    test_eval_err!("\"hallo\"");
}

#[test]
fn test_integer() {
    test_eval_err!("10");
    test_eval_err!("-12");
    test_eval_err!("0");
}

#[test]
fn test_float() {
    test_eval_err!("10.2");
    test_eval_err!("-3.2");
    test_eval_err!("0.0");
}
