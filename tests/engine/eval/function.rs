use balena_temen::engine::context::Context;
use balena_temen::engine::{Engine, EngineBuilder};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::{Uuid, Version};

#[test]
fn test_uuidv4() {
    let engine = Engine::default();
    let context = Context::default();

    let uuid_value = engine.eval(&"uuidv4()".parse().unwrap(), &context, None).unwrap();
    let uuid_str = uuid_value.as_str().unwrap();
    let uuid = Uuid::parse_str(&uuid_str).unwrap();
    // Version::Random == UUIDv4
    assert_eq!(uuid.get_version(), Some(Version::Random));
}

#[test]
fn test_now() {
    let engine = Engine::default();
    let context = Context::default();

    // now() has unit tests covering return value validity, so, it's enough to check result type here

    assert_eq!(
        engine.eval(&"now()".parse().unwrap(), &context, None).unwrap(),
        engine.eval(&"now()".parse().unwrap(), &context, None).unwrap()
    );
    assert_eq!(
        engine
            .eval(
                &"now() == now(timestamp=false, utc=false)".parse().unwrap(),
                &context,
                None
            )
            .unwrap(),
        Value::Bool(true)
    );

    assert!(engine
        .eval(&"now()".parse().unwrap(), &context, None)
        .unwrap()
        .is_string());
    assert!(engine
        .eval(&"now(utc=true)".parse().unwrap(), &context, None)
        .unwrap()
        .is_string());
    assert!(engine
        .eval(&"now(timestamp=true)".parse().unwrap(), &context, None)
        .unwrap()
        .is_number());
    assert!(engine
        .eval(&"now(timestamp=true, utc=true)".parse().unwrap(), &context, None)
        .unwrap()
        .is_number());
}

#[test]
fn test_custom_function() {
    let cf = |args: &HashMap<String, Value>, _: &Context| {
        if let Some(name) = args.get("name") {
            Ok(name.clone())
        } else {
            Ok(Value::String("no-name-passed".to_string()))
        }
    };

    let engine: Engine = EngineBuilder::default().function("echo", cf).into();
    let ctx = Context::default();

    assert_eq!(
        engine.eval(&"echo()".parse().unwrap(), &ctx, None).unwrap(),
        json!("no-name-passed")
    );
    assert_eq!(
        engine.eval(&"echo(name=`Zrzka`)".parse().unwrap(), &ctx, None).unwrap(),
        json!("Zrzka")
    );
}
