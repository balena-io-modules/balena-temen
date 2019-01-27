use balena_temen::ast::*;

use crate::test_parse_eq;

#[test]
fn single_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![FunctionCall::new("SLUGIFY".to_string(), vec![])],
    );
    test_parse_eq!("'Abc' | SLUGIFY", exp);
}

#[test]
fn chained_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![
            FunctionCall::new("SLUGIFY".to_string(), vec![]),
            FunctionCall::new("RUSTIFY".to_string(), vec![]),
        ],
    );
    test_parse_eq!("'Abc' | SLUGIFY | RUSTIFY", exp);
}
