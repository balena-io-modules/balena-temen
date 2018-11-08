use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;
use serde_json::json;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &context, None).unwrap(), $r);
    }};
}

#[test]
fn test_equal() {
    test_eval_eq!("true == true", json!(true));
    test_eval_eq!("true == false", json!(false));

    test_eval_eq!("1 == 1", json!(true));
    test_eval_eq!("1 == 2", json!(false));

    test_eval_eq!("1.0 == 1", json!(true));
    test_eval_eq!("1.0 == 2", json!(false));

    test_eval_eq!("`abc` == 'abc'", json!(true));
    test_eval_eq!("`abc` == 'ab'", json!(false));
}

#[test]
fn test_equal_with_filter() {
    test_eval_eq!("`abc` | upper == 'ABC'", json!(true));
    test_eval_eq!("`ABC` | lower == 'abc'", json!(true));
    test_eval_eq!("`ABC` == 'abc' | upper", json!(true));
    test_eval_eq!("`abc` == 'ABC' | lower", json!(true));
}

#[test]
fn test_not_equal() {
    test_eval_eq!("true != true", json!(false));
    test_eval_eq!("true != false", json!(true));

    test_eval_eq!("1 != 1", json!(false));
    test_eval_eq!("1 != 2", json!(true));

    test_eval_eq!("1.0 != 1", json!(false));
    test_eval_eq!("1.0 != 2", json!(true));

    test_eval_eq!("`abc` != 'abc'", json!(false));
    test_eval_eq!("`abc` != 'ab'", json!(true));
}

#[test]
fn test_not() {
    test_eval_eq!("not false", json!(true));
    test_eval_eq!("not 1 + 1 == 3", json!(true));
}

#[test]
fn test_greater_than() {
    test_eval_eq!("1 > 1", json!(false));
    test_eval_eq!("1 > 2", json!(false));
    test_eval_eq!("3 > 2", json!(true));

    test_eval_eq!("3.0 > 2", json!(true));
    test_eval_eq!("3 > 2.0", json!(true));
}

#[test]
fn test_greater_than_or_equal() {
    test_eval_eq!("1 >= 1", json!(true));
    test_eval_eq!("1 >= 2", json!(false));
    test_eval_eq!("3 >= 2", json!(true));

    test_eval_eq!("3.0 >= 2", json!(true));
    test_eval_eq!("3 >= 2.0", json!(true));
}

#[test]
fn test_lower() {
    test_eval_eq!("1 < 1", json!(false));
    test_eval_eq!("1 < 2", json!(true));
    test_eval_eq!("3 < 2", json!(false));

    test_eval_eq!("3.0 < 2", json!(false));
    test_eval_eq!("3 < 2.0", json!(false));
}

#[test]
fn test_lower_or_equal() {
    test_eval_eq!("1 <= 1", json!(true));
    test_eval_eq!("1 <= 2", json!(true));
    test_eval_eq!("3 <= 2", json!(false));

    test_eval_eq!("3.0 <= 2", json!(false));
    test_eval_eq!("3 <= 2.0", json!(false));
}

#[test]
fn test_and() {
    test_eval_eq!("true and true", json!(true));
    test_eval_eq!("true and false", json!(false));

    test_eval_eq!("1 + 1 == 2 and true", json!(true));
    test_eval_eq!("1 + 1 > 1 and 3 < 2", json!(false));
}

#[test]
fn test_or() {
    test_eval_eq!("true or true", json!(true));
    test_eval_eq!("true or false", json!(true));

    test_eval_eq!("1 + 1 == 2 or false", json!(true));
    test_eval_eq!("1 + 1 > 1 or 3 < 2", json!(true));
}
