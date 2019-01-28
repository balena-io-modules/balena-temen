mod engine;
mod parser;

#[macro_export]
macro_rules! test_eval_eq {
    ($engine:expr, $expression:expr, $result:expr) => {{
        let mut context = balena_temen::Context::default();
        let position = balena_temen::ast::Identifier::default();
        let data = balena_temen::Value::Null;

        assert_eq!(
            $engine.eval($expression, &position, &data, &mut context).unwrap(),
            $result
        );
    }};
    ($expression:expr, $result:expr) => {{
        let engine = balena_temen::Engine::default();
        test_eval_eq!(engine, $expression, $result);
    }};
}

#[macro_export]
macro_rules! test_eval_err {
    ($engine:expr, $expression:expr) => {{
        let mut context = balena_temen::Context::default();
        let position = balena_temen::ast::Identifier::default();
        let data = balena_temen::Value::Null;

        assert!($engine.eval($expression, &position, &data, &mut context).is_err());
    }};
    ($expression:expr) => {{
        let engine = balena_temen::Engine::default();
        test_eval_err!(engine, $expression);
    }};
}

#[macro_export]
macro_rules! test_eval_ok {
    ($engine:expr, $expression:expr) => {{
        let mut context = balena_temen::Context::default();
        let position = balena_temen::ast::Identifier::default();
        let data = balena_temen::Value::Null;

        assert!($engine.eval($expression, &position, &data, &mut context).is_ok());
    }};
    ($expression:expr) => {{
        let engine = balena_temen::Engine::default();
        test_eval_ok!(engine, $expression);
    }};
}

#[macro_export]
macro_rules! test_eval_as_bool_eq {
    ($engine:expr, $expression:expr, $result:expr) => {{
        let mut context = balena_temen::Context::default();
        let position = balena_temen::ast::Identifier::default();
        let data = balena_temen::Value::Null;

        assert_eq!(
            $engine
                .eval_as_bool($expression, &position, &data, &mut context)
                .unwrap(),
            $result
        );
    }};
    ($expression:expr, $result:expr) => {{
        let engine = balena_temen::Engine::default();
        test_eval_as_bool_eq!(engine, $expression, $result);
    }};
}

#[macro_export]
macro_rules! test_eval_as_bool_err {
    ($engine:expr, $expression:expr) => {{
        let mut context = balena_temen::Context::default();
        let position = balena_temen::ast::Identifier::default();
        let data = balena_temen::Value::Null;

        assert!($engine
            .eval_as_bool($expression, &position, &data, &mut context)
            .is_err());
    }};
    ($expression:expr) => {{
        let engine = balena_temen::Engine::default();
        test_eval_as_bool_err!(engine, $expression);
    }};
}

#[macro_export]
macro_rules! test_eval_as_bool_ok {
    ($engine:expr, $expression:expr) => {{
        let mut context = balena_temen::Context::default();
        let position = balena_temen::ast::Identifier::default();
        let data = balena_temen::Value::Null;

        assert!($engine
            .eval_as_bool($expression, &position, &data, &mut context)
            .is_ok());
    }};
    ($expression:expr) => {{
        let engine = balena_temen::Engine::default();
        test_eval_as_bool_ok!(engine, $expression);
    }};
}

#[macro_export]
macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {
        assert_eq!(
            ($e.parse() as balena_temen::error::Result<balena_temen::ast::Expression>).unwrap(),
            $r
        );
    };
}

#[macro_export]
macro_rules! test_parse_err {
    ($e:expr) => {
        assert!(($e.parse() as balena_temen::error::Result<balena_temen::ast::Expression>).is_err());
    };
}
