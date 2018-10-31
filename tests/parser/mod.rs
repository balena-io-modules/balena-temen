use balena_template::{error::Error, parser::ast::*};
use std::collections::HashMap;

macro_rules! test_parser {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), $r);
    }};
}

#[test]
fn test_boolean() {
    test_parser!("true", Expression::new(ExpressionValue::Boolean(true)));
    test_parser!("false", Expression::new(ExpressionValue::Boolean(false)));
}

#[test]
fn test_integer() {
    test_parser!("0", Expression::new(ExpressionValue::Integer(0)));
    test_parser!("00000", Expression::new(ExpressionValue::Integer(0)));
    test_parser!("00001", Expression::new(ExpressionValue::Integer(1)));
    test_parser!("-1234", Expression::new(ExpressionValue::Integer(-1234)));
    test_parser!("-00001234", Expression::new(ExpressionValue::Integer(-1234)));
}

#[test]
fn test_float() {
    test_parser!("0.0", Expression::new(ExpressionValue::Float(0.0)));
    test_parser!("0000.0000", Expression::new(ExpressionValue::Float(0.0)));
    test_parser!("-1.3", Expression::new(ExpressionValue::Float(-1.3)));
    test_parser!("-01.30", Expression::new(ExpressionValue::Float(-1.3)));
    test_parser!("2.9", Expression::new(ExpressionValue::Float(2.9)));
    test_parser!("002.900", Expression::new(ExpressionValue::Float(2.9)));
}

#[test]
fn test_string() {
    test_parser!(
        "\"hallo\"",
        Expression::new(ExpressionValue::String("hallo".to_string()))
    );
    test_parser!(
        "\"ha'l'lo\"",
        Expression::new(ExpressionValue::String("ha'l'lo".to_string()))
    );
    test_parser!(
        "\"ha`l`lo\"",
        Expression::new(ExpressionValue::String("ha`l`lo".to_string()))
    );
    test_parser!("'hallo'", Expression::new(ExpressionValue::String("hallo".to_string())));
    test_parser!(
        "'ha\"l\"lo'",
        Expression::new(ExpressionValue::String("ha\"l\"lo".to_string()))
    );
    test_parser!(
        "'ha`l`lo'",
        Expression::new(ExpressionValue::String("ha`l`lo".to_string()))
    );
    test_parser!("`hallo`", Expression::new(ExpressionValue::String("hallo".to_string())));
    test_parser!(
        "`ha'l'lo`",
        Expression::new(ExpressionValue::String("ha'l'lo".to_string()))
    );
    test_parser!(
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

    test_parser!("1 + 1", exp(1, 1, MathOperator::Addition));
    test_parser!("2 - (-3)", exp(2, -3, MathOperator::Subtraction));
    test_parser!("4 * 5", exp(4, 5, MathOperator::Multiplication));
    test_parser!("4 / 2", exp(4, 2, MathOperator::Division));
    test_parser!("4 % 2", exp(4, 2, MathOperator::Modulo));
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

    test_parser!("true or false", exp(true, false, LogicalOperator::Or));
    test_parser!("true and false", exp(true, false, LogicalOperator::And));
    test_parser!("not true", Expression::new_negated(ExpressionValue::Boolean(true)));
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

    test_parser!("1 == 2", exp(1, 2, LogicalOperator::Equal));
    test_parser!("3 != 2", exp(3, 2, LogicalOperator::NotEqual));
    test_parser!("3 > 2", exp(3, 2, LogicalOperator::GreaterThan));
    test_parser!("3 >= 2", exp(3, 2, LogicalOperator::GreaterThanOrEqual));
    test_parser!("3 < 2", exp(3, 2, LogicalOperator::LowerThan));
    test_parser!("3 <= 2", exp(3, 2, LogicalOperator::LowerThanOrEqual));
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

    test_parser!(
        "uuid()",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", HashMap::new())))
    );
    test_parser!(
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
    test_parser!("'Abc' | slugify | rustify", exp);
}
