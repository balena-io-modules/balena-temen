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
    error::*,
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
///   |      └ IdentifierValue::Index(0)
///   |
///   └ IdentifierValue::Name("networks")
/// ```
///
/// ```text
/// persons[boss.id]["name"]
///   |      │         |
///   |      |         └ IdentifierValue::Name("name")
///   |      |
///   |      └ IdentifierValue::Identifier(boss.id)
///   |                                     |   |
///   |                                     |   └ IdentifierValue::Name("id")
///   |                                     |
///   |                                     └ IdentifierValue::Name("boss")
///   |
///   └ IdentifierValue::Name("persons")
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

    /// Check if an identifier is canonical
    ///
    /// An identifier is considered as canonical if all relative identifier values
    /// (`IdentifierValue::This`, `IdentifierValue::Super`) are not present.
    ///
    /// It affects (checks) nested identifiers as well.
    ///
    /// # Examples
    ///
    /// Canonical identifiers.
    ///
    /// ```rust
    /// use balena_temen::ast::*;
    ///
    /// let identifier: Identifier = "names.wifi".parse().unwrap();
    /// assert!(identifier.is_canonical());
    ///
    /// let identifier: Identifier = "names.wifi[first].id".parse().unwrap();
    /// assert!(identifier.is_canonical());
    /// ```
    ///
    /// Not canonical identifiers.
    ///
    /// ```rust
    /// use balena_temen::ast::*;
    ///
    /// let identifier: Identifier = "names.this".parse().unwrap();
    /// assert!(!identifier.is_canonical());
    ///
    /// let identifier: Identifier = "names[this.index]".parse().unwrap();
    /// assert!(!identifier.is_canonical());
    /// ```
    pub fn is_canonical(&self) -> bool {
        for v in &self.values {
            match v {
                IdentifierValue::This | IdentifierValue::Super => return false,
                IdentifierValue::Identifier(ref identifier) => {
                    if !identifier.is_canonical() {
                        return false;
                    }
                }
                _ => {}
            };
        }

        true
    }

    fn is_relative(&self) -> bool {
        if let Some(first) = self.values.first() {
            match first {
                IdentifierValue::This | IdentifierValue::Super => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Returns the canonical, absolute, identifier with all intermediate
    /// components normalized and nested identifiers canonicalized.
    ///
    /// Nested identifiers (`IdentifierValue::Identifier`) are canonicalized
    /// too.
    ///
    /// # Arguments
    ///
    /// * `position` - An identifier position
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balena_temen::ast::*;
    ///
    /// let identifier: Identifier = "names".parse().unwrap();
    /// assert_eq!(identifier.canonicalize(&Identifier::default()).unwrap(), identifier);
    ///
    /// let identifier: Identifier = "names.this.id.this.super".parse().unwrap();
    /// let canonicalized: Identifier = "names".parse().unwrap();
    /// assert_eq!(identifier.canonicalize(&Identifier::default()).unwrap(), canonicalized);
    ///
    /// let identifier: Identifier = "super.id".parse().unwrap();
    /// let position: Identifier = "wifi[`zrzka`].ssid".parse().unwrap();
    /// let canonicalized: Identifier = "wifi[`zrzka`].id".parse().unwrap();
    /// assert_eq!(identifier.canonicalize(&position).unwrap(), canonicalized);
    /// ```
    pub fn canonicalize(&self, position: &Identifier) -> Result<Identifier> {
        let position_values = if self.is_relative() {
            // Identifier is relative, we must have non empty absolute identifier
            if position.is_relative() {
                return Err(Error::with_message("unable to canonicalize identifier")
                    .context("reason", "identifier and position are relative identifiers")
                    .context("identifier", format!("{:?}", self))
                    .context("position", format!("{:?}", position)));
            }
            if position.values.is_empty() {
                return Err(Error::with_message("unable to canonicalize identifier")
                    .context("reason", "identifier is relative and position is empty")
                    .context("identifier", format!("{:?}", self))
                    .context("position", format!("{:?}", position)));
            }
            Some(position.values.iter())
        } else {
            None
        };

        let values = position_values.into_iter().flatten().chain(self.values.iter());

        let mut result = vec![];
        for value in values {
            match value {
                IdentifierValue::This => {
                    // This resolves to self, we can remove it
                }
                IdentifierValue::Super => {
                    // Super should resolve to parent, pop the latest identifier
                    // from result
                    result.pop().ok_or_else(|| {
                        Error::with_message("unable to canonicalize identifier")
                            .context("reason", "`super` can not be resolved")
                    })?;
                }
                IdentifierValue::Identifier(ref identifier) => {
                    // Canonicalize nested identifiers
                    result.push(IdentifierValue::Identifier(identifier.canonicalize(position)?));
                }
                _ => {
                    // Rest is just cloned
                    result.push(value.clone());
                }
            }
        }

        Ok(Identifier::new(result))
    }

    /// Appends `IdentifierValue::Name` to the identifier
    ///
    /// # Arguments
    ///
    /// * `name` - A name (object field, string index)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balena_temen::ast::*;
    ///
    /// let identifier = Identifier::default()
    ///     .name("wifi")
    ///     .name("ssid");
    ///
    /// let parsed = "wifi.ssid".parse().unwrap();
    ///
    /// assert_eq!(identifier, parsed);
    /// ```
    pub fn name<S>(mut self, name: S) -> Identifier
    where
        S: Into<String>,
    {
        self.values.push(IdentifierValue::Name(name.into()));
        self
    }

    /// Appends `IdentifierValue::Index` to the identifier
    ///
    /// # Arguments
    ///
    /// * `index` - An array index
    ///
    /// ```rust
    /// use balena_temen::ast::*;
    ///
    /// let identifier = Identifier::default()
    ///     .name("networks")
    ///     .index(0);
    ///
    /// let parsed = "networks[0]".parse().unwrap();
    ///
    /// assert_eq!(identifier, parsed);
    /// ```
    pub fn index(mut self, index: isize) -> Identifier {
        self.values.push(IdentifierValue::Index(index));
        self
    }

    /// Appends `IdentifierValue::Identifier` to the identifier
    ///
    /// # Arguments
    ///
    /// * `identifier` - An identifier index
    ///
    /// ```rust
    /// use balena_temen::ast::*;
    ///
    /// let identifier = Identifier::default()
    ///     .name("wifi")
    ///     .identifier(Identifier::default().name("first_wifi_id"));
    ///
    /// let parsed = "wifi[first_wifi_id]".parse().unwrap();
    ///
    /// assert_eq!(identifier, parsed);
    /// ```
    pub fn identifier(mut self, identifier: Identifier) -> Identifier {
        self.values.push(IdentifierValue::Identifier(identifier));
        self
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
    /// A string index (dictionaries)
    Name(String),
    /// An integer index (arrays)
    Index(isize),
    /// An indirect index (value of another identifier)
    Identifier(Identifier),
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
    pub fn into_identifier(self) -> Result<Identifier> {
        match self.value {
            ExpressionValue::Identifier(identifier) => Ok(identifier),
            _ => Err(Error::with_message("expression does not contain an identifier")
                .context("expression", format!("{:?}", self))),
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Expression> {
        parse(s)
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Identifier> {
        parse(s)?.into_identifier()
    }
}
