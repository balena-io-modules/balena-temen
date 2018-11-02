use balena_template::engine::context::Context;
use balena_template::engine::Engine;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval_as_bool(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        if let Ok(exp) = $e.parse() {
            assert!(engine.eval_as_bool(&exp, &context).is_err());
        }
    }};
}

#[test]
fn test_logical_and() {
    test_eval_eq!("true and true", true);
    test_eval_eq!("true and false", false);
    test_eval_eq!("false and true", false);
    test_eval_eq!("false and false", false);
}

#[test]
fn test_logical_or() {
    test_eval_eq!("true or true", true);
    test_eval_eq!("true or false", true);
    test_eval_eq!("false or true", true);
    test_eval_eq!("false or false", false);
}

#[test]
fn test_logical_not() {
    test_eval_eq!("not false", true);
    test_eval_eq!("not 1 == 2", true);
}

#[test]
fn test_invalid_syntax() {
    test_eval_err!("true and");
    test_eval_err!("and true");
    test_eval_err!("true or");
    test_eval_err!("or true");
    test_eval_err!("or");
    test_eval_err!("and");
    test_eval_err!("not");
}

#[test]
fn test_bool_with_other_types() {
    test_eval_err!("true and 1");
    test_eval_err!("true and 1.2");
    test_eval_err!("true and `abc`");
    test_eval_err!("true and null");
    test_eval_err!("true or 1");
    test_eval_err!("true or 1.2");
    test_eval_err!("true or `abc`");
    test_eval_err!("true or null");
}
