use balena_temen::ast::*;

use crate::test_parse_eq;

#[test]
fn without_arguments() {
    test_parse_eq!(
        "UUID()",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("UUID", vec![])))
    );
}

#[test]
fn with_positional_arguments() {
    test_parse_eq!(
        "UUID(1)",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new(
            "UUID",
            vec![Expression::new(ExpressionValue::Integer(1))]
        )))
    );
}

#[test]
fn with_arguments() {
    let args = vec![
        Expression::new(ExpressionValue::Integer(4)),
        Expression::new(ExpressionValue::String("abc".to_string())),
    ];

    test_parse_eq!(
        "UUID(4, `abc`)",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("UUID", args)))
    );
}
