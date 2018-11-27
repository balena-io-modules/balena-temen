use std::collections::HashMap;

use rand::{self, RngCore};
use serde_json::Value;
use uuid::Uuid;

use crate::context::Context;
use crate::error::Result;

pub(crate) fn uuidv4(_args: &HashMap<String, Value>, _context: &mut Context) -> Result<Value> {
    // We're using uuid crate with disabled v4 feature, because of the wasm issue
    // (rand requires stdweb / wasm-bindgen feature enabled to work on the wasm32 arch).
    // We have added rand as our direct dependency with wasm-bindgen feature enabled
    // and following is a copy & paste of the Uuid::new_v4() method to make it working.
    let mut rng = rand::thread_rng();
    let mut bytes = [0; 16];

    rng.fill_bytes(&mut bytes);

    let uuid = Uuid::from_random_bytes(bytes);

    Ok(Value::String(uuid.to_hyphenated().to_string()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::{Uuid, Version};

    use crate::context::Context;

    use super::uuidv4;

    #[test]
    fn result_is_valid_uuid_v4() {
        let mut ctx = Context::default();
        let args = HashMap::new();

        let uuid_value = uuidv4(&args, &mut ctx).unwrap();
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid = Uuid::parse_str(&uuid_str).unwrap();
        // Version::Random == UUIDv4
        assert_eq!(uuid.get_version(), Some(Version::Random));
    }
}
