//! [Balena](https://www.balena.io) TEMplating ENgine
//!
//! A crate that provides facilities to:
//!
//! * evaluate expressions
//! * evaluate logical expressions
//! * retrieve AST of any expression for further validation
#![allow(clippy::possible_missing_comma)]

pub(crate) mod builtin;

pub mod engine;
pub mod error;
pub mod parser;
mod utils;

// Re-Exports

pub use serde_json::Value;
