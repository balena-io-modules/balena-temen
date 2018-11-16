use balena_temen::ast::*;

use crate::{fn_args_map, test_parse_eq};

#[test]
fn without_arguments() {
    test_parse_eq!(
        "uuid()",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", fn_args_map!())))
    );
}

#[test]
fn with_arguments() {
    let args = fn_args_map! {
        "v" => ExpressionValue::Integer(4),
        "dummy" => ExpressionValue::String("abc".to_string())
    };

    test_parse_eq!(
        "uuid(v=4, dummy=`abc`)",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", args)))
    );
}
