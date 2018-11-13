use crate::{test_eval_as_bool_eq, test_eval_as_bool_err};

#[test]
fn boolean() {
    test_eval_as_bool_eq!("true", true);
    test_eval_as_bool_eq!("false", false);
}

#[test]
fn fail_on_string() {
    test_eval_as_bool_err!("\"\"");
    test_eval_as_bool_err!("\"hallo\"");
}

#[test]
fn fail_on_integer() {
    test_eval_as_bool_err!("10");
    test_eval_as_bool_err!("-12");
    test_eval_as_bool_err!("0");
}

#[test]
fn fail_on_float() {
    test_eval_as_bool_err!("10.2");
    test_eval_as_bool_err!("-3.2");
    test_eval_as_bool_err!("0.0");
}
