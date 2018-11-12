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
    lookup::Lookup,
};

pub(crate) mod builtin;

pub mod ast;
mod context;
mod engine;
pub mod error;
mod lookup;
mod parser;
mod utils;
