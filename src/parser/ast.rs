//! AST = Abstract Syntax Tree.
//!
//! AST is considered as a part of the public API and follows semantic Versioning.
use crate::{error::Error, parser::parse};
use std::{collections::HashMap, str::FromStr};

/// Mathematical operator
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MathOperator {
    /// +
    Addition,
    /// -
    Subtraction,
    /// *
    Multiplication,
    /// /
    Division,
    /// %
    Modulo,
}

/// Logical operator
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicalOperator {
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// >
    GreaterThan,
    /// >=
    GreaterThanOrEqual,
    /// <
    LowerThan,
    /// <=
    LowerThanOrEqual,
    /// and
    And,
    /// or
    Or,
}

/// Function call
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    /// Function name
    pub name: String,
    /// Function arguments (kwargs style, see Python)
    pub args: HashMap<String, Expression>,
}

impl FunctionCall {
    /// Create new function call
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

/// Mathematical expression
#[derive(Clone, Debug, PartialEq)]
pub struct MathExpression {
    /// Left side
    pub lhs: Box<Expression>,
    /// Right side
    pub rhs: Box<Expression>,
    /// Operator
    pub operator: MathOperator,
}

impl MathExpression {
    /// Create new mathematical expression
    ///
    /// # Arguments
    ///
    /// * `lhs` - Left side
    /// * `rhs` - Right side
    /// * `operator` - Operator
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
    /// Left side
    pub lhs: Box<Expression>,
    /// Right side
    pub rhs: Box<Expression>,
    /// Operator
    pub operator: LogicalOperator,
}

impl LogicalExpression {
    /// Create new logical expression
    ///
    /// # Arguments
    ///
    /// * `lhs` - Left side
    /// * `rhs` - Right side
    /// * `operator` - Operator
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
    /// Values to concatenate
    pub values: Vec<ExpressionValue>,
}

impl StringConcat {
    /// Create new concatenation
    ///
    /// # Arguments
    ///
    /// * `values` - List of values to concatenate
    pub fn new(values: Vec<ExpressionValue>) -> StringConcat {
        StringConcat { values }
    }
}

/// Identifier
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub values: Vec<IdentifierValue>,
}

impl Identifier {
    /// Create new identifier
    ///
    /// # Arguments
    ///
    /// * `values` - List of identifier values
    pub fn new(values: Vec<IdentifierValue>) -> Identifier {
        Identifier { values }
    }
}

/// Identifier value
#[derive(Clone, Debug, PartialEq)]
pub enum IdentifierValue {
    /// Name (variable/property name)
    Name(String),
    /// Integer index (arrays)
    IntegerIndex(isize),
    /// String index (dictionaries)
    StringIndex(String),
    /// Indirect index (value of another identifier)
    IdentifierIndex(Identifier),
}

/// Expression value
#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionValue {
    /// Integer
    Integer(i64),
    /// Floating point
    Float(f64),
    /// Boolean
    Boolean(bool),
    /// String
    String(String),
    /// Identifier (variable name, array index, ...)
    Identifier(Identifier),
    /// Mathematical expression
    Math(MathExpression),
    /// Logical expression
    Logical(LogicalExpression),
    /// Function call
    FunctionCall(FunctionCall),
    /// String concatenation
    StringConcat(StringConcat),
}

/// Expression
#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    /// Expression value
    pub value: ExpressionValue,
    /// Is expression negated?
    pub negated: bool,
    /// Filters to apply
    pub filters: Vec<FunctionCall>,
}

impl Expression {
    /// Create new expression
    ///
    /// Expression is not negated and no filters are applied.
    ///
    /// # Arguments
    ///
    /// * `value` - Expression value
    pub fn new(value: ExpressionValue) -> Expression {
        Expression {
            value,
            negated: false,
            filters: vec![],
        }
    }

    /// Create new negated expression
    ///
    /// Expression is negated and no filters are applied.
    ///
    /// # Arguments
    ///
    /// * `value` - Expression value
    pub fn new_negated(value: ExpressionValue) -> Expression {
        Expression {
            value,
            negated: true,
            filters: vec![],
        }
    }

    /// Create new expression
    ///
    /// Expression is not negated and filters are applied.
    ///
    /// # Arguments
    ///
    /// * `value` - Expression value
    /// * `filters` - List of filters to apply
    pub fn new_with_filters(value: ExpressionValue, filters: Vec<FunctionCall>) -> Expression {
        Expression {
            value,
            negated: false,
            filters,
        }
    }

    /// Negate expression (`self` is consumed)
    pub fn negate(self) -> Expression {
        Expression {
            value: self.value,
            negated: !self.negated,
            filters: self.filters,
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}
