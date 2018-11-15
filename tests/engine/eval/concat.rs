use serde_json::json;

use crate::test_eval_eq;

#[test]
fn multiple_strings() {
    test_eval_eq!("`a` ~ `b`", json!("ab"));
    test_eval_eq!("`a` ~ `b` ~ `c`", json!("abc"));
}

#[test]
fn string_with_integer() {
    test_eval_eq!("`a` ~ 10", json!("a10"));
}

#[test]
fn string_with_float() {
    test_eval_eq!("`a` ~ 9.9", json!("a9.9"));
}
