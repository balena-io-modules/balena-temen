use serde_json::Value;

pub struct Context {
    #[allow(dead_code)]
    data: Value,
}

impl Context {
    pub fn new(data: Value) -> Context {
        Context { data }
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new(Value::Null)
    }
}
