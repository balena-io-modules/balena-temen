use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::engine::context::Context;
use crate::error::Result;

pub(crate) fn uuidv4(_args: &HashMap<String, Value>, _context: &Context) -> Result<Value> {
    Ok(Value::String(Uuid::new_v4().to_hyphenated().to_string()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::{Uuid, Version};

    use crate::engine::context::Context;

    use super::uuidv4;

    #[test]
    fn result_is_valid_uuid_v4() {
        let ctx = Context::default();
        let args = HashMap::new();

        let uuid_value = uuidv4(&args, &ctx).unwrap();
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid = Uuid::parse_str(&uuid_str).unwrap();
        // Version::Random == UUIDv4
        assert_eq!(uuid.get_version(), Some(Version::Random));
    }
}
