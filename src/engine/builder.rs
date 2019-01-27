use std::collections::HashMap;

use crate::{
    builtin::{
        filter::{self, FilterFn},
        function::{self, FunctionFn},
    },
    engine::Engine,
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
            .filter("UPPER", filter::upper)
            .filter("LOWER", filter::lower)
            .filter("TIME", filter::time)
            .filter("DATE", filter::date)
            .filter("DATETIME", filter::datetime)
            .filter("TRIM", filter::trim)
            .filter("SLUGIFY", filter::slugify)
            .function("UUIDV4", function::uuidv4)
            .function("NOW", function::now)
            .function("POW", function::math::pow)
            .function("LOG10", function::math::log10)
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
    ///     ast::Identifier,
    ///     Engine, EngineBuilder, Context, Value,
    ///     error::*
    /// };
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// fn text_filter(input: &Value, args: &[Value], _: &mut Context) -> Result<Value> {
    ///     let input = input.as_str()
    ///         .ok_or_else(|| {
    ///             Error::with_message("invalid input type")
    ///                 .context("expected", "string")
    ///                 .context("value", input.to_string())
    ///     })?;
    ///
    ///     let trim = args.get(0)
    ///         .unwrap_or_else(|| &Value::Bool(false));
    ///     let trim = trim
    ///         .as_bool()
    ///         .ok_or_else(|| {
    ///             Error::with_message("invalid argument type")
    ///                 .context("argument", "trim")
    ///                 .context("expected", "boolean")
    ///                 .context("value", trim.to_string())
    ///         })?;
    ///
    ///     let upper = args.get(1)
    ///         .unwrap_or_else(|| &Value::Bool(false));
    ///     let upper = upper
    ///         .as_bool()
    ///         .ok_or_else(|| {
    ///             Error::with_message("invalid argument type")
    ///                 .context("argument", "upper")
    ///                 .context("expected", "boolean")
    ///                 .context("value", trim.to_string())
    ///         })?;
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
    ///     .filter("TEXT", text_filter)
    ///     .into();
    /// let mut ctx = Context::default();
    /// let position = Identifier::default();
    /// let data = Value::Null;
    ///
    /// assert_eq!(
    ///     engine.eval("` abc ` | TEXT", &position, &data, &mut ctx).unwrap(),
    ///     json!(" abc ")
    /// );
    /// assert_eq!(
    ///     engine.eval("` abc ` | TEXT(true)", &position, &data, &mut ctx).unwrap(),
    ///     json!("abc")
    /// );
    /// assert_eq!(
    ///     engine.eval("` abc ` | TEXT(true, true)", &position, &data, &mut ctx).unwrap(),
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
        filters.insert(name.into().to_uppercase(), filter);
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
    ///     ast::Identifier,
    ///     Engine, EngineBuilder, Context, Value,
    ///     error::*
    /// };
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// fn echo_function(args: &[Value], _: &mut Context) -> Result<Value> {
    ///     let value = match args.first() {
    ///         Some(x) => {
    ///             x.as_str().ok_or_else(|| {
    ///                 Error::with_message("invalid argument type")
    ///                     .context("expect", "string")
    ///                     .context("value", x.to_string())
    ///             })?
    ///         },
    ///         None => "echo"
    ///     };
    ///
    ///     Ok(Value::String(value.to_string()))
    /// };
    ///
    /// let engine: Engine = EngineBuilder::default()
    ///     .function("ECHO", echo_function)
    ///     .into();
    /// let mut ctx = Context::default();
    /// let position = Identifier::default();
    /// let data = Value::Null;
    ///
    /// assert_eq!(
    ///     engine.eval("ECHO()", &position, &data, &mut ctx).unwrap(),
    ///     json!("echo")
    /// );
    /// assert_eq!(
    ///     engine.eval("ECHO(`Hallo`)", &position, &data, &mut ctx).unwrap(),
    ///     json!("Hallo")
    /// );
    /// assert!(
    ///     engine.eval("ECHO(1)", &position, &data, &mut ctx).is_err()
    /// );
    /// ```
    ///
    /// [`FunctionFn`]: type.FunctionFn.html
    pub fn function<S>(self, name: S, function: FunctionFn) -> EngineBuilder
    where
        S: Into<String>,
    {
        let mut functions = self.functions;
        functions.insert(name.into().to_uppercase(), function);
        EngineBuilder {
            functions,
            filters: self.filters,
            eval_keyword: self.eval_keyword,
        }
    }

    /// Registers custom evaluation keyword
    ///
    /// Defaults to `$$formula` if no keyword is registered.
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
            eval_keyword: builder.eval_keyword.unwrap_or_else(|| "$$formula".into()),
        }
    }
}
