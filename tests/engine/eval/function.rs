use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::{Uuid, Version};

#[test]
fn test_uuidv4() {
    let engine = Engine::default();
    let context = Context::default();

    let uuid_value = engine.eval(&"uuidv4()".parse().unwrap(), &context).unwrap();
    let uuid_str = uuid_value.as_str().unwrap();
    let uuid = Uuid::parse_str(&uuid_str).unwrap();
    // Version::Random == UUIDv4
    assert_eq!(uuid.get_version(), Some(Version::Random));
}

#[test]
fn test_custom_function() {
    let cf = |args: &HashMap<String, Value>| {
        if let Some(name) = args.get("name") {
            Ok(name.clone())
        } else {
            Ok(Value::String("no-name-passed".to_string()))
        }
    };

    let mut engine = Engine::default();
    engine.register_function("echo", cf);
    let ctx = Context::default();

    assert_eq!(
        engine.eval(&"echo()".parse().unwrap(), &ctx).unwrap(),
        json!("no-name-passed")
    );
    assert_eq!(
        engine.eval(&"echo(name=`Zrzka`)".parse().unwrap(), &ctx).unwrap(),
        json!("Zrzka")
    );
}
