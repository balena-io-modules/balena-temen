use balena_temen::ast::*;
use crate::{test_parse_eq, test_parse_err};

#[test]
fn min_length() {
    test_parse_eq!(
        "a",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("a".to_string())
        ])))
    );
}

#[test]
fn max_length() {
    test_parse_eq!(
        "abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcd",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcd".to_string())
        ])))
    );
}

#[test]
fn fail_on_exceeded_max_length() {
    test_parse_err!("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcde");
}

#[test]
fn allowed_characters() {
    test_parse_eq!(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890_",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890_".to_string())
        ])))
    );
}

#[test]
fn fail_on_number_prefix() {
    for i in 0..10 {
        test_parse_err!(format!("{}abc", i));
    }
}

#[test]
fn simple() {
    test_parse_eq!(
        "networks",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("networks".to_string())
        ])))
    );
}

#[test]
fn dotted() {
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
fn integer_index() {
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
fn string_index() {
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
fn indirect_index() {
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
fn fail_on_float_index() {
    test_parse_err!("networks[2.3].wifi");
}

#[test]
fn fail_on_bool_index() {
    test_parse_err!("networks[true].wifi");
}

#[test]
fn this_reserved_keyword() {
    test_parse_eq!(
        "this",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::This
        ])))
    );
    test_parse_eq!(
        "this.ssid",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::This,
            IdentifierValue::Name("ssid".to_string())
        ])))
    );
}

#[test]
fn super_reserved_keyword() {
    test_parse_eq!(
        "super.ssid",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Super,
            IdentifierValue::Name("ssid".to_string())
        ])))
    );
    test_parse_eq!(
        "networks[0].super",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Name("networks".to_string()),
            IdentifierValue::IntegerIndex(0),
            IdentifierValue::Super
        ])))
    );
    test_parse_eq!(
        "super.super.super",
        Expression::new(ExpressionValue::Identifier(Identifier::new(vec![
            IdentifierValue::Super,
            IdentifierValue::Super,
            IdentifierValue::Super
        ])))
    );
}
