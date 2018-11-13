use std::collections::HashMap;

use serde_json::{Number, Value};

use crate::{
    ast::*,
    builtin::{
        filter::{self, FilterFn},
        function::{self, FunctionFn},
    },
    context::Context,
    error::{bail, Result},
    lookup::Lookup,
    utils::{RelativeEq, validate_f64}
};

/// A custom engine builder
///
/// Allows to build an [`Engine`] with custom filters, functions or the evaluation keyword.
///
/// [`Engine`]: struct.Engine.html
pub struct EngineBuilder {
    functions: HashMap<String, FunctionFn>,
    filters: HashMap<String, FilterFn>,
    eval_keyword: Option<String>,
}

impl Default for EngineBuilder {
    /// Creates new [`EngineBuilder`] with default filters, functions and the evaluation keyword
    ///
    /// [`EngineBuilder`]: struct.EngineBuilder.html
    fn default() -> EngineBuilder {
        EngineBuilder::new()
            .filter("upper", filter::upper)
            .filter("lower", filter::lower)
            .filter("time", filter::time)
            .filter("date", filter::date)
            .filter("datetime", filter::datetime)
            .filter("trim", filter::trim)
            .filter("slugify", filter::slugify)
            .function("uuidv4", function::uuidv4)
            .function("now", function::now)
    }
}

impl EngineBuilder {
    /// Creates new, empty, [`EngineBuilder`]
    ///
    /// No filters and functions are registered.
    ///
    /// [`EngineBuilder`]: struct.EngineBuilder.html
    fn new() -> EngineBuilder {
        EngineBuilder {
            functions: HashMap::new(),
            filters: HashMap::new(),
            eval_keyword: None,
        }
    }

    /// Registers custom filter
    ///
    /// If a filter with the name already exists, it will be overwritten.
    ///
    /// Visit [`FilterFn`] to learn more about filters.
    ///
    /// # Arguments
    ///
    /// * `name` - Custom filter name
    /// * `filter` - Custom filter function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balena_temen::{
    ///   ast::Identifier,
    ///   Engine, EngineBuilder, Context, Value,
    ///   error::Result
    /// };
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// fn text_filter(input: &Value, args: &HashMap<String, Value>, _: &mut Context) -> Result<Value> {
    ///     let input = input.as_str()
    ///         .ok_or_else(|| "`text` filter accepts string only")?;
    ///
    ///     let trim = args.get("trim")
    ///         .unwrap_or_else(|| &Value::Bool(false))
    ///         .as_bool()
    ///         .ok_or_else(|| "`trim` argument must be a bool")?;
    ///
    ///     let upper = args.get("upper")
    ///         .unwrap_or_else(|| &Value::Bool(false))
    ///         .as_bool()
    ///         .ok_or_else(|| "`upper` argument must be a bool")?;
    ///
    ///     let result = match (trim, upper) {
    ///         (false, false) => input.to_string(),
    ///         (true, false) => input.trim().to_string(),
    ///         (false, true) => input.to_uppercase(),
    ///         (true, true) => input.trim().to_uppercase(),
    ///     };
    ///
    ///     Ok(Value::String(result))
    /// };
    ///
    /// let engine: Engine = EngineBuilder::default()
    ///     .filter("text", text_filter)
    ///     .into();
    /// let mut ctx = Context::default();
    /// let position = Identifier::default();
    /// let data = Value::Null;
    ///
    /// assert_eq!(
    ///     engine.eval("` abc ` | text", &position, &data, &mut ctx).unwrap(),
    ///     json!(" abc ")
    /// );
    /// assert_eq!(
    ///     engine.eval("` abc ` | text(trim=true)", &position, &data, &mut ctx).unwrap(),
    ///     json!("abc")
    /// );
    /// assert_eq!(
    ///     engine.eval("` abc ` | text(trim=true, upper=true)", &position, &data, &mut ctx).unwrap(),
    ///     json!("ABC")
    /// );
    /// ```
    ///
    /// [`FilterFn`]: type.FilterFn.html
    pub fn filter<S>(self, name: S, filter: FilterFn) -> EngineBuilder
    where
        S: Into<String>,
    {
        let mut filters = self.filters;
        filters.insert(name.into(), filter);
        EngineBuilder {
            functions: self.functions,
            filters,
            eval_keyword: self.eval_keyword,
        }
    }

