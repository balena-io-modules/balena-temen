use serde_json::json;

use balena_temen::{evaluate, evaluate_with_engine, Context, Engine, EngineBuilder};

#[test]
fn one_formula_fail_second_formula_success() {
    // Lucian & Alex calculation project
    let data = json!({
        "foo": {
            "$$formula": "UUIDV4()"
        },
        "prop": {
            "$$formula": "super.notExistingProperty"
        }
    });

    assert!(evaluate(data).is_err());
}
