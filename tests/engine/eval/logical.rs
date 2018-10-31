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
fn test_equal() {
    test_eval!("true == true", json!(true));
    test_eval!("true == false", json!(false));

    test_eval!("1 == 1", json!(true));
    test_eval!("1 == 2", json!(false));

    test_eval!("1.0 == 1", json!(true));
    test_eval!("1.0 == 2", json!(false));

    test_eval!("`abc` == 'abc'", json!(true));
    test_eval!("`abc` == 'ab'", json!(false));
}

#[test]
fn test_equal_with_filter() {
    test_eval!("`abc` | upper == 'ABC'", json!(true));
    test_eval!("`ABC` | lower == 'abc'", json!(true));
    test_eval!("`ABC` == 'abc' | upper", json!(true));
    test_eval!("`abc` == 'ABC' | lower", json!(true));
}

#[test]
fn test_not_equal() {
    test_eval!("true != true", json!(false));
    test_eval!("true != false", json!(true));

    test_eval!("1 != 1", json!(false));
    test_eval!("1 != 2", json!(true));

    test_eval!("1.0 != 1", json!(false));
    test_eval!("1.0 != 2", json!(true));

    test_eval!("`abc` != 'abc'", json!(false));
    test_eval!("`abc` != 'ab'", json!(true));
}

#[test]
fn test_not() {
    test_eval!("not false", json!(true));
    test_eval!("not 1 + 1 == 3", json!(true));
}

#[test]
fn test_greater_than() {
    test_eval!("1 > 1", json!(false));
    test_eval!("1 > 2", json!(false));
    test_eval!("3 > 2", json!(true));

    test_eval!("3.0 > 2", json!(true));
    test_eval!("3 > 2.0", json!(true));
}

#[test]
fn test_greater_than_or_equal() {
    test_eval!("1 >= 1", json!(true));
    test_eval!("1 >= 2", json!(false));
    test_eval!("3 >= 2", json!(true));

    test_eval!("3.0 >= 2", json!(true));
    test_eval!("3 >= 2.0", json!(true));
}

#[test]
fn test_lower() {
    test_eval!("1 < 1", json!(false));
    test_eval!("1 < 2", json!(true));
    test_eval!("3 < 2", json!(false));

    test_eval!("3.0 < 2", json!(false));
    test_eval!("3 < 2.0", json!(false));
}

#[test]
fn test_lower_or_equal() {
    test_eval!("1 <= 1", json!(true));
    test_eval!("1 <= 2", json!(true));
    test_eval!("3 <= 2", json!(false));

    test_eval!("3.0 <= 2", json!(false));
    test_eval!("3 <= 2.0", json!(false));
}

#[test]
fn test_and() {
    test_eval!("true and true", json!(true));
    test_eval!("true and false", json!(false));

    test_eval!("1 + 1 == 2 and true", json!(true));
    test_eval!("1 + 1 > 1 and 3 < 2", json!(false));
}

#[test]
fn test_or() {
    test_eval!("true or true", json!(true));
    test_eval!("true or false", json!(true));

    test_eval!("1 + 1 == 2 or false", json!(true));
    test_eval!("1 + 1 > 1 or 3 < 2", json!(true));
}
