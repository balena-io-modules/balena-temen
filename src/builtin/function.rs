use crate::error::Result;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub type FunctionFn = Box<dyn Fn(&HashMap<String, Value>) -> Result<Value>>;

pub(crate) fn uuidv4(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(Value::String(Uuid::new_v4().to_hyphenated().to_string()))
}

#[cfg(test)]
mod tests {
    use super::{uuidv4, HashMap};
    use uuid::{Uuid, Version};

    #[test]
    fn test_uuidv4() {
        // Just a simple test that checks if UUIDv4 is generated
        let uuid_value = uuidv4(&HashMap::new()).unwrap();
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid = Uuid::parse_str(&uuid_str).unwrap();
        // Version::Random == UUIDv4
        assert_eq!(uuid.get_version(), Some(Version::Random));
    }
}