    /// Registers custom function
    ///
    /// If a function with the name already exists, it will be overwritten.
    ///
    /// Visit [`FunctionFn`] to learn more about functions.
    ///
    /// # Arguments
    ///
    /// * `name` - Custom function name
    /// * `function` - Custom function function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balena_temen::{
    ///   ast::Identifier,
    ///   Engine, EngineBuilder, Context, Value,
    ///   error::Result
    /// };
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// fn echo_function(args: &HashMap<String, Value>, _: &mut Context) -> Result<Value> {
    ///     let value = match args.get("value") {
    ///         Some(x) => {
    ///             x.as_str().ok_or_else(|| "`value` argument must be a string")?
    ///         },
    ///         None => "echo"
    ///     };
    ///
    ///     Ok(Value::String(value.to_string()))
    /// };
    ///
    /// let engine: Engine = EngineBuilder::default()
    ///     .function("echo", echo_function)
    ///     .into();
    /// let mut ctx = Context::default();
    /// let position = Identifier::default();
    /// let data = Value::Null;
    ///
    /// assert_eq!(
    ///     engine.eval("echo()", &position, &data, &mut ctx).unwrap(),
    ///     json!("echo")
    /// );
    /// assert_eq!(
    ///     engine.eval("echo(value=`Hallo`)", &position, &data, &mut ctx).unwrap(),
    ///     json!("Hallo")
    /// );
    /// assert!(
    ///     engine.eval("echo(value=1)", &position, &data, &mut ctx).is_err()
    /// );
    /// ```
    ///
    /// [`FunctionFn`]: type.FunctionFn.html
    pub fn function<S>(self, name: S, function: FunctionFn) -> EngineBuilder
    where
        S: Into<String>,
    {
        let mut functions = self.functions;
        functions.insert(name.into(), function);
        EngineBuilder {
            functions,
            filters: self.filters,
            eval_keyword: self.eval_keyword,
        }
    }

    /// Registers custom evaluation keyword
    ///
    /// Defaults to `$$eval` if no keyword is registered.
    ///
    /// # Arguments
    ///
    /// * `keyword` - An evaluation keyword
    ///
    /// # Examples
    ///
    // TODO Add example
    pub fn eval_keyword<S>(self, keyword: S) -> EngineBuilder
    where
        S: Into<String>,
    {
        EngineBuilder {
            functions: self.functions,
            filters: self.filters,
            eval_keyword: Some(keyword.into()),
        }
    }
}

impl From<EngineBuilder> for Engine {
    fn from(builder: EngineBuilder) -> Engine {
        Engine {
            functions: builder.functions,
            filters: builder.filters,
            eval_keyword: builder.eval_keyword.unwrap_or_else(|| "$$eval".into()),
        }
    }
}

/// An expression evaluation engine
pub struct Engine {
    functions: HashMap<String, FunctionFn>,
    filters: HashMap<String, FilterFn>,
    #[allow(dead_code)]
    eval_keyword: String,
}

impl Default for Engine {
    /// Creates new [`Engine`] with default set of functions, filters and the evaluation keyword.
    ///
    /// [`Engine`]: struct.Engine.html
    fn default() -> Engine {
        EngineBuilder::default().into()
    }
}

