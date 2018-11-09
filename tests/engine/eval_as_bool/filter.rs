use balena_temen::{Context, Engine};

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval_as_bool(&$e.parse().unwrap(), &context, None).is_err());
    }};
}

#[test]
fn fail_on_invalid_output_type() {
    // Evaluates to String
    test_eval_err!("`true` | lower");
}
