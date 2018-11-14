//! An expression Abstract Syntax Tree
//!
//! [The Elegant Parser] is used to parse an expression. Full [grammar].
//!
//! # Examples
//!
//! ```rust
//! use balena_temen::ast::*;
//!
//! let parsed: Expression = "1 + 2".parse().unwrap();
//! let manual = Expression::new(
//!     ExpressionValue::Math(
//!         MathExpression::new(
//!             Expression::new(ExpressionValue::Integer(1)),
//!             Expression::new(ExpressionValue::Integer(2)),
//!             MathOperator::Addition
//!         )
//!     )
//! );
//! assert_eq!(parsed, manual);
//! ```
//!
//! [The Elegant Parser]: https://github.com/pest-parser/pest
//! [grammar]: https://github.com/balena-io-modules/balena-temen/blob/master/src/parser/grammar.pest
use std::{collections::HashMap, str::FromStr};

use crate::{
    error::{bail, Error},
    parser::parse
};

/// Math operator
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MathOperator {
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// `%`
    Modulo,
}

/// Logical operator
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicalOperator {
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanOrEqual,
    /// `<`
    LowerThan,
    /// `<=`
    LowerThanOrEqual,
    /// `and`
    And,
    /// `or`
    Or,
}

/// A function call
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    /// A function name
    pub name: String,
    /// A function arguments (kwargs style, see Python)
    pub args: HashMap<String, Expression>,
}

impl FunctionCall {
    /// Creates new function call
    ///
    /// # Arguments
    ///
    /// * `name` - A function name
    /// * `args` - A function arguments (empty map allowed)
    pub fn new<S>(name: S, args: HashMap<String, Expression>) -> FunctionCall
    where
        S: Into<String>,
    {
        FunctionCall {
            name: name.into(),
            args,
        }
    }
}

/// Math expression
#[derive(Clone, Debug, PartialEq)]
pub struct MathExpression {
    /// A left-hand side
    pub lhs: Box<Expression>,
    /// A right-hand side
    pub rhs: Box<Expression>,
    /// An operator
    pub operator: MathOperator,
}

impl MathExpression {
    /// Creates new mathematical expression
    ///
    /// # Arguments
    ///
    /// * `lhs` - A left-hand side
    /// * `rhs` - A right-hand side
    /// * `operator` - An operator
    pub fn new(lhs: Expression, rhs: Expression, operator: MathOperator) -> MathExpression {
        MathExpression {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operator,
        }
    }
}

/// Logical expression
#[derive(Clone, Debug, PartialEq)]
pub struct LogicalExpression {
    /// A left-hand side
    pub lhs: Box<Expression>,
    /// A right-hand side
    pub rhs: Box<Expression>,
    /// An operator
    pub operator: LogicalOperator,
}

impl LogicalExpression {
    /// Creates new logical expression
    ///
    /// # Arguments
    ///
    /// * `lhs` - A left-hand side
    /// * `rhs` - A right-hand side
    /// * `operator` - An operator
    pub fn new(lhs: Expression, rhs: Expression, operator: LogicalOperator) -> LogicalExpression {
        LogicalExpression {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operator,
        }
    }
}

/// String concatenation
#[derive(Clone, Debug, PartialEq)]
pub struct StringConcat {
    /// List of values to concatenate
    pub values: Vec<ExpressionValue>,
}

impl StringConcat {
    /// Creates new concatenation expression
    ///
    /// # Arguments
    ///
    /// * `values` - List of values to concatenate
    pub fn new(values: Vec<ExpressionValue>) -> StringConcat {
        StringConcat { values }
    }
}

/// An identifier
///
/// # Examples
///
/// ```text
/// networks[0].name
///   |      │   |
///   |      |   └ IdentifierValue::Name("name")
///   |      |
///   |      └ IdentifierValue::IntegerIndex(0)
///   |
///   └ IdentifierValue::Name("networks")
/// ```
///
/// ```text
/// persons[boss.id]["name"]
///   |      │         |
///   |      |         └ IdentifierValue::StringIndex("name")
///   |      |
///   |      └ IdentifierValue::IdentifierIndex(boss.id)
///   |                                          |   |
///   |                                          |   └ IdentifierValue::Name("id")
///   |                                          |
///   |                                          └ IdentifierValue::Name("boss")
///   |
///   └ IdentifierValue::Name(String)
/// ```
///
/// ```text
/// this.id
///   |  |
///   |  └ IdentifierValue::Name("id")
///   |
///   └ IdentifierValue::This
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    /// List of identifier values (components)
    pub values: Vec<IdentifierValue>,
}

