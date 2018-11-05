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
fn test_identifier_min_length() {
    test_parse_eq!(
        "a",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("a".to_string())
        ])))
    );
}

#[test]
fn test_identifier_max_length() {
    test_parse_eq!(
        "abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcd",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcd".to_string())
        ])))
    );
    test_parse_err!("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcde");
}

#[test]
fn test_identifier_allowed_characters() {
    test_parse_eq!(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890_",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890_".to_string())
        ])))
    );
}

#[test]
fn test_identifier_must_not_start_with_number() {
    for i in 0..10 {
        test_parse_err!(format!("{}abc", i));
    }
}

#[test]
fn test_simple_identifier() {
    test_parse_eq!(
        "networks",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("networks".to_string())
        ])))
    );
}

#[test]
fn test_dotted_identifier() {
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
fn test_identifier_float_index() {
    test_parse_err!("networks[2.3].wifi");
}

#[test]
fn test_identifier_bool_index() {
    test_parse_err!("networks[true].wifi");
}
