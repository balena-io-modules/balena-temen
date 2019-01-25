use serde_json::json;

use balena_temen::{Context, Engine, EngineBuilder, evaluate, evaluate_with_engine};

#[test]
fn primitive_types_pass_through() {
    assert_eq!(evaluate(json!(true)).unwrap(), json!(true));
    assert_eq!(evaluate(json!(10)).unwrap(), json!(10));
    assert_eq!(evaluate(json!(10.5)).unwrap(), json!(10.5));
    assert_eq!(evaluate(json!("hallo")).unwrap(), json!("hallo"));
}

#[test]
fn root_object() {
    assert_eq!(evaluate(json!({"$$formula": "1 + 2"})).unwrap(), json!(3));
}

#[test]
fn nested_object() {
    assert_eq!(
        evaluate(json!({"nested": {"$$formula": "1 + 2"}})).unwrap(),
        json!({"nested": 3})
    );
}

#[test]
fn array() {
    let data = json!([
        "a",
        "b",
        {
            "$$formula": "`C` | LOWER"
        },
        "d"
    ]);

    assert_eq!(evaluate(data).unwrap(), json!(["a", "b", "c", "d"]));
}

#[test]
fn chained_references() {
    let data = json!({
        "first": "a",
        "second": {
            "$$formula": "first ~ `a`"
        },
        "third": {
            "$$formula": "second ~ `a`"
        },
        "fourth": {
            "$$formula": "third ~ `a`"
        },
    });
    let evaluated = json!({
        "first": "a",
        "second": "aa",
        "third": "aaa",
        "fourth": "aaaa"
    });

    assert_eq!(evaluate(data).unwrap(), evaluated);
}

#[test]
fn fail_on_circular_dependencies() {
    let data = json!({
        "first": {
            "$$formula": "second"
        },
        "second": {
            "$$formula": "first"
        }
    });

    assert!(evaluate(data).is_err());
}

#[test]
fn custom_keyword() {
    let engine: Engine = EngineBuilder::default().eval_keyword("evalMePlease").into();
    let mut context = Context::default();

    assert_eq!(
        evaluate_with_engine(json!({"evalMePlease": "1 + 2"}), &engine, &mut context).unwrap(),
        json!(3)
    );
}
