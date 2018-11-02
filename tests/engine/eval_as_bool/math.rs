use balena_template::engine::context::Context;
use balena_template::engine::Engine;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval_as_bool(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval_as_bool(&$e.parse().unwrap(), &context).is_err());
    }};
}

#[test]
fn test_math() {
    test_eval_eq!("1 - 1 == 2 - 2", true);
    test_eval_eq!("3 * 9 == 27", true);

    // Math expression == number - can't be evaluated as bool
    test_eval_err!("1 - 1");
    test_eval_err!("2 - 1");
}
