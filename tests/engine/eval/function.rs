use std::collections::HashMap;

use serde_json::json;

use balena_temen::{Context, Engine, EngineBuilder, Value};

#[test]
fn default_functions_are_registered() {
    // All functions have unit tests and it's enough to test if they're called / registered / work
    let engine = Engine::default();
    let ctx = Context::default();

    assert!(engine.eval(&"uuidv4()".parse().unwrap(), &ctx, None).is_ok());
    assert!(engine.eval(&"now()".parse().unwrap(), &ctx, None).is_ok());
}

#[test]
fn fail_on_unknown_function() {
    let engine = Engine::default();
    let context = Context::default();

    assert!(engine
        .eval(&"fndoesnotexistoratleastitshouldnot()".parse().unwrap(), &context, None)
        .is_err());
}

#[test]
fn custom_function() {
    let cf = |args: &HashMap<String, Value>, _: &Context| {
        if let Some(name) = args.get("value") {
            Ok(name.clone())
        } else {
            Ok(Value::String("no-value-passed".to_string()))
        }
    };

    let engine: Engine = EngineBuilder::default().function("echo", cf).into();
    let ctx = Context::default();

    assert_eq!(
        engine.eval(&"echo()".parse().unwrap(), &ctx, None).unwrap(),
        json!("no-value-passed")
    );
    assert_eq!(
        engine
            .eval(&"echo(value=`Zrzka`)".parse().unwrap(), &ctx, None)
            .unwrap(),
        json!("Zrzka")
    );
}
