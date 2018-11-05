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

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval(&$e.parse().unwrap(), &context).is_err());
    }};
}

// TODO Add better comparison of numbers, especially floats

#[test]
fn test_integer_math_operations() {
    test_eval_eq!("1 + 1", json!(2));
    test_eval_eq!("2 - 1", json!(1));
    test_eval_eq!("3 * 3", json!(9));
    test_eval_eq!("6 / 3", json!(2));
    test_eval_eq!("8 % 3", json!(2));
}

#[test]
fn test_float_math_operation() {
    test_eval_eq!("1.0 + 1.1", json!(2.1));
    test_eval_eq!("2.0 - 1.3", json!(0.7));
    test_eval_eq!("3.5 * 2.0", json!(7.0));
    test_eval_eq!("6.1 / 3.05", json!(2.0));
    test_eval_eq!("8.0 % 3.0", json!(2.0));
}

#[test]
fn test_math_operator_precedence() {
    test_eval_eq!("1 + 2 * 3", json!(7));
    test_eval_eq!("1 - 2 * 3", json!(-5));
    test_eval_eq!("1 + 6 / 3", json!(3));
    test_eval_eq!("1 - 6 / 3", json!(-1));
    test_eval_eq!("1 + 6 % 3", json!(1));
    test_eval_eq!("1 - 6 % 3", json!(1));
}

#[test]
fn test_zero_division() {
    test_eval_err!("8 / 0");
    test_eval_err!("8.0 % 0.0");
}

#[test]
fn test_division_result_as_integer() {
    test_eval_eq!("8 / 2", json!(4));
    test_eval_eq!("1000 / 5", json!(200));
}

#[test]
fn test_division_result_as_float() {
    test_eval_eq!("9 / 2", json!(4.5));
    test_eval_eq!("8.0 / 2.0", json!(4.0));
}

#[test]
fn test_integer_operation_boundaries() {
    // If engine can't do i64 math, values are converted to f64
    test_eval_eq!(
        format!("{} + {}", std::i64::MAX, std::i64::MAX),
        json!(std::i64::MAX as f64 + std::i64::MAX as f64)
    );
    test_eval_eq!(
        format!("{} - {}", std::i64::MIN, std::i64::MAX),
        json!(std::i64::MIN as f64 - std::i64::MAX as f64)
    );
    test_eval_eq!(
        format!("{} * {}", std::i64::MAX, std::i64::MAX),
        json!(std::i64::MAX as f64 * std::i64::MAX as f64)
    );
}

#[test]
fn test_float_operation_boundaries() {
    test_eval_err!(format!("{:.1} + {:.1}", std::f64::MAX, std::f64::MAX));
    test_eval_err!(format!("{:.1} - {:.1}", std::f64::MIN, std::f64::MAX));
    test_eval_err!(format!("{:.1} * {:.1}", std::f64::MAX, std::f64::MAX));
}
