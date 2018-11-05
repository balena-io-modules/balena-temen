use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval_as_bool(&$e.parse().unwrap(), &context).is_err());
    }};
}

#[test]
fn test_uuidv4() {
    // Evaluates to String
    test_eval_err!("uuidv4()");
}
