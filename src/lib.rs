//! [Balena](https://www.balena.io) TEMplating ENgine
//!
//! A crate that provides facilities to:
//!
//! * evaluate expressions
//! * evaluate logical expressions
//! * retrieve AST of any expression for further validation
#![allow(clippy::possible_missing_comma)]

pub use serde_json::Value;

pub use crate::{
    context::Context,
    engine::{Engine, EngineBuilder},
};

pub(crate) mod builtin;

mod context;
mod engine;
pub mod error;
mod lookup;
pub mod parser;
mod utils;
