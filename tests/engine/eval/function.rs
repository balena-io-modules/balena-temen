use serde_json::json;

use balena_temen::{Context, Engine, EngineBuilder, Value};

use crate::{test_eval_eq, test_eval_err, test_eval_ok};

#[test]
fn default_functions_are_registered() {
    // All functions have unit tests and it's enough to test if they're called / registered / work
    test_eval_ok!("UUIDV4()");
    test_eval_ok!("NOW()");
}

#[test]
fn fail_on_unknown_function() {
    test_eval_err!("FNDOESNOTEXISTORATLEASTITSHOULDNOT()");
}

#[test]
fn custom_function() {
    let cf = |args: &[Value], _: &mut Context| {
        if let Some(name) = args.first() {
            Ok(name.clone())
        } else {
            Ok(Value::String("no-value-passed".to_string()))
        }
    };

    let engine: Engine = EngineBuilder::default().function("ECHO", cf).into();

    test_eval_eq!(engine, "ECHO()", json!("no-value-passed"));
    test_eval_eq!(engine, "ECHO(`Zrzka`)", json!("Zrzka"));
}