impl Engine {
    /// Evaluates an expression
    ///
    /// Result can be any valid JSON value.
    ///
    /// # Arguments
    ///
    /// * `expression` - An expression to evaluate
    /// * `position` - An initial position for relative identifiers
    /// * `data` - A JSON with variable values
    /// * `context` - An evaluation context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balena_temen::{
    ///   ast::Identifier,
    ///   Engine, Context, Value
    /// };
    /// use serde_json::json;
    ///
    /// let engine = Engine::default();       // Default functions, filters
    /// let mut ctx = Context::default();     // Default context
    /// let position = Identifier::default(); // Evaluate from the root
    /// let data = json!({
    ///   "numbers": {
    ///     "one": 1,
    ///     "two": 2
    ///   },
    ///   "names": [
    ///     "zero",
    ///     "one",
    ///     "two"
    ///   ]
    /// });
    ///
    /// // Math expression
    ///
    /// assert_eq!(
    ///     engine.eval("2 + 3", &position, &data, &mut ctx).unwrap(),
    ///     json!(5)
    /// );
    ///
    /// // Filters
    ///
    /// assert_eq!(
    ///     engine.eval("`Balena is great!` | slugify", &position, &data, &mut ctx).unwrap(),
    ///     json!("balena-is-great")
    /// );
    ///
    /// // Variables
    ///
    /// assert_eq!(
    ///     engine.eval("numbers.one + numbers.two", &position, &data, &mut ctx).unwrap(),
    ///     json!(3)
    /// );
    /// assert_eq!(
    ///     engine.eval("numbers[`one`] * numbers[`two`]", &position, &data, &mut ctx).unwrap(),
    ///     json!(2)
    /// );
    ///
    /// // Indirect / nested variables
    ///
    /// assert_eq!(
    ///     engine.eval("numbers[names[1]] + numbers[names[2]]", &position, &data, &mut ctx).unwrap(),
    ///     json!(3)
    /// );
    /// assert_eq!(
    ///     engine.eval("numbers[names.1] + numbers[names.2]", &position, &data, &mut ctx).unwrap(),
    ///     json!(3)
    /// );
    /// ```
    pub fn eval(&self, expression: &str, position: &Identifier, data: &Value, context: &mut Context) -> Result<Value> {
        let expression = expression.parse()?;
        self.eval_expression(&expression, position, data, context)
    }