impl Identifier {
    /// Creates new identifier
    ///
    /// # Arguments
    ///
    /// * `values` - List of identifier values (components)
    pub fn new(values: Vec<IdentifierValue>) -> Identifier {
        Identifier { values }
    }

    /// Check if an identifier is relative
    ///
    /// An identifier is considered as a relative one if it starts with `this` or `super`
    /// keyword.
    pub fn is_relative(&self) -> bool {
        if let Some(first) = self.values.first() {
            first == &IdentifierValue::This || first == &IdentifierValue::Super
        } else {
            false
        }
    }

    /// Check if an identifier is absolute
    ///
    /// An identifier is considered as an absolute one if it does not start with `this`
    /// or `super` keyword.
    pub fn is_absolute(&self) -> bool {
        !self.is_relative()
    }
}

impl Default for Identifier {
    /// Creates new, empty, identifier
    ///
    /// This identifier can be used to refer to the root.
    fn default() -> Identifier {
        Identifier::new(vec![])
    }
}

/// An identifier value (component)
#[derive(Clone, Debug, PartialEq)]
pub enum IdentifierValue {
    /// A name (variable/property name)
    Name(String),
    /// An integer index (arrays)
    IntegerIndex(isize),
    /// A string index (dictionaries)
    StringIndex(String),
    /// An indirect index (value of another identifier)
    IdentifierIndex(Identifier),
    /// Current object
    This,
    /// Parent object
    Super,
}

/// An expression value
#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionValue {
    /// An integer
    Integer(i64),
    /// A floating point
    Float(f64),
    /// A boolean
    Boolean(bool),
    /// A string
    String(String),
    /// An identifier (variable name, array index, ...)
    Identifier(Identifier),
    /// A mathematical expression
    Math(MathExpression),
    /// A logical expression
    Logical(LogicalExpression),
    /// A function call
    FunctionCall(FunctionCall),
    /// String concatenation
    StringConcat(StringConcat),
}

/// An expression
#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    /// An expression value
    pub value: ExpressionValue,
    /// Is expression negated?
    pub negated: bool,
    /// List of filters to apply
    pub filters: Vec<FunctionCall>,
}

impl Expression {
    /// Creates new expression
    ///
    /// Expression is not negated and no filters are applied.
    ///
    /// # Arguments
    ///
    /// * `value` - An expression value
    pub fn new(value: ExpressionValue) -> Expression {
        Expression {
            value,
            negated: false,
            filters: vec![],
        }
    }

    /// Creates new negated expression
    ///
    /// Expression is negated and no filters are applied.
    ///
    /// # Arguments
    ///
    /// * `value` - An expression value
    pub fn new_negated(value: ExpressionValue) -> Expression {
        Expression {
            value,
            negated: true,
            filters: vec![],
        }
    }

    /// Creates new expression
    ///
    /// Expression is not negated and filters are applied.
    ///
    /// # Arguments
    ///
    /// * `value` - An expression value
    /// * `filters` - List of filters to apply
    pub fn new_with_filters(value: ExpressionValue, filters: Vec<FunctionCall>) -> Expression {
        Expression {
            value,
            negated: false,
            filters,
        }
    }

    /// Converts self into negated expression
    pub fn into_negated(self) -> Expression {
        Expression {
            value: self.value,
            negated: !self.negated,
            filters: self.filters,
        }
    }

    /// Returns identifier from an expression value
    pub fn identifier(&self) -> Option<&Identifier> {
        match &self.value {
            ExpressionValue::Identifier(ref identifier) => Some(identifier),
            _ => None,
        }
    }

    /// Converts self into [`Identifier`]
    ///
    /// [`Identifier`]: struct.Identifier.html
    pub fn into_identifier(self) -> Result<Identifier, Error> {
        match self.value {
            ExpressionValue::Identifier(identifier) => Ok(identifier),
            _ => bail!("expression is not an identifier"),
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Expression, Self::Err> {
        parse(s)
    }
}
