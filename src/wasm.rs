//! This encloses everything that is needed to use this library from WASM

// https://github.com/rustwasm/console_error_panic_hook#readme
pub use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ast::Identifier;
use crate::Context;
use crate::Engine;
use crate::error::Error;

#[wasm_bindgen]
pub fn temen_evaluate(expression: JsValue, data: JsValue) -> Result<JsValue, JsValue> {
    // use console.log for nice errors from Rust-land
    console_error_panic_hook::set_once();

    let engine = Engine::default(); // Default functions, filters
    let mut ctx = Context::default(); // Default context
    let position = Identifier::default(); // Evaluate from the root

    let expression = expression
        .as_string()
        .ok_or_else(|| Error::with_message("expression must be a string"))
        .map_err(|e| JsValue::from(format!("{:#?}", e)))?;
    let data = data
        .as_string()
        .ok_or_else(|| Error::with_message("data must be a string"))
        .map_err(|e| JsValue::from(format!("{:#?}", e)))?;
    let data = serde_json::from_str(&data).map_err(|e| JsValue::from(format!("{:#?}", e)))?;
    engine
        .eval(&expression, &position, &data, &mut ctx)
        .map(|value| value.to_string().into())
        .map_err(|e| format!("{:#?}", e).into())
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use serde_json::json;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn run_in_browser() {
        let data = json!({});
        let expression = "1 + 2";
        let evaluated = temen_evaluate(expression.into(), data.to_string().into()).unwrap();
        assert_eq!( JsValue::from("3"), evaluated);
    }
}
