use balena_temen::{error::Error, parser::ast::*};

macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), $r);
    }};
}

macro_rules! test_parse_err {
    ($e:expr) => {{
        assert!(($e.parse() as Result<Expression, Error>).is_err());
    }};
}

#[test]
fn operator() {
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
fn fail_on_invalid_syntax() {
    test_parse_err!("1+");
    test_parse_err!("1*");
    test_parse_err!("1/");
    test_parse_err!("1-");
    test_parse_err!("1%");
    test_parse_err!("+1");
    test_parse_err!("*2");
    test_parse_err!("/3");
    test_parse_err!("%1");
}
