#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate balena_temen;

fuzz_target!(|data: &[u8]| {
    if let Ok(json_string) = std::str::from_utf8(data) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_string) {
            let _ = balena_temen::evaluate(json);
        }
    }
});
