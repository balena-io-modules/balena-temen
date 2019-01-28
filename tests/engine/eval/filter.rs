use serde_json::json;

use balena_temen::{error::*, Context, Engine, EngineBuilder, Value};

use crate::{test_eval_eq, test_eval_err};

#[test]
fn default_filters_are_registered() {
    // All filters have unit tests and it's enough to test if they're called / registered / work
    test_eval_eq!("1541485381 | TIME", json!("06:23:01"));
    test_eval_eq!("1541485381 | DATE", json!("2018-11-06"));
    test_eval_eq!("1541485381 | DATETIME", json!("2018-11-06T06:23:01+00:00"));
    test_eval_eq!("`A` | LOWER", json!("a"));
    test_eval_eq!("`A` | SLUGIFY", json!("a"));
    test_eval_eq!("`A` | TRIM", json!("A"));
    test_eval_eq!("`a` | UPPER", json!("A"));
}

#[test]
fn filter_chain() {
    test_eval_eq!("`a` | LOWER | UPPER", json!("A"));
    test_eval_eq!("`A` | LOWER | UPPER", json!("A"));
}

#[test]
fn fail_on_unknown_filter() {
    test_eval_err!("1 | FILTERDOESNOTEXISTORATLEASTITSHOULDNOT");
}

#[test]
fn custom_filter() {
    let cf = |input: &Value, _: &[Value], _: &mut Context| {
        if input.is_string() {
            Ok(Value::String(input.as_str().unwrap().replace("a", "b")))
        } else {
            Err(Error::with_message("no string, no fun"))
        }
    };

    let engine: Engine = EngineBuilder::default().filter("ATOB", cf).into();

    test_eval_eq!(engine, "`abc` | ATOB", json!("bbc"));
    test_eval_err!(engine, "true | ATOB");
}
