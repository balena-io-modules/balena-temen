use rand::{self, RngCore};
use serde_json::Value;
use uuid::{builder::Builder, Variant, Version};

use crate::context::Context;
use crate::error::{Error, Result};

pub(crate) fn uuidv4(_args: &[Value], _context: &mut Context) -> Result<Value> {
    // We're using uuid crate with disabled v4 feature, because of the wasm issue
    // (rand requires stdweb / wasm-bindgen feature enabled to work on the wasm32 arch).
    // We have added rand as our direct dependency with wasm-bindgen feature enabled
    // and following is a copy & paste of the Uuid::new_v4() method to make it working.
    let mut rng = rand::thread_rng();
    let mut bytes = [0; 16];

    rng.fill_bytes(&mut bytes);

    let uuid = Builder::from_slice(&bytes)
        .map_err(|_| Error::with_message("random generator failure"))?
        .set_version(Version::Random)
        .set_variant(Variant::RFC4122)
        .build();

    Ok(Value::String(uuid.to_hyphenated().to_string()))
}

#[cfg(test)]
mod tests {
    use uuid::{Uuid, Version};

    use crate::context::Context;

    use super::uuidv4;

    #[test]
    fn result_is_valid_uuid_v4() {
        let mut ctx = Context::default();

        let uuid_value = uuidv4(&[], &mut ctx).unwrap();
        let uuid_str = uuid_value.as_str().unwrap();
        let uuid = Uuid::parse_str(&uuid_str).unwrap();
        // Version::Random == UUIDv4
        assert_eq!(uuid.get_version(), Some(Version::Random));
    }
}
