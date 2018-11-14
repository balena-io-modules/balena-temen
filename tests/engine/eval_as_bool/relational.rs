use crate::test_eval_as_bool_eq;

#[test]
fn equal() {
    test_eval_as_bool_eq!("true == true", true);
    test_eval_as_bool_eq!("1 == 1", true);
    test_eval_as_bool_eq!("2.3 == 2.3", true);
    test_eval_as_bool_eq!("`a` == `a`", true);
    test_eval_as_bool_eq!("`a` == `b`", false);
    test_eval_as_bool_eq!("`1` == 1", false);
}

#[test]
fn not_equal() {
    test_eval_as_bool_eq!("true != true", false);
    test_eval_as_bool_eq!("1 != 1", false);
    test_eval_as_bool_eq!("2.3 != 2.3", false);
    test_eval_as_bool_eq!("`a` != `a`", false);
    test_eval_as_bool_eq!("`a` != `b`", true);
    test_eval_as_bool_eq!("`1` != 1", true);
}

#[test]
fn greater_than() {
    test_eval_as_bool_eq!("1 > 2", false);
    test_eval_as_bool_eq!("3 > 2", true);
    test_eval_as_bool_eq!("3.1 > 2", true);
}

#[test]
fn greater_than_or_equal() {
    test_eval_as_bool_eq!("1 >= 2", false);
    test_eval_as_bool_eq!("3 >= 2", true);
    test_eval_as_bool_eq!("3.1 >= 2", true);
    test_eval_as_bool_eq!("3.1 >= 3.1", true);
    test_eval_as_bool_eq!("3 >= 3.0", true);
}

#[test]
fn lower_than() {
    test_eval_as_bool_eq!("1 < 2", true);
    test_eval_as_bool_eq!("3 < 2", false);
    test_eval_as_bool_eq!("3.1 < 2", false);
    test_eval_as_bool_eq!("3.1 < 3.1", false);
    test_eval_as_bool_eq!("3 < 3.0", false);
}

#[test]
fn lower_than_or_equal() {
    test_eval_as_bool_eq!("1 <= 2", true);
    test_eval_as_bool_eq!("3 <= 2", false);
    test_eval_as_bool_eq!("2 <= 3.1", true);
    test_eval_as_bool_eq!("3.1 <= 3.1", true);
    test_eval_as_bool_eq!("3 <= 3.0", true);
}