    /// Evaluates an expression as a boolean
    ///
    /// Result must evaluate to a boolean value otherwise it fails. Numbers, strings, ... do not
    /// evaluate to a boolean like in other languages.
    ///
    /// # Arguments
    ///
    /// * `expression` - An expression to evaluate
    /// * `position` - An initial position for relative identifiers
    /// * `data` - A JSON with variable values
    /// * `context` - An evaluation context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balena_temen::{
    ///   ast::Identifier,
    ///   Engine, Context, Value
    /// };
    /// use serde_json::json;
    ///
    /// let engine = Engine::default();       // Default functions, filters
    /// let mut ctx = Context::default();     // Default context
    /// let position = Identifier::default(); // Evaluate from the root
    /// let data = Value::Null;               // No data (variables)
    ///
    /// assert_eq!(
    ///     engine.eval_as_bool("2 == 2 + 3", &position, &data, &mut ctx).unwrap(),
    ///     json!(false)
    /// );
    ///
    /// // An expression MUST evaluate to a boolean otherwise the evaluation fails
    ///
    /// assert!(
    ///     engine.eval_as_bool("1", &position, &data, &mut ctx).is_err()
    /// );
    ///
    /// // Invalid syntax leads to a failure too
    ///
    /// assert!(
    ///     engine.eval_as_bool("true ==", &position, &data, &mut ctx).is_err()
    /// );
    /// ```
    pub fn eval_as_bool(
        &self,
        expression: &str,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<bool> {
        let expression = expression.parse()?;
        self.eval_expression_as_bool(&expression, position, data, context)
    }

    #[allow(dead_code)]
    pub(crate) fn eval_keyword(&self) -> &str {
        &self.eval_keyword
    }

    fn eval_math(&self, lhs: &Number, rhs: &Number, operator: MathOperator) -> Result<Number> {
        // TODO Extract to a generic function
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

                Ok(Number::from_f64(validate_f64(result)?).unwrap())
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

                Ok(Number::from_f64(validate_f64(result)?).unwrap())
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

                Ok(Number::from_f64(validate_f64(result)?).unwrap())
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

                Ok(Number::from_f64(validate_f64(result)?).unwrap())
            }
            MathOperator::Division => {
                if lhs.is_i64() && rhs.is_i64() {
                    // Try to divide integers and if there's no remained, return result as integer as well
                    if let Some(0) = lhs.as_i64().unwrap().checked_rem(rhs.as_i64().unwrap()) {
                        if let Some(x) = lhs.as_i64().unwrap().checked_div(rhs.as_i64().unwrap()) {
                            return Ok(Number::from(x));
                        }
                    }
                }

                let lhs = lhs.as_f64().unwrap();
                let rhs = rhs.as_f64().unwrap();
                let result = lhs / rhs;

                Ok(Number::from_f64(validate_f64(result)?).unwrap())
            }
        }
    }

    fn eval_args(
        &self,
        args: &HashMap<String, Expression>,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<HashMap<String, Value>> {
        let mut result = HashMap::new();

        for (k, v) in args.iter() {
            result.insert(k.to_string(), self.eval_expression(v, position, data, context)?);
        }

        Ok(result)
    }

    fn eval_function(
        &self,
        name: &str,
        args: &HashMap<String, Expression>,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<Value> {
        let args = self.eval_args(args, position, data, context)?;

        if let Some(f) = self.functions.get(name) {
            f(&args, context)
        } else {
            bail!("function `{}` not found", name);
        }
    }

    fn eval_filter(
        &self,
        name: &str,
        input: &Value,
        args: &HashMap<String, Expression>,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<Value> {
        let args = self.eval_args(args, position, data, context)?;

        if let Some(f) = self.filters.get(name) {
            f(input, &args, context)
        } else {
            bail!("filter `{}` not found", name);
        }
    }

    fn eval_value_as_number(
        &self,
        value: &ExpressionValue,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<Number> {
        let number = match value {
            ExpressionValue::Integer(x) => Number::from(*x),
            ExpressionValue::Float(x) => Number::from_f64(*x).unwrap(),
            ExpressionValue::Identifier(x) => match data.lookup_identifier(x, position)? {
                Value::Number(ref x) => x.clone(),
                _ => bail!("identifier does not evaluate to a number"),
            },
            ExpressionValue::Math(MathExpression {
                ref lhs,
                ref rhs,
                ref operator,
            }) => {
                let lhs = self.eval_as_number(lhs, position, data, context)?;
                let rhs = self.eval_as_number(rhs, position, data, context)?;
                self.eval_math(&lhs, &rhs, *operator)?
            }
            ExpressionValue::FunctionCall(FunctionCall { ref name, ref args }) => {
                match self.eval_function(name, args, position, data, context)? {
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

    fn eval_as_number(
        &self,
        expression: &Expression,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<Number> {
        if expression.filters.is_empty() {
            // We can directly evaluate the value as a number, because
            // we have no filters
            return self.eval_value_as_number(&expression.value, position, data, context);
        }

        // In case of filters, just evaluate the expression as a generic one
        // and check if the result is a Number
        if let Value::Number(number) = self.eval_expression(expression, position, data, context)? {
            return Ok(number);
        }

        bail!("unable to evaluate expression as a number: {:?}", expression)
    }

    fn eval_expression(
        &self,
        expression: &Expression,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<Value> {
        let mut result = match expression.value {
            ExpressionValue::Integer(x) => Value::Number(Number::from(x)),
            ExpressionValue::Float(x) => Value::Number(Number::from_f64(x).unwrap()),
            ExpressionValue::Boolean(x) => Value::Bool(x),
            ExpressionValue::String(ref x) => Value::String(x.to_string()),
            ExpressionValue::Identifier(ref x) => data.lookup_identifier(x, position)?.clone(),
            ExpressionValue::Math(_) => Value::Number(self.eval_as_number(expression, position, data, context)?),
            ExpressionValue::Logical(_) => {
                Value::Bool(self.eval_value_as_bool(&expression.value, position, data, context)?)
            }
            ExpressionValue::FunctionCall(FunctionCall { ref name, ref args }) => {
                self.eval_function(name, args, position, data, context)?
            }
            ExpressionValue::StringConcat(StringConcat { ref values }) => {
                let mut result = String::new();

                for value in values {
                    match value {
                        ExpressionValue::String(ref x) => result.push_str(x),
                        ExpressionValue::Integer(x) => result.push_str(&format!("{}", x)),
                        ExpressionValue::Float(x) => result.push_str(&format!("{}", x)),
                        ExpressionValue::Identifier(ref x) => match data.lookup_identifier(x, position)? {
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
            result = self.eval_filter(&filter.name, &result, &filter.args, position, data, context)?;
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

    fn eval_value_as_bool(
        &self,
        value: &ExpressionValue,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<bool> {
        let result = match value {
            ExpressionValue::Integer(_) => bail!("integer can't be evaluated as bool"),
            ExpressionValue::Float(_) => bail!("float can't be evaluated as bool"),
            ExpressionValue::Boolean(x) => *x,
            ExpressionValue::String(_) => bail!("string can't be evaluated as bool"),
            ExpressionValue::Identifier(x) => match data.lookup_identifier(x, position)? {
                Value::Bool(x) => *x,
                _ => bail!("identifier does not evaluate to a boolean"),
            },
            ExpressionValue::Math(_) => bail!("math expression can't be evaluated as bool"),
            ExpressionValue::Logical(LogicalExpression {
                ref lhs,
                ref rhs,
                ref operator,
            }) => match operator {
                LogicalOperator::And => {
                    self.eval_expression_as_bool(lhs, position, data, context)?
                        && self.eval_expression_as_bool(rhs, position, data, context)?
                }
                LogicalOperator::Or => {
                    let lhs = self.eval_expression_as_bool(lhs, position, data, context)?;
                    let rhs = self.eval_expression_as_bool(rhs, position, data, context)?;
                    lhs || rhs
                }
                LogicalOperator::Equal | LogicalOperator::NotEqual => {
                    let lhs = self.eval_expression(lhs, position, data, context)?;
                    let rhs = self.eval_expression(rhs, position, data, context)?;

                    match (&lhs, &rhs) {
                        (Value::Number(ref lhs), Value::Number(ref rhs)) => {
                            if operator == &LogicalOperator::Equal {
                                lhs.relative_eq(rhs)
                            } else {
                                lhs.relative_ne(rhs)
                            }
                        }
                        _ => {
                            if operator == &LogicalOperator::Equal {
                                lhs == rhs
                            } else {
                                lhs != rhs
                            }
                        }
                    }
                }
                LogicalOperator::GreaterThan
                | LogicalOperator::GreaterThanOrEqual
                | LogicalOperator::LowerThan
                | LogicalOperator::LowerThanOrEqual => {
                    let lhs = self.eval_as_number(lhs, position, data, context)?.as_f64().unwrap();
                    let rhs = self.eval_as_number(rhs, position, data, context)?.as_f64().unwrap();

                    match operator {
                        LogicalOperator::GreaterThan => lhs > rhs,
                        LogicalOperator::GreaterThanOrEqual => lhs >= rhs,
                        LogicalOperator::LowerThan => lhs < rhs,
                        LogicalOperator::LowerThanOrEqual => lhs <= rhs,
                        _ => bail!("grammar error?"),
                    }
                }
            },
            ExpressionValue::FunctionCall(FunctionCall { ref name, ref args }) => {
                match self.eval_function(name, args, position, data, context)? {
                    Value::Bool(x) => x,
                    _ => bail!("unable to evaluate `{}` function result as bool", name),
                }
            }
            ExpressionValue::StringConcat(_) => bail!("string concatenation can't be evaluated as bool"),
        };

        Ok(result)
    }

    fn eval_expression_as_bool(
        &self,
        expression: &Expression,
        position: &Identifier,
        data: &Value,
        context: &mut Context,
    ) -> Result<bool> {
        let mut value = self.eval_value_as_bool(&expression.value, position, data, context)?;

        if expression.negated {
            value = !value;
        }

        Ok(value)
    }
}
