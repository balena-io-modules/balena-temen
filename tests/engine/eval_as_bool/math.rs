use crate::{test_eval_as_bool_eq, test_eval_as_bool_err};

#[test]
fn math() {
    test_eval_as_bool_eq!("1 - 1 == 2 - 2", true);
    test_eval_as_bool_eq!("3 * 9 == 27", true);
}

#[test]
fn fail_on_math_result() {
    // Math expression -> number - can't be evaluated as bool
    test_eval_as_bool_err!("1 - 1");
    test_eval_as_bool_err!("2 - 1");
}

#[test]
fn float_relative_eq() {
    test_eval_as_bool_eq!("2.1 - 2 == 0.1", true);
    test_eval_as_bool_eq!("2.1 - 0.1 == 2", true);
    test_eval_as_bool_eq!("0.1 == 0.1", true);
    test_eval_as_bool_eq!("322323.1 * 2 == 644646.2", true);
}
