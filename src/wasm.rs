//! This encloses everything that is needed to use this library from WASM

// https://github.com/rustwasm/console_error_panic_hook#readme
pub use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::eval;

/// This re-exports `eval` into WASM-land, as `temen_evaluate`, with a prefix to avoid global namespace confusion
#[wasm_bindgen]
pub fn temen_evaluate(data: JsValue) -> Result<JsValue, JsValue> {
    // use console.log for nice errors from Rust-land
    console_error_panic_hook::set_once();

    let data = data.into_serde().map_err(|e| JsValue::from(format!("{:#?}", e)))?;
    eval(data)
        .map(|value| JsValue::from(value.to_string()))
        .map_err(|e| JsValue::from(format!("{:#?}", e)))
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use serde_json::json;
    use pretty_assertions::assert_eq;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn run_as_wasm_in_browser() {
        let data = JsValue::from_serde(&json!(
        {
            "number": 3,
            "value": {
                 "$$eval": "super.number + 5"
            }
        }))
        .unwrap();

        let evaluated = temen_evaluate(data).unwrap().as_string().unwrap();
        // use same serde_json `.to_string` to format both, to not rely on
        // JSValue formatter and serde_json formatter to be same
        let evaluated: serde_json::Value = serde_json::from_str(&evaluated).unwrap();
        let evaluated = evaluated.to_string();

        let expected = json!(
        {
            "number": 3,
            "value": 8
        })
        .to_string();

        // equality operators on JsValues do not seem to produce expected results
        // comparing string representations here instead
        assert_eq!(expected, evaluated);
    }
}
