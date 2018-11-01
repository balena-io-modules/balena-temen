use self::context::Context;
use crate::{
    builtin::{
        filter::{self, FilterFn},
        function::{self, FunctionFn},
    },
    error::{bail, Result},
    parser::ast::*,
};
use serde_json::{Number, Value};
use std::collections::HashMap;

pub mod context;

pub struct Engine {
    functions: HashMap<String, FunctionFn>,
    filters: HashMap<String, FilterFn>,
}

impl Default for Engine {
    fn default() -> Engine {
        let mut filters = HashMap::new();
        filters.insert("upper".into(), Box::new(filter::upper) as FilterFn);
        filters.insert("lower".into(), Box::new(filter::lower) as FilterFn);

        let mut functions = HashMap::new();
        functions.insert("uuidv4".into(), Box::new(function::uuidv4) as FunctionFn);

        Engine { functions, filters }
    }
}

impl Engine {
    fn eval_math(&self, lhs: &Number, rhs: &Number, operator: MathOperator) -> Result<Number> {
        match operator {
            MathOperator::Addition => {
                if lhs.is_i64() && rhs.is_i64() {
                    if let Some(x) = lhs.as_i64().unwrap().checked_add(rhs.as_i64().unwrap()) {
                        return Ok(Number::from(x));
                    }
                }

                let lhs = lhs.as_f64().unwrap();
                let rhs = rhs.as_f64().unwrap();
                let result = lhs + rhs;

                if result.is_finite() {
                    Ok(Number::from_f64(result).unwrap())
                } else {
                    bail!("result of `{} + {}` is not finite", lhs, rhs);
                }
            }
            MathOperator::Subtraction => {
                if lhs.is_i64() && rhs.is_i64() {
                    if let Some(x) = lhs.as_i64().unwrap().checked_sub(rhs.as_i64().unwrap()) {
                        return Ok(Number::from(x));
                    }
                }

                let lhs = lhs.as_f64().unwrap();
                let rhs = rhs.as_f64().unwrap();
                let result = lhs - rhs;

                if result.is_finite() {
                    Ok(Number::from_f64(result).unwrap())
                } else {
                    bail!("result of `{} - {}` is not finite", lhs, rhs);
                }
            }
            MathOperator::Multiplication => {
                if lhs.is_i64() && rhs.is_i64() {
                    if let Some(x) = lhs.as_i64().unwrap().checked_mul(rhs.as_i64().unwrap()) {
                        return Ok(Number::from(x));
                    }
                }

                let lhs = lhs.as_f64().unwrap();
                let rhs = rhs.as_f64().unwrap();
                let result = lhs * rhs;

                if result.is_finite() {
                    Ok(Number::from_f64(result).unwrap())
                } else {
                    bail!("result of `{} * {}` is not finite", lhs, rhs);
                }
            }
            MathOperator::Modulo => {
                if lhs.is_i64() && rhs.is_i64() {
                    if let Some(x) = lhs.as_i64().unwrap().checked_rem(rhs.as_i64().unwrap()) {
                        return Ok(Number::from(x));
                    }
                }

                let lhs = lhs.as_f64().unwrap();
                let rhs = rhs.as_f64().unwrap();
                let result = lhs % rhs;

                if result.is_finite() {
                    Ok(Number::from_f64(result).unwrap())
                } else {
                    bail!("result of `{} % {}` is not finite", lhs, rhs);
                }
            }
            MathOperator::Division => {
                let lhs = lhs.as_f64().unwrap();
                let rhs = rhs.as_f64().unwrap();
                let result = lhs / rhs;

                if result.is_finite() {
                    Ok(Number::from_f64(result).unwrap())
                } else {
                    bail!("result of `{} / {}` is not finite", lhs, rhs);
                }
            }
        }
    }

