use balena_template::engine::context::Context;
use balena_template::engine::Engine;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval_as_bool(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

#[test]
fn test_logical_equal() {
    test_eval_eq!("true == true", true);
    test_eval_eq!("1 == 1", true);
    test_eval_eq!("2.3 == 2.3", true);
    test_eval_eq!("`a` == `a`", true);
    test_eval_eq!("`a` == `b`", false);
    test_eval_eq!("`1` == 1", false);
}

#[test]
fn test_logical_not_equal() {
    test_eval_eq!("true != true", false);
    test_eval_eq!("1 != 1", false);
    test_eval_eq!("2.3 != 2.3", false);
    test_eval_eq!("`a` != `a`", false);
    test_eval_eq!("`a` != `b`", true);
    test_eval_eq!("`1` != 1", true);
}

#[test]
fn test_relational_greater_than() {
    test_eval_eq!("1 > 2", false);
    test_eval_eq!("3 > 2", true);
    test_eval_eq!("3.1 > 2", true);
}

#[test]
fn test_relational_greater_than_or_equal() {
    test_eval_eq!("1 >= 2", false);
    test_eval_eq!("3 >= 2", true);
    test_eval_eq!("3.1 >= 2", true);
    test_eval_eq!("3.1 >= 3.1", true);
    test_eval_eq!("3 >= 3.0", true);
}

#[test]
fn test_relational_lower_than() {
    test_eval_eq!("1 < 2", true);
    test_eval_eq!("3 < 2", false);
    test_eval_eq!("3.1 < 2", false);
    test_eval_eq!("3.1 < 3.1", false);
    test_eval_eq!("3 < 3.0", false);
}

#[test]
fn test_relational_lower_than_or_equal() {
    test_eval_eq!("1 <= 2", true);
    test_eval_eq!("3 <= 2", false);
    test_eval_eq!("2 <= 3.1", true);
    test_eval_eq!("3.1 <= 3.1", true);
    test_eval_eq!("3 <= 3.0", true);
}
