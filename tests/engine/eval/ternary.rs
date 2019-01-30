use serde_json::json;

use crate::test_eval_eq;

#[test]
fn bool_value_as_condition() {
    test_eval_eq!("(true ? `yes` : `no`)", json!("yes"));
    test_eval_eq!("(false ? `yes` : `no`)", json!("no"));
}

#[test]
fn fn_in_condition() {
    test_eval_eq!("(POW(10, 2) == 100 ? `yes` : `no`)", json!("yes"));
}

#[test]
fn string_concat_in_condition() {
    test_eval_eq!("(`a` ~ `b` == `ab` ? `yes` : `no`)", json!("yes"));
}

#[test]
fn nested() {
    test_eval_eq!(
        "(true ? (true ? (true ? (true ? `yes` : `no`) : `no`) : `no`) : `no`)",
        json!("yes")
    );
}

#[test]
fn math_expression_as_truthy() {
    test_eval_eq!("(true ? 3 + 5 : 0)", json!(8));
}

#[test]
fn math_expression_as_falsy() {
    test_eval_eq!("(not true ? 0 : 3 + 5)", json!(8));
}
