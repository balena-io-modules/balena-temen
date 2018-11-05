use balena_temen::{error::Error, parser::ast::*};

macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), $r);
    }};
}

#[test]
fn test_single_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![
            FunctionCall::new("slugify".to_string(), HashMap::default())
        ],
    );
    test_parse_eq!("'Abc' | slugify", exp);
}

#[test]
fn test_chained_filter() {
    let exp = Expression::new_with_filters(
        ExpressionValue::String("Abc".to_string()),
        vec![
            FunctionCall::new("slugify".to_string(), HashMap::default()),
            FunctionCall::new("rustify".to_string(), HashMap::default()),
        ],
    );
    test_parse_eq!("'Abc' | slugify | rustify", exp);
}
