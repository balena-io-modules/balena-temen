use crate::{test_eval_as_bool_eq, test_eval_as_bool_err};

#[test]
fn logical_and() {
    test_eval_as_bool_eq!("true and true", true);
    test_eval_as_bool_eq!("true and false", false);
    test_eval_as_bool_eq!("false and true", false);
    test_eval_as_bool_eq!("false and false", false);
}

#[test]
fn logical_or() {
    test_eval_as_bool_eq!("true or true", true);
    test_eval_as_bool_eq!("true or false", true);
    test_eval_as_bool_eq!("false or true", true);
    test_eval_as_bool_eq!("false or false", false);
}

#[test]
fn logical_not() {
    test_eval_as_bool_eq!("not false", true);
    test_eval_as_bool_eq!("not 1 == 2", true);
}

#[test]
fn fail_on_invalid_syntax() {
    test_eval_as_bool_err!("true and");
    test_eval_as_bool_err!("and true");
    test_eval_as_bool_err!("true or");
    test_eval_as_bool_err!("or true");
    test_eval_as_bool_err!("or");
    test_eval_as_bool_err!("and");
    test_eval_as_bool_err!("not");
}

#[test]
fn fail_on_invalid_type() {
    test_eval_as_bool_err!("true and 1");
    test_eval_as_bool_err!("true and 1.2");
    test_eval_as_bool_err!("true and `abc`");
    test_eval_as_bool_err!("true and null");
    test_eval_as_bool_err!("true or 1");
    test_eval_as_bool_err!("true or 1.2");
    test_eval_as_bool_err!("true or `abc`");
    test_eval_as_bool_err!("true or null");
}
