use balena_temen::{error::Error, parser::ast::*};
use crate::test_parse_eq;

#[test]
fn single_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![
            FunctionCall::new("slugify".to_string(), HashMap::default())
        ],
    );
    test_parse_eq!("'Abc' | slugify", exp);
}

#[test]
fn chained_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![
            FunctionCall::new("slugify".to_string(), HashMap::default()),
            FunctionCall::new("rustify".to_string(), HashMap::default()),
        ],
    );
    test_parse_eq!("'Abc' | slugify | rustify", exp);
}
