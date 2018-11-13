//! [balena] templating engine
//!
//! A crate that provides facilities to:
//!
//! * parse an expression
//! * evaluate an expression
//! * evaluate an expression as a boolean (logical expressions)
//! * register custom functions and filters
//!
//! # Versioning
//!
//! This crate is being actively developed and it does NOT follow [Semantic Versioning] yet.
//! It will follow semantic versioning when it reaches version 1.0.
//!
//! MINOR version changes denotes incompatible API changes and PATCH version changes denotes
//! both new functionality in a backwards-compatible manner and backwards-compatible bug fixes.
//!
//! # Examples
//!
//! ## Expression parsing
//!
//! ```rust
//! use balena_temen::ast::*;
//!
//! let parsed: Expression = "1 + 2 == 3".parse().unwrap();
//!
//! let lhs = Expression::new(
//!     ExpressionValue::Math(
//!         MathExpression::new(
//!             Expression::new(ExpressionValue::Integer(1)),
//!             Expression::new(ExpressionValue::Integer(2)),
//!             MathOperator::Addition
//!         )
//!     )
//! );
//! let rhs = Expression::new(
//!     ExpressionValue::Integer(3)
//! );
//! let manual = Expression::new(
//!     ExpressionValue::Logical(
//!         LogicalExpression::new(
//!             lhs, rhs, LogicalOperator::Equal
//!         )
//!     )
//! );
//! assert_eq!(parsed, manual);
//! ```
//!
//! Visit [ast] module documentation for more info.
//!
//! ## Expression evaluation
//!
//! ```rust
//! use balena_temen::{
//!   ast::Identifier,
//!   Engine, Context, Value
//! };
//! use serde_json::json;
//!
//! let engine = Engine::default();       // Default functions, filters
//! let mut ctx = Context::default();     // Default context
//! let position = Identifier::default(); // Evaluate from the root
//! let data = Value::Null;
//!
//! assert_eq!(
//!     engine.eval("2 + 3", &position, &data, &mut ctx).unwrap(),
//!     json!(5)
//! );
//!
//! assert_eq!(
//!     engine.eval("`Balena templating engine!` | slugify", &position, &data, &mut ctx).unwrap(),
//!     json!("balena-templating-engine")
//! );
//! ```
//!
//! ## Logical expression evaluation
//!
//! ```rust
//! use balena_temen::{
//!   ast::Identifier,
//!   Engine, Context, Value
//! };
//! use serde_json::json;
//!
//! let engine = Engine::default();       // Default functions, filters
//! let mut ctx = Context::default();     // Default context
//! let position = Identifier::default(); // Evaluate from the root
//! let data = Value::Null;
//!
//! assert_eq!(
//!     engine.eval_as_bool("1 == 1", &position, &data, &mut ctx).unwrap(),
//!     json!(true)
//! );
//!
//! assert_eq!(
//!     engine.eval_as_bool("2 > 4", &position, &data, &mut ctx).unwrap(),
//!     json!(false)
//! );
//!
//! assert_eq!(
//!     engine.eval_as_bool("1 > 4 or 3 > 2", &position, &data, &mut ctx).unwrap(),
//!     json!(true)
//! );
//! ```
//!
//! ## Custom functions and filters
//!
//! Visit [`EngineBuilder::function`] and [`EngineBuilder::filter`] methods
//! documentation for examples.
//!
//! [balena]: https://www.balena.io
//! [ast]: ast/index.html
//! [`EngineBuilder::function`]: struct.EngineBuilder.html#method.function
//! [`EngineBuilder::filter`]: struct.EngineBuilder.html#method.filter
//! [Semantic Versioning]: https://semver.org/
#![allow(clippy::possible_missing_comma)]

pub use serde_json::Value;

pub use crate::{
    builtin::{
        filter::FilterFn,
        function::FunctionFn,
    },
    context::Context,
    engine::{Engine, EngineBuilder}
};

pub(crate) mod builtin;

pub mod ast;
mod context;
mod engine;
pub mod error;
mod lookup;
mod parser;
mod utils;
