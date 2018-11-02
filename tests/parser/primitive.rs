use balena_template::{error::Error, parser::ast::*};

macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), Expression::new($r));
    }};
}

macro_rules! test_parse_err {
    ($e:expr) => {{
        assert!(($e.parse() as Result<Expression, Error>).is_err());
    }};
}

#[test]
fn test_boolean() {
    test_parse_eq!("true", ExpressionValue::Boolean(true));
    test_parse_eq!("false", ExpressionValue::Boolean(false));
}

#[test]
fn test_integer() {
    test_parse_eq!("0", ExpressionValue::Integer(0));
    test_parse_eq!("123", ExpressionValue::Integer(123));
    test_parse_eq!("-346", ExpressionValue::Integer(-346));
}

#[test]
fn test_integer_leading_zeroes() {
    test_parse_eq!("00000", ExpressionValue::Integer(0));
    test_parse_eq!("00001", ExpressionValue::Integer(1));
    test_parse_eq!("-00001234", ExpressionValue::Integer(-1234));
}

#[test]
fn test_integer_boundaries() {
    test_parse_eq!(format!("{}", std::i64::MIN), ExpressionValue::Integer(std::i64::MIN));
    test_parse_eq!(format!("{}", std::i64::MAX), ExpressionValue::Integer(std::i64::MAX));

    test_parse_err!(format!("{}9999", std::i64::MIN));
    test_parse_err!(format!("{}9999", std::i64::MAX));
}

#[test]
fn test_float() {
    test_parse_eq!("0.0", ExpressionValue::Float(0.0));
    test_parse_eq!("-1.3", ExpressionValue::Float(-1.3));
    test_parse_eq!("2.9", ExpressionValue::Float(2.9));
    test_parse_err!(".0");
}

#[test]
fn test_float_must_start_with_digit() {
    test_parse_err!(".0");
}

#[test]
fn test_float_leading_and_trailing_zeroes() {
    test_parse_eq!("00000.0000", ExpressionValue::Float(0.0));
    test_parse_eq!("-00001.30000", ExpressionValue::Float(-1.3));
    test_parse_eq!("0002.9000", ExpressionValue::Float(2.9));
}

#[test]
fn test_float_boundaries() {
    // We have to format with at least one decimal digit otherwise it's parsed as int
    test_parse_eq!(format!("{:.1}", std::f64::MIN), ExpressionValue::Float(std::f64::MIN));
    test_parse_eq!(format!("{:.1}", std::f64::MAX), ExpressionValue::Float(std::f64::MAX));

    // Infinite numbers are not supported
    test_parse_err!(format!("{:.0}999.99", std::f64::MAX));
    test_parse_err!(format!("{:.0}999.99", std::f64::MIN));
}

#[test]
fn test_string() {
    test_parse_eq!("\"hallo\"", ExpressionValue::String("hallo".to_string()));
    test_parse_eq!("\"ha'l'lo\"", ExpressionValue::String("ha'l'lo".to_string()));
    test_parse_eq!("\"ha`l`lo\"", ExpressionValue::String("ha`l`lo".to_string()));
    test_parse_eq!("'hallo'", ExpressionValue::String("hallo".to_string()));
    test_parse_eq!("'ha\"l\"lo'", ExpressionValue::String("ha\"l\"lo".to_string()));
    test_parse_eq!("'ha`l`lo'", ExpressionValue::String("ha`l`lo".to_string()));
    test_parse_eq!("`hallo`", ExpressionValue::String("hallo".to_string()));
    test_parse_eq!("`ha'l'lo`", ExpressionValue::String("ha'l'lo".to_string()));
    test_parse_eq!("`ha\"l\"lo`", ExpressionValue::String("ha\"l\"lo".to_string()));
}

#[test]
fn test_unclosed_string() {
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
