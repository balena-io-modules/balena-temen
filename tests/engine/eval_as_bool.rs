use balena_template::engine::context::Context;
use balena_template::engine::Engine;

macro_rules! test_as_bool {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval_as_bool(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

#[test]
fn test_boolean() {
    test_as_bool!("true", true);
    test_as_bool!("false", false);
}

#[test]
fn test_string() {
    test_as_bool!("\"\"", false);
    test_as_bool!("\"hallo\"", true);
}

#[test]
fn test_integer() {
    test_as_bool!("10", true);
    test_as_bool!("-12", true);
    test_as_bool!("0", false);
}

#[test]
fn test_float() {
    test_as_bool!("10.2", true);
    test_as_bool!("-3.2", true);
    test_as_bool!("0.0", false);
}

#[test]
fn test_logical_and() {
    test_as_bool!("true and true", true);
    test_as_bool!("true and false", false);
    test_as_bool!("false and true", false);
    test_as_bool!("false and false", false);
}

#[test]
fn test_logical_or() {
    test_as_bool!("true or true", true);
    test_as_bool!("true or false", true);
    test_as_bool!("false or true", true);
    test_as_bool!("false or false", false);
}

#[test]
fn test_logical_equal() {
    test_as_bool!("true == true", true);
    test_as_bool!("1 == 1", true);
    test_as_bool!("2.3 == 2.3", true);
    test_as_bool!("`a` == `a`", true);
    test_as_bool!("`a` == `b`", false);
    test_as_bool!("`1` == 1", false);
}

#[test]
fn test_logical_not_equal() {
    test_as_bool!("true != true", false);
    test_as_bool!("1 != 1", false);
    test_as_bool!("2.3 != 2.3", false);
    test_as_bool!("`a` != `a`", false);
    test_as_bool!("`a` != `b`", true);
    test_as_bool!("`1` != 1", true);
}

#[test]
fn test_relational_greater_than() {
    test_as_bool!("1 > 2", false);
    test_as_bool!("3 > 2", true);
    test_as_bool!("3.1 > 2", true);
}

#[test]
fn test_relational_greater_than_or_equal() {
    test_as_bool!("1 >= 2", false);
    test_as_bool!("3 >= 2", true);
    test_as_bool!("3.1 >= 2", true);
    test_as_bool!("3.1 >= 3.1", true);
    test_as_bool!("3 >= 3.0", true);
}

#[test]
fn test_relational_lower_than() {
    test_as_bool!("1 < 2", true);
    test_as_bool!("3 < 2", false);
    test_as_bool!("3.1 < 2", false);
    test_as_bool!("3.1 < 3.1", false);
    test_as_bool!("3 < 3.0", false);
}

#[test]
fn test_relational_lower_than_or_equal() {
    test_as_bool!("1 <= 2", true);
    test_as_bool!("3 <= 2", false);
    test_as_bool!("2 <= 3.1", true);
    test_as_bool!("3.1 <= 3.1", true);
    test_as_bool!("3 <= 3.0", true);
}

#[test]
fn test_math() {
    test_as_bool!("1 - 1", false);
    test_as_bool!("2 - 1", true);
}

