use std::collections::HashMap;

use serde_json::json;
use crate::{test_eval_eq, test_eval_err};

use balena_temen::{Context, Engine, EngineBuilder, Value};

#[test]
fn default_filters_are_registered() {
    // All filters have unit tests and it's enough to test if they're called / registered / work
    test_eval_eq!("1541485381 | time", json!("06:23:01"));
    test_eval_eq!("1541485381 | date", json!("2018-11-06"));
    test_eval_eq!("1541485381 | datetime", json!("2018-11-06T06:23:01+00:00"));
    test_eval_eq!("`A` | lower", json!("a"));
    test_eval_eq!("`A` | slugify", json!("a"));
    test_eval_eq!("`A` | trim", json!("A"));
    test_eval_eq!("`a` | upper", json!("A"));
}

#[test]
fn filter_chain() {
    test_eval_eq!("`a` | lower | upper", json!("A"));
    test_eval_eq!("`A` | lower | upper", json!("A"));
}

#[test]
fn fail_on_unknown_filter() {
    test_eval_err!("1 | filterdoesnotexistoratleastitshouldnot");
}

#[test]
fn custom_filter() {
    let cf = |input: &Value, _: &HashMap<String, Value>, _: &mut Context| {
        if input.is_string() {
            Ok(Value::String(input.as_str().unwrap().replace("a", "b")))
        } else {
            Err("no string, no fun".into())
        }
    };

    let engine: Engine = EngineBuilder::default().filter("atob", cf).into();

    test_eval_eq!(engine, "`abc` | atob", json!("bbc"));
    test_eval_err!(engine, "true | atob");
}
