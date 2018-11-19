use serde_json::Value;

use crate::ast::*;
use crate::context::Context;
use crate::engine::Engine;
use crate::error::*;

/// Item to evaluate
struct Item {
    /// Item position
    position: Identifier,
    /// Item expression (`$$eval` value)
    expression: String,
}

/// Creates an item to evaluate if applicable
///
/// `value` must be an object containing the `$$eval` keyword and value of this
/// keyword must be a `String`.
///
/// # Arguments
///
/// * `value` - A JSON value to create item from
/// * `position` - A JSON value position
/// * `keyword` - An evaluation keyword
fn item_to_eval(value: &Value, position: &Identifier, keyword: &str) -> Result<Option<Item>> {
    match value {
        Value::Object(ref object) => {
            if let Some(ref value) = object.get(keyword) {
                // Object with $$eval keyword, must be a string
                let expression = value.as_str().ok_or_else(|| {
                    Error::with_message("unable to evaluate")
                        .context("reason", "eval keyword value is not a string")
                        .context("value", value.to_string())
                        .context("position", format!("{:?}", position))
                })?;
                Ok(Some(Item {
                    position: position.clone(),
                    expression: expression.to_string(),
                }))
            } else {
                // Object, but not $$eval keyword
                Ok(None)
            }
        }
        _ => {
            // Not an object, nothing to evaluate
            Ok(None)
        }
    }
}

/// Creates list of items to evaluate
///
/// It traverses the whole JSON recuresively.
///
/// # Arguments
///
/// * `value` - A value to traverse
/// * `position` - Current value position
/// * `keyword` - An evaluation keyword
fn items_to_eval(value: &Value, position: Identifier, keyword: &str) -> Result<Option<Vec<Item>>> {
    match value {
        Value::Null | Value::String(_) | Value::Number(_) | Value::Bool(_) => {
            // There's nothing to evaluate
            Ok(None)
        }
        Value::Array(ref array) => {
            // We have to check if this array contains objects to evaluate
            let mut result = vec![];

            for (idx, value) in array.iter().enumerate() {
                match items_to_eval(value, position.clone().index(idx as isize), keyword)? {
                    Some(items) => result.extend(items),
                    None => {}
                };
            }

            if result.len() > 0 {
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }
        Value::Object(ref object) => match item_to_eval(value, &position, keyword)? {
            Some(item) => {
                // Object contains $$eval and value is a string
                Ok(Some(vec![item]))
            }
            None => {
                // Object does not contain $$eval, check object key/value pairs recursively
                let mut result = vec![];

                for (k, v) in object {
                    match items_to_eval(v, position.clone().name(k.to_string()), keyword)? {
                        Some(items) => result.extend(items),
                        None => {}
                    };
                }

                if result.len() > 0 {
                    Ok(Some(result))
                } else {
                    Ok(None)
                }
            }
        },
    }
}

/// Replaces value in a JSON
///
/// # Arguments
///
/// * `data` - A JSON
/// * `new_value` - New value to use
/// * `position` - A position of the new value
fn replace_value(data: Value, new_value: Value, position: &Identifier) -> Value {
    if position.values.is_empty() {
        // Empty position = root = whole JSON
        return new_value;
    }

    let mut data = data;
    let mut current = &mut data;
    for value in &position.values {
        match value {
            // .unwrap()'s are safe - position was constructed by us
            IdentifierValue::Name(ref name) => current = current.get_mut(name).unwrap(),
            IdentifierValue::Index(ref index) => current = current.get_mut(*index as usize).unwrap(),
            _ => unreachable!(),
        }
    }

    *current = new_value;
    data
}

// This is pretty naive, multi pass evaluation. It works in this way:
//
//   * evaluate all items, one by one,
//   * do not fail if it fails, just increase the counters
//   * nothing failed? return what we have, success
//   * at least one item failed to evaluate?
//     * no item succeeded? return an error
//   * try again with another pass
//
// It's good enough for now.
//
// We will see what kind of DSLs we will have and if we will need to create
// dependency tree, detect circular dependencies, analyze if we can evaluate
// before the actual evaluation, etc.
fn eval_with_items(data: Value, items: Vec<Item>, engine: &Engine, context: &mut Context) -> Result<Value> {
    let mut fail_counter;
    let mut success_counter;
    let mut data = data;

    loop {
        fail_counter = 0;
        success_counter = 0;

        for item in &items {
            match engine.eval(&item.expression, &item.position, &data, context) {
                Err(_) => fail_counter += 1,
                Ok(new_value) => {
                    data = replace_value(data, new_value, &item.position);
                    success_counter += 1;
                }
            };
        }

        if fail_counter == 0 {
            // Nothing failed, return what we have
            return Ok(data);
        }

        if fail_counter > 0 && success_counter == 0 {
            // Something failed, but not even one item was evaluated, another pass won't help, fail
            return Err(Error::with_message("unable to evaluate"));
        }

        // Something failed here, but also at least one item was evaluated. Try
        // another pass to check if we can evaluate more.
    }
}

/// Evaluates the whole JSON
///
/// # Arguments
///
/// * `data` - A JSON to evaluate
///
/// # Examples
///
/// An object evaluation.
///
/// ```rust
/// use balena_temen::{eval, Value};
/// use serde_json::json;
///
/// let data = json!({
///   "$$eval": "1 + 2"
/// });
///
/// assert_eq!(eval(data).unwrap(), json!(3));
/// ```
///
/// Chained dependencies evaluation.
///
/// ```rust
/// use balena_temen::{eval, Value};
/// use serde_json::json;
///
/// let data = json!({
///     "ssid": "Zrzka 5G",
///     "id": {
///         "$$eval": "super.ssid | slugify"
///     },
///     "upperId": {
///         "$$eval": "super.id | upper"
///     }
/// });
///
/// let evaluated = json!({
///     "ssid": "Zrzka 5G",
///     "id": "zrzka-5g",
///     "upperId": "ZRZKA-5G"
/// });
///
/// assert_eq!(eval(data).unwrap(), evaluated);
/// ```
pub fn eval(data: Value) -> Result<Value> {
    let engine = Engine::default();
    let mut context = Context::default();

    match items_to_eval(&data, Identifier::default(), engine.eval_keyword())? {
        Some(items) => eval_with_items(data, items, &engine, &mut context),
        None => Ok(data),
    }
}

/// Evaluates the whole JSON with custom [`Engine`]
///
/// # Arguments
///
/// * `data` - A JSON to evaluate
///
/// # Examples
///
/// ```rust
/// use balena_temen::{Context, eval_with_engine, Engine, EngineBuilder, Value};
/// use serde_json::json;
///
/// let mut context = Context::default();
/// let engine: Engine = EngineBuilder::default()
///     .eval_keyword("evalMePlease")
///     .into();
///
/// let data = json!({
///   "evalMePlease": "1 + 2"
/// });
///
/// assert_eq!(eval_with_engine(data, &engine, &mut context).unwrap(), json!(3));
/// ```
///
/// Check the [`eval`] function for more examples.
///
/// [`eval`]: fn.eval.html
/// [`Engine`]: struct.Engine.html
pub fn eval_with_engine(data: Value, engine: &Engine, context: &mut Context) -> Result<Value> {
    match items_to_eval(&data, Identifier::default(), engine.eval_keyword())? {
        Some(items) => eval_with_items(data, items, engine, context),
        None => Ok(data),
    }
}
