use balena_template::{error::Error, parser::ast::*};
use std::collections::HashMap;

macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), $r);
    }};
}

#[test]
fn test_boolean() {
    test_parse_eq!("true", Expression::new(ExpressionValue::Boolean(true)));
    test_parse_eq!("false", Expression::new(ExpressionValue::Boolean(false)));
}

#[test]
fn test_integer() {
    test_parse_eq!("0", Expression::new(ExpressionValue::Integer(0)));
    test_parse_eq!("00000", Expression::new(ExpressionValue::Integer(0)));
    test_parse_eq!("00001", Expression::new(ExpressionValue::Integer(1)));
    test_parse_eq!("-1234", Expression::new(ExpressionValue::Integer(-1234)));
    test_parse_eq!("-00001234", Expression::new(ExpressionValue::Integer(-1234)));
}

#[test]
fn test_float() {
    test_parse_eq!("0.0", Expression::new(ExpressionValue::Float(0.0)));
    test_parse_eq!("0000.0000", Expression::new(ExpressionValue::Float(0.0)));
    test_parse_eq!("-1.3", Expression::new(ExpressionValue::Float(-1.3)));
    test_parse_eq!("-01.30", Expression::new(ExpressionValue::Float(-1.3)));
    test_parse_eq!("2.9", Expression::new(ExpressionValue::Float(2.9)));
    test_parse_eq!("002.900", Expression::new(ExpressionValue::Float(2.9)));
}

#[test]
fn test_string() {
    test_parse_eq!(
        "\"hallo\"",
        Expression::new(ExpressionValue::String("hallo".to_string()))
    );
    test_parse_eq!(
        "\"ha'l'lo\"",
        Expression::new(ExpressionValue::String("ha'l'lo".to_string()))
    );
    test_parse_eq!(
        "\"ha`l`lo\"",
        Expression::new(ExpressionValue::String("ha`l`lo".to_string()))
    );
    test_parse_eq!("'hallo'", Expression::new(ExpressionValue::String("hallo".to_string())));
    test_parse_eq!(
        "'ha\"l\"lo'",
        Expression::new(ExpressionValue::String("ha\"l\"lo".to_string()))
    );
    test_parse_eq!(
        "'ha`l`lo'",
        Expression::new(ExpressionValue::String("ha`l`lo".to_string()))
    );
    test_parse_eq!("`hallo`", Expression::new(ExpressionValue::String("hallo".to_string())));
    test_parse_eq!(
        "`ha'l'lo`",
        Expression::new(ExpressionValue::String("ha'l'lo".to_string()))
    );
    test_parse_eq!(
        "`ha\"l\"lo`",
        Expression::new(ExpressionValue::String("ha\"l\"lo".to_string()))
    );
}

#[test]
fn test_math_operator() {
    let exp = |lhs: i64, rhs: i64, operator: MathOperator| {
        let lhs_exp = Expression::new(ExpressionValue::Integer(lhs));
        let rhs_exp = Expression::new(ExpressionValue::Integer(rhs));
        Expression::new(ExpressionValue::Math(MathExpression::new(lhs_exp, rhs_exp, operator)))
    };

    test_parse_eq!("1 + 1", exp(1, 1, MathOperator::Addition));
    test_parse_eq!("2 - (-3)", exp(2, -3, MathOperator::Subtraction));
    test_parse_eq!("4 * 5", exp(4, 5, MathOperator::Multiplication));
    test_parse_eq!("4 / 2", exp(4, 2, MathOperator::Division));
    test_parse_eq!("4 % 2", exp(4, 2, MathOperator::Modulo));
}

#[test]
fn test_logical_operator() {
    let exp = |lhs: bool, rhs: bool, operator: LogicalOperator| {
        let lhs_exp = Expression::new(ExpressionValue::Boolean(lhs));
        let rhs_exp = Expression::new(ExpressionValue::Boolean(rhs));
        Expression::new(ExpressionValue::Logical(LogicalExpression::new(
            lhs_exp, rhs_exp, operator,
        )))
    };

    test_parse_eq!("true or false", exp(true, false, LogicalOperator::Or));
    test_parse_eq!("true and false", exp(true, false, LogicalOperator::And));
    test_parse_eq!("not true", Expression::new_negated(ExpressionValue::Boolean(true)));
}

#[test]
fn test_relational_operator() {
    let exp = |lhs: i64, rhs: i64, operator: LogicalOperator| {
        let lhs_exp = Expression::new(ExpressionValue::Integer(lhs));
        let rhs_exp = Expression::new(ExpressionValue::Integer(rhs));
        Expression::new(ExpressionValue::Logical(LogicalExpression::new(
            lhs_exp, rhs_exp, operator,
        )))
    };

    test_parse_eq!("1 == 2", exp(1, 2, LogicalOperator::Equal));
    test_parse_eq!("3 != 2", exp(3, 2, LogicalOperator::NotEqual));
    test_parse_eq!("3 > 2", exp(3, 2, LogicalOperator::GreaterThan));
    test_parse_eq!("3 >= 2", exp(3, 2, LogicalOperator::GreaterThanOrEqual));
    test_parse_eq!("3 < 2", exp(3, 2, LogicalOperator::LowerThan));
    test_parse_eq!("3 <= 2", exp(3, 2, LogicalOperator::LowerThanOrEqual));
}

macro_rules! fn_args_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), Expression::new($value));
            )+
            m
        }
     };
);

#[test]
fn test_function() {
    let args = fn_args_map! {
        "v" => ExpressionValue::Integer(4),
        "dummy" => ExpressionValue::String("abc".to_string())
    };

    test_parse_eq!(
        "uuid()",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", HashMap::new())))
    );
    test_parse_eq!(
        "uuid(v=4, dummy=`abc`)",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", args)))
    );
}

#[test]
fn test_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![
            FunctionCall::new("slugify".to_string(), HashMap::default()),
            FunctionCall::new("rustify".to_string(), HashMap::default()),
        ],
    );
    test_parse_eq!("'Abc' | slugify | rustify", exp);
}
