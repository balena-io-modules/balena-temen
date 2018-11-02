use balena_template::{error::Error, parser::ast::*};
use std::collections::HashMap;

mod primitive;

macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), $r);
    }};
}

#[test]
fn test_simple_identifier() {
    test_parse_eq!(
        "networks",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("networks".to_string())
        ])))
    );
    test_parse_eq!(
        "advanced.logging",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("advanced".to_string()),
            IdentifierValue::Name("logging".to_string())
        ])))
    );
    test_parse_eq!(
        "advanced.logging.enabled",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("advanced".to_string()),
            IdentifierValue::Name("logging".to_string()),
            IdentifierValue::Name("enabled".to_string())
        ])))
    );
}

#[test]
fn test_identifier_integer_index() {
    test_parse_eq!(
        "networks[0].wifi",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("networks".to_string()),
            IdentifierValue::IntegerIndex(0),
            IdentifierValue::Name("wifi".to_string())
        ])))
    );
}

#[test]
fn test_identifier_string_index() {
    test_parse_eq!(
        "people[`123-456-789`].first",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("people".to_string()),
            IdentifierValue::StringIndex("123-456-789".to_string()),
            IdentifierValue::Name("first".to_string())
        ])))
    );

    test_parse_eq!(
        "people[`123-456-789`].first | lower",
        Expression::new_with_filters(
            ExpressionValue::Identifier(Identifier::new(vec![
                IdentifierValue::Name("people".to_string()),
                IdentifierValue::StringIndex("123-456-789".to_string()),
                IdentifierValue::Name("first".to_string())
            ])),
            vec![FunctionCall::new("lower", HashMap::new())]
        )
    );
}

#[test]
fn test_identifier_indirect_index() {
    test_parse_eq!(
        "people[people[0].id].first",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("people".to_string()),
            IdentifierValue::IdentifierIndex(Identifier::new(vec![
                IdentifierValue::Name("people".to_string()),
                IdentifierValue::IntegerIndex(0),
                IdentifierValue::Name("id".to_string()),
            ])),
            IdentifierValue::Name("first".to_string())
        ])))
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
