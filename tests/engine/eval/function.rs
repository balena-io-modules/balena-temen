use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;
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