    fn eval_args(&self, args: &HashMap<String, Expression>, context: &Context) -> Result<HashMap<String, Value>> {
        let mut result = HashMap::new();

        for (k, v) in args.iter() {
            result.insert(k.to_string(), self.eval(v, context)?);
        }

        Ok(result)
    }

    fn eval_function(&self, name: &str, args: &HashMap<String, Expression>, context: &Context) -> Result<Value> {
        let args = self.eval_args(args, context)?;

        let f = self.functions.get(name);

        if f.is_none() {
            bail!("function `{}` not found", name);
        }

        (f.unwrap())(&args)
    }

    fn eval_filter(&self, name: &str, value: &Value) -> Result<Value> {
        let f = self.filters.get(name);

        if f.is_none() {
            bail!("filter `{}` not found", name);
        }

        (f.unwrap())(value)
    }

    fn eval_value_as_number(&self, value: &ExpressionValue, context: &Context) -> Result<Number> {
        let number = match value {
            ExpressionValue::Integer(x) => Number::from(*x),
            ExpressionValue::Float(x) => Number::from_f64(*x).unwrap(),
            ExpressionValue::Identifier(x) => match context.lookup_variable(x)? {
                Value::Number(ref x) => x.clone(),
                _ => bail!("identifier does not evaluate to a number"),
            },
            ExpressionValue::Math(MathExpression {
                ref lhs,
                ref rhs,
                ref operator,
            }) => {
                let lhs = self.eval_as_number(lhs, context)?;
                let rhs = self.eval_as_number(rhs, context)?;
                self.eval_math(&lhs, &rhs, *operator)?
            }
            ExpressionValue::FunctionCall(FunctionCall { ref name, ref args }) => {
                match self.eval_function(name, args, context)? {
                    Value::Number(x) => x,
                    _ => bail!("result of `{}` is not a number", name),
                }
            }
            ExpressionValue::Boolean(_) => bail!("unable to evaluate boolean as a number"),
            ExpressionValue::String(_) => bail!("unable to evaluate string as a number"),
            ExpressionValue::Logical(_) => bail!("unable to evaluate logical expression as a number"),
            ExpressionValue::StringConcat(_) => bail!("unable to evaluate string concatenation as a number"),
        };

        Ok(number)
    }

    fn eval_as_number(&self, expression: &Expression, context: &Context) -> Result<Number> {
        if expression.filters.is_empty() {
            // We can directly evaluate the value as a number, because
            // we have no filters
            return self.eval_value_as_number(&expression.value, context);
        }

        // In case of filters, just evaluate the expression as a generic one
        // and check if the result is a Number
        if let Value::Number(number) = self.eval(expression, context)? {
            return Ok(number);
        }

        bail!("unable to evaluate expression as a number: {:?}", expression)
    }

    pub fn eval(&self, expression: &Expression, context: &Context) -> Result<Value> {
        let mut result = match expression.value {
            ExpressionValue::Integer(x) => Value::Number(Number::from(x)),
            ExpressionValue::Float(x) => Value::Number(Number::from_f64(x).unwrap()),
            ExpressionValue::Boolean(x) => Value::Bool(x),
            ExpressionValue::String(ref x) => Value::String(x.to_string()),
            ExpressionValue::Identifier(ref x) => context.lookup_variable(x)?.clone(),
            ExpressionValue::Math(_) => Value::Number(self.eval_as_number(expression, context)?),
            ExpressionValue::Logical(_) => Value::Bool(self.eval_value_as_bool(&expression.value, context)?),
            ExpressionValue::FunctionCall(FunctionCall { ref name, ref args }) => {
                self.eval_function(name, args, context)?
            }
            ExpressionValue::StringConcat(StringConcat { ref values }) => {
                let mut result = String::new();

                for value in values {
                    match value {
                        ExpressionValue::String(ref x) => result.push_str(x),
                        ExpressionValue::Integer(x) => result.push_str(&format!("{}", x)),
                        ExpressionValue::Float(x) => result.push_str(&format!("{}", x)),
                        ExpressionValue::Identifier(ref x) => match context.lookup_variable(x)? {
                            Value::String(ref x) => result.push_str(x),
                            Value::Number(ref x) => result.push_str(&format!("{}", x)),
                            _ => bail!(
                                "unable to concatenate string (identifier does not evaluated to a number / string)"
                            ),
                        },
                        _ => bail!("unable to concatenate string (string, number or identifiers supported only)"),
                    };
                }

                Value::String(result)
            }
        };

        for filter in expression.filters.iter() {
            result = self.eval_filter(&filter.name, &result)?;
        }

        if expression.negated {
            if let Value::Bool(x) = result {
                result = Value::Bool(!x);
            } else {
                bail!("unable to negate non bool value");
            }
        }

        Ok(result)
    }

