use balena_temen::ast::*;

use crate::{test_parse_eq, test_parse_err};

macro_rules! exp {
    ($e:expr) => {
        balena_temen::ast::Expression::new($e)
    };
}

#[test]
fn boolean() {
    test_parse_eq!("true", exp!(ExpressionValue::Boolean(true)));
    test_parse_eq!("false", exp!(ExpressionValue::Boolean(false)));
}

#[test]
fn integer() {
    test_parse_eq!("0", exp!(ExpressionValue::Integer(0)));
    test_parse_eq!("123", exp!(ExpressionValue::Integer(123)));
    test_parse_eq!("-346", exp!(ExpressionValue::Integer(-346)));
}

#[test]
fn integer_with_leading_zeroes() {
    test_parse_eq!("00000", exp!(ExpressionValue::Integer(0)));
    test_parse_eq!("00001", exp!(ExpressionValue::Integer(1)));
    test_parse_eq!("-00001234", exp!(ExpressionValue::Integer(-1234)));
}

#[test]
fn integer_boundaries() {
    test_parse_eq!(
        &format!("{}", std::i64::MIN),
        exp!(ExpressionValue::Integer(std::i64::MIN))
    );
    test_parse_eq!(
        &format!("{}", std::i64::MAX),
        exp!(ExpressionValue::Integer(std::i64::MAX))
    );
}

#[test]
fn fail_on_integer_out_of_bounds() {
    test_parse_err!(&format!("{}9999", std::i64::MIN));
    test_parse_err!(&format!("{}9999", std::i64::MAX));
}

#[test]
fn float() {
    test_parse_eq!("0.0", exp!(ExpressionValue::Float(0.0)));
    test_parse_eq!("-1.3", exp!(ExpressionValue::Float(-1.3)));
    test_parse_eq!("2.9", exp!(ExpressionValue::Float(2.9)));
}

#[test]
fn fail_on_float_without_leading_digit() {
    test_parse_err!(".0");
}

#[test]
fn float_leading_and_trailing_zeroes() {
    test_parse_eq!("00000.0000", exp!(ExpressionValue::Float(0.0)));
    test_parse_eq!("-00001.30000", exp!(ExpressionValue::Float(-1.3)));
    test_parse_eq!("0002.9000", exp!(ExpressionValue::Float(2.9)));
}

#[test]
fn float_boundaries() {
    // We have to format with at least one decimal digit otherwise it's parsed as int
    test_parse_eq!(
        format!("{:.1}", std::f64::MIN),
        exp!(ExpressionValue::Float(std::f64::MIN))
    );
    test_parse_eq!(
        format!("{:.1}", std::f64::MAX),
        exp!(ExpressionValue::Float(std::f64::MAX))
    );
}

#[test]
fn fail_on_float_infinity() {
    // Infinite numbers are not supported
    test_parse_err!(&format!("{:.0}999.99", std::f64::MAX));
    test_parse_err!(&format!("{:.0}999.99", std::f64::MIN));
}

#[test]
fn string() {
    test_parse_eq!("\"hallo\"", exp!(ExpressionValue::String("hallo".to_string())));
    test_parse_eq!("\"ha'l'lo\"", exp!(ExpressionValue::String("ha'l'lo".to_string())));
    test_parse_eq!("\"ha`l`lo\"", exp!(ExpressionValue::String("ha`l`lo".to_string())));
    test_parse_eq!("'hallo'", exp!(ExpressionValue::String("hallo".to_string())));
    test_parse_eq!("'ha\"l\"lo'", exp!(ExpressionValue::String("ha\"l\"lo".to_string())));
    test_parse_eq!("'ha`l`lo'", exp!(ExpressionValue::String("ha`l`lo".to_string())));
    test_parse_eq!("`hallo`", exp!(ExpressionValue::String("hallo".to_string())));
    test_parse_eq!("`ha'l'lo`", exp!(ExpressionValue::String("ha'l'lo".to_string())));
    test_parse_eq!("`ha\"l\"lo`", exp!(ExpressionValue::String("ha\"l\"lo".to_string())));
}

#[test]
fn fail_on_invalid_string_syntax() {
    test_parse_err!("\"hallo");
    test_parse_err!("\"ha'l'lo");
    test_parse_err!("\"ha`l`lo");
    test_parse_err!("'hallo");
    test_parse_err!("'ha\"l\"lo");
    test_parse_err!("'ha`l`lo");
    test_parse_err!("`hallo");
    test_parse_err!("`ha'l'lo");
    test_parse_err!("`ha\"l\"lo");
}
