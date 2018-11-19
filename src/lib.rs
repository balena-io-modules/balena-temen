//! [balena] **tem**plating **en**gine
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
//! ## High level
//!
//! Helpers are not implemented yet. You can track the implementation progress
//! [here](https://github.com/balena-io-modules/balena-temen/issues/20).
//!
//! ## Low level
//!
//! You should not use this functionality directly unless you need some really
//! specific stuff.
//!
//! ### Expression evaluation
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
//! ### Logical expression evaluation
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
//! ### Custom functions and filters
//!
//! Visit [`EngineBuilder::function`] and [`EngineBuilder::filter`] methods
//! documentation for examples.
//!
//! ### Expression parsing
//!
//! ```rust
//! use balena_temen::ast::*;
//!
//! // Parse expression automatically
//! let parsed: Expression = "1 + 2 == 3".parse().unwrap();
//!
//! // Construct expression manually
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
//! let full_expression = Expression::new(
//!     ExpressionValue::Logical(
//!         LogicalExpression::new(
//!             lhs, rhs, LogicalOperator::Equal
//!         )
//!     )
//! );
//!
//! // Compare and check that the expression was parsed correctly
//! assert_eq!(parsed, full_expression);
//! ```
//!
//! Visit [ast] module documentation for more info.
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
    engine::{
        builder::EngineBuilder,
        Engine
    }
};

pub(crate) mod builtin;

pub mod ast;
mod context;
mod engine;
pub mod error;
mod parser;
mod utils;
