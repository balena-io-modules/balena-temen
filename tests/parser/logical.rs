use balena_temen::ast::*;

use crate::{test_parse_eq, test_parse_err};

#[test]
fn operator() {
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
fn fail_on_invalid_syntax() {
    test_parse_err!("true and");
    test_parse_err!("and true");
    test_parse_err!("true or");
    test_parse_err!("or true");
    test_parse_err!("or");
    test_parse_err!("and");
    test_parse_err!("not");
}
