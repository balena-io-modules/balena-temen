use serde_json::{Number, Value};

use crate::context::Context;
use crate::error::{Error, Result};

fn validate_args_len(args: &[Value], expected: usize, name: &'static str) -> Result<()> {
    if args.len() != expected {
        return Err(Error::with_message("invalid number of arguments")
            .context("function", name)
            .context("argument count", format!("{}", args.len()))
            .context("expected", format!("{}", expected)));
    }
    Ok(())
}

fn get_one_arg<T, F>(args: &[Value], f: F) -> Result<T>
where
    F: FnOnce(&Value) -> Option<T>,
{
    args.first()
        .and_then(f)
        .ok_or_else(|| Error::with_message("invalid argument type"))
}

fn get_two_args<T1, T2, F1, F2>(args: &[Value], f1: F1, f2: F2) -> Result<(T1, T2)>
where
    F1: FnOnce(&Value) -> Option<T1>,
    F2: FnOnce(&Value) -> Option<T2>,
{
    let r1 = args
        .get(0)
        .and_then(f1)
        .ok_or_else(|| Error::with_message("invalid argument type"))?;
    let r2 = args
        .get(1)
        .and_then(f2)
        .ok_or_else(|| Error::with_message("invalid argument type"))?;
    Ok((r1, r2))
}

pub(crate) fn pow(args: &[Value], _context: &mut Context) -> Result<Value> {
    validate_args_len(args, 2, "POW")?;

    if let Ok((b, e)) = get_two_args(args, Value::as_i64, Value::as_u64) {
        return Ok(Value::from(b.pow(e as u32)));
    }

    let (b, e) = get_two_args(args, Value::as_f64, Value::as_f64)?;
    Ok(Value::Number(Number::from_f64(b.powf(e)).ok_or_else(|| {
        Error::with_message("expressions results to NaN").context("function", "POW")
    })?))
}

pub(crate) fn log10(args: &[Value], _context: &mut Context) -> Result<Value> {
    validate_args_len(args, 1, "LOG10")?;

    let x = get_one_arg(args, Value::as_f64)?;
    Ok(Value::Number(Number::from_f64(x.log10()).ok_or_else(|| {
        Error::with_message("expressions results to NaN").context("function", "LOG10")
    })?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn pow_10_2() {
        let mut ctx = Context::default();
        let args = vec![Value::Number(Number::from(10)), Value::Number(Number::from(2))];

        let result = pow(&args, &mut ctx).unwrap().as_i64().unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn pow_7_3() {
        let mut ctx = Context::default();
        let args = vec![Value::Number(Number::from(7)), Value::Number(Number::from(3))];

        let result = pow(&args, &mut ctx).unwrap().as_i64().unwrap();
        assert_eq!(result, 343);
    }

    #[test]
    fn pow_4_0_5() {
        let mut ctx = Context::default();
        let args = vec![
            Value::Number(Number::from(4)),
            Value::Number(Number::from_f64(0.5).unwrap()),
        ];

        let result = pow(&args, &mut ctx).unwrap().as_f64().unwrap();
        assert_relative_eq!(result, 2.0);
    }

    #[test]
    fn pow_7_minus_2() {
        let mut ctx = Context::default();
        let args = vec![Value::Number(Number::from(7)), Value::Number(Number::from(-2))];

        let result = pow(&args, &mut ctx).unwrap().as_f64().unwrap();
        assert_relative_eq!(result, 0.020_408_163_265_306_12);
    }

    #[test]
    fn pow_minus_7_0_5() {
        let mut ctx = Context::default();
        let args = vec![
            Value::Number(Number::from(-7)),
            Value::Number(Number::from_f64(0.5).unwrap()),
        ];

        assert!(pow(&args, &mut ctx).is_err());
    }

    #[test]
    fn log10_100000() {
        let mut ctx = Context::default();

        let args = vec![Value::Number(Number::from(100_000))];
        let result = log10(&args, &mut ctx).unwrap().as_f64().unwrap();
        assert_relative_eq!(result, 5.0);
    }

    #[test]
    fn log10_2() {
        let mut ctx = Context::default();

        let args = vec![Value::Number(Number::from(2))];
        let result = log10(&args, &mut ctx).unwrap().as_f64().unwrap();
        assert_relative_eq!(result, 0.301_029_995_663_981_2);
    }

    #[test]
    fn log10_1() {
        let mut ctx = Context::default();

        let args = vec![Value::Number(Number::from(1))];
        let result = log10(&args, &mut ctx).unwrap().as_f64().unwrap();
        assert_relative_eq!(result, 0.0);
    }

    #[test]
    fn log10_0() {
        // Result is -Inf, but JSON doesn't support these values
        let mut ctx = Context::default();
        let args = vec![Value::Number(Number::from(0))];
        assert!(log10(&args, &mut ctx).is_err());
    }
}
