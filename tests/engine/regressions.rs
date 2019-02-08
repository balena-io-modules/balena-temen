use serde_json::json;

use balena_temen::evaluate;

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

#[test]
fn fuzzer_invalid_overflow() {
    // Found by Cyryl's fuzzer
    // This isn't about parser input, but evaluation engine recursive
    // loop, which was caused by the eval_as_number function. Called by
    // eval_expression and eval_as_number called eval_expression again.
    let data = json!({
        " ssid": "S(ssidool SSID Network!",
        "id": {
            "$$formula": "sup/r.ssid | SLUGIFY"
        }
    });

    assert!(evaluate(data).is_err());
}
