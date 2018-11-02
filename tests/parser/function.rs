use balena_template::{error::Error, parser::ast::*};
use std::collections::HashMap;

macro_rules! test_parse_eq {
    ($e:expr, $r:expr) => {{
        assert_eq!(($e.parse() as Result<Expression, Error>).unwrap(), $r);
    }};
}

macro_rules! fn_args_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), Expression::new($value));
            )+
            m
        }
     };
);

#[test]
fn test_function() {
    let args = fn_args_map! {
        "v" => ExpressionValue::Integer(4),
        "dummy" => ExpressionValue::String("abc".to_string())
    };

    test_parse_eq!(
        "uuid()",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", HashMap::new())))
    );
    test_parse_eq!(
        "uuid(v=4, dummy=`abc`)",
        Expression::new(ExpressionValue::FunctionCall(FunctionCall::new("uuid", args)))
    );
}
