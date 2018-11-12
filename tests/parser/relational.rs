use balena_temen::ast::*;
use testutils::{test_parse_eq, test_parse_err};

#[test]
fn operator() {
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

#[test]
fn fail_on_invalid_syntax() {
    test_parse_err!("==1");
    test_parse_err!("!=1");
    test_parse_err!(">=1");
    test_parse_err!(">1");
    test_parse_err!("<1");
    test_parse_err!("<=1");
    test_parse_err!("1==");
    test_parse_err!("1!=");
    test_parse_err!("1>=");
    test_parse_err!("1>");
    test_parse_err!("1<");
    test_parse_err!("1<=");
}