    fn eval_value_as_bool(&self, value: &ExpressionValue, context: &Context) -> Result<bool> {
        let result = match value {
            ExpressionValue::Integer(_) => bail!("integer can't be evaluated as bool"),
            ExpressionValue::Float(_) => bail!("float can't be evaluated as bool"),
            ExpressionValue::Boolean(x) => *x,
            ExpressionValue::String(_) => bail!("string can't be evaluated as bool"),
            ExpressionValue::Identifier(x) => match context.lookup_variable(x)? {
                Value::Bool(x) => *x,
                _ => bail!("identifier does not evaluated to a boolean"),
            },
            ExpressionValue::Math(_) => bail!("math expression can't be evaluated as bool"),
            ExpressionValue::Logical(LogicalExpression {
                ref lhs,
                ref rhs,
                ref operator,
            }) => match operator {
                LogicalOperator::And => self.eval_as_bool(lhs, context)? && self.eval_as_bool(rhs, context)?,
                LogicalOperator::Or => self.eval_as_bool(lhs, context)? || self.eval_as_bool(rhs, context)?,
                LogicalOperator::Equal | LogicalOperator::NotEqual => {
                    let mut lhs = self.eval(lhs, context)?;
                    let mut rhs = self.eval(rhs, context)?;

                    if lhs.is_number() {
                        lhs = Value::Number(Number::from_f64(lhs.as_f64().unwrap()).unwrap());
                    }

                    if rhs.is_number() {
                        rhs = Value::Number(Number::from_f64(rhs.as_f64().unwrap()).unwrap());
                    }

                    if operator == &LogicalOperator::Equal {
                        lhs == rhs
                    } else {
                        lhs != rhs
                    }
                }
                LogicalOperator::GreaterThan
                | LogicalOperator::GreaterThanOrEqual
                | LogicalOperator::LowerThan
                | LogicalOperator::LowerThanOrEqual => {
                    let lhs = self.eval_as_number(lhs, context)?.as_f64().unwrap();
                    let rhs = self.eval_as_number(rhs, context)?.as_f64().unwrap();

                    match operator {
                        LogicalOperator::GreaterThan => lhs > rhs,
                        LogicalOperator::GreaterThanOrEqual => lhs >= rhs,
                        LogicalOperator::LowerThan => lhs < rhs,
                        LogicalOperator::LowerThanOrEqual => lhs <= rhs,
                        _ => unreachable!(),
                    }
                }
            },
            ExpressionValue::FunctionCall(FunctionCall { ref name, ref args }) => {
                match self.eval_function(name, args, context)? {
                    Value::Bool(x) => x,
                    _ => bail!("unable to evaluate `{}` function result as bool", name),
                }
            }
            ExpressionValue::StringConcat(_) => bail!("string concatenation can't be evaluated as bool"),
        };

        Ok(result)
    }

    pub fn eval_as_bool(&self, expression: &Expression, context: &Context) -> Result<bool> {
        let mut value = self.eval_value_as_bool(&expression.value, context)?;

        if expression.negated {
            value = !value;
        }

        Ok(value)
    }
}
