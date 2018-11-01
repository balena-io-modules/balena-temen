use crate::{error::Result, parser::ast::*};
use lazy_static::lazy_static;
use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};
use pest_derive::Parser;
use std::collections::HashMap;

pub mod ast;

lazy_static! {
    static ref MATH_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::math_addition, Assoc::Left) | Operator::new(Rule::math_subtraction, Assoc::Left),
        Operator::new(Rule::math_multiplication, Assoc::Left)
            | Operator::new(Rule::math_division, Assoc::Left)
            | Operator::new(Rule::math_modulo, Assoc::Left),
    ]);
    static ref RELATIONAL_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::relational_lower_than, Assoc::Left)
            | Operator::new(Rule::relational_lower_than_or_equal, Assoc::Left)
            | Operator::new(Rule::relational_greater_than, Assoc::Left)
            | Operator::new(Rule::relational_greater_than_or_equal, Assoc::Left)
            | Operator::new(Rule::relational_equal, Assoc::Left)
            | Operator::new(Rule::relational_not_equal, Assoc::Left),
    ]);
    static ref LOGICAL_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::logical_and, Assoc::Left),
        Operator::new(Rule::logical_or, Assoc::Left),
    ]);
}

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct ExpressionParser;

//
// kwarg = { identifier ~ "=" ~ (logical_expression | basic_expression_filter) }
//
fn parse_kwarg(pair: Pair<Rule>) -> (String, Expression) {
    let mut name = None;
    let mut value = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::logical_expression => value = Some(parse_logical_expression(p)),
            Rule::basic_expression_filter => value = Some(parse_basic_expression_with_filters(p)),
            _ => unreachable!("parse_kwarg: {:?}", p.as_rule()),
        };
    }

    (name.unwrap(), value.unwrap())
}

//
// function_call = { identifier ~ "(" ~ kwargs? ~ ")" }
//
fn parse_function_call(pair: Pair<Rule>) -> FunctionCall {
    let mut name = None;
    let mut args = HashMap::new();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::kwarg => {
                let (name, value) = parse_kwarg(p);
                args.insert(name, value);
            }
            _ => unreachable!("parse_function_call: {:?}", p.as_rule()),
        };
    }

    FunctionCall::new(name.unwrap(), args)
}

//
// filter  = { "|" ~ (function_call | identifier) }
//
fn parse_filter(pair: Pair<Rule>) -> FunctionCall {
    let mut name = None;
    let mut args = HashMap::new();
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::kwarg => {
                let (name, value) = parse_kwarg(p);
                args.insert(name, value);
            }
            Rule::function_call => {
                return parse_function_call(p);
            }
            _ => unreachable!("parse_filter: {:?}", p.as_rule()),
        };
    }

    FunctionCall::new(name.unwrap(), args)
}

//
// basic_expression = { ("(" ~ basic_expression ~ ")" | basic_value) ~ (basic_operator ~ ("(" ~ basic_expression ~ ")" | basic_value))* }
//
fn parse_basic_expression(pair: Pair<Rule>) -> ExpressionValue {
    let primary = |pair| parse_basic_expression(pair);

    let infix = |lhs: ExpressionValue, op: Pair<Rule>, rhs: ExpressionValue| {
        let operator = match op.as_rule() {
            Rule::math_addition => MathOperator::Addition,
            Rule::math_subtraction => MathOperator::Subtraction,
            Rule::math_multiplication => MathOperator::Multiplication,
            Rule::math_division => MathOperator::Division,
            Rule::math_modulo => MathOperator::Modulo,
            _ => unreachable!("parse_basic_expression(infix): {:?}", op),
        };

        ExpressionValue::Math(MathExpression::new(
            Expression::new(lhs),
            Expression::new(rhs),
            operator,
        ))
    };

    match pair.as_rule() {
        Rule::integer => ExpressionValue::Integer(pair.as_str().parse().unwrap()),
        Rule::float => ExpressionValue::Float(pair.as_str().parse().unwrap()),
        Rule::boolean => match pair.as_str() {
            "true" => ExpressionValue::Boolean(true),
            "false" => ExpressionValue::Boolean(false),
            _ => unreachable!("parse_basic_expression(boolean): {:?}", pair.as_rule()),
        },
        Rule::function_call => ExpressionValue::FunctionCall(parse_function_call(pair)),
        Rule::string => ExpressionValue::String(remove_string_quotes(pair.as_str())),
        Rule::dotted_square_bracket_identifier => parse_dotted_square_bracket_identifier(pair),
        Rule::string_concat => parse_string_concat(pair),
        Rule::basic_expression => MATH_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("parse_basic_expression: {:?}", pair.as_rule()),
    }
}

//
// logical_value  = { logical_not? ~ comparison_expression }
//
fn parse_logical_value(pair: Pair<Rule>) -> Expression {
    let mut negated = false;
    let mut expression = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::logical_not => negated = true,
            Rule::comparison_expression => expression = Some(parse_comparison_expression(p)),
            _ => unreachable!("parse_logical_value: {:?}", p.as_rule()),
        };
    }

    if negated {
        expression.unwrap().negate()
    } else {
        expression.unwrap()
    }
}

//
// logical_expression = { logical_value ~ ((logical_or | logical_and) ~ logical_value)* }
//
fn parse_logical_expression(pair: Pair<Rule>) -> Expression {
    let primary = |pair: Pair<Rule>| parse_logical_expression(pair);

    let infix = |lhs: Expression, op: Pair<Rule>, rhs: Expression| match op.as_rule() {
        Rule::logical_or => Expression::new(ExpressionValue::Logical(LogicalExpression::new(
            lhs,
            rhs,
            LogicalOperator::Or,
        ))),
        Rule::logical_and => Expression::new(ExpressionValue::Logical(LogicalExpression::new(
            lhs,
            rhs,
            LogicalOperator::And,
        ))),
        _ => unreachable!("parse_logical_expression(infix): {:?}", op.as_rule()),
    };

    match pair.as_rule() {
        Rule::logical_value => parse_logical_value(pair),
        Rule::logical_expression => LOGICAL_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("parse_logical_expression: {:?}", pair.as_rule()),
    }
}

//
// basic_expression_filter = { basic_expression ~ filter* }
//
fn parse_basic_expression_with_filters(pair: Pair<Rule>) -> Expression {
    let mut expression = None;
    let mut filters = vec![];

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::basic_expression => expression = Some(parse_basic_expression(p)),
            Rule::filter => filters.push(parse_filter(p)),
            _ => unreachable!("parse_basic_expression_with_filters: {:?}", p),
        };
    }

    Expression::new_with_filters(expression.unwrap(), filters)
}

//
// double_quoted_string  = @{ "\"" ~ (!("\"") ~ ANY)* ~ "\""}
// single_quoted_string  = @{ "\'" ~ (!("\'") ~ ANY)* ~ "\'"}
// backquoted_quoted_string  = @{ "`" ~ (!("`") ~ ANY)* ~ "`"}
//
// string = @{ double_quoted_string | single_quoted_string | backquoted_quoted_string }
//
fn remove_string_quotes(input: &str) -> String {
    match input.chars().next().unwrap() {
        '"' => input.replace('"', "").to_string(),
        '\'' => input.replace('\'', "").to_string(),
        '`' => input.replace('`', "").to_string(),
        _ => unreachable!("remove_string_quotes: {}", input),
    }
}

//
// all_chars = _{'a'..'z' | 'A'..'Z' | "_" | '0'..'9'}
// identifier = @{
//     ('a'..'z' | 'A'..'Z' | "_") ~
//     all_chars*
// }
//
// dotted_identifier = @{
//     ('a'..'z' | 'A'..'Z' | "_") ~
//     all_chars* ~
//     ("." ~ all_chars+)*
// }
//
// square_brackets = @{
//     "[" ~ (integer | string | dotted_square_bracket_identifier) ~ "]"
// }
//
// dotted_square_bracket_identifier = @{
//     dotted_identifier ~ ( ("." ~ all_chars+) | square_brackets )*
// }
//
fn parse_dotted_square_bracket_identifier(pair: Pair<Rule>) -> ExpressionValue {
    ExpressionValue::Identifier(pair.as_str().to_string())
}

//
// string_concat = { (string | dotted_square_bracket_identifier) ~ ("~" ~ (float | integer | string | dotted_square_bracket_identifier))+ }
//
fn parse_string_concat(pair: Pair<Rule>) -> ExpressionValue {
    let values: Vec<ExpressionValue> = pair
        .into_inner()
        .map(|p| match p.as_rule() {
            Rule::string => ExpressionValue::String(remove_string_quotes(p.as_str())),
            Rule::integer => ExpressionValue::Integer(p.as_str().parse().unwrap()),
            Rule::float => ExpressionValue::Float(p.as_str().parse().unwrap()),
            Rule::dotted_square_bracket_identifier => parse_dotted_square_bracket_identifier(p),
            _ => unreachable!("parse_string_concat: {:?}", p),
        })
        .collect();

    ExpressionValue::StringConcat(StringConcat::new(values))
}

//
// comparison_value  = { basic_expression_filter ~ (basic_operator ~ basic_expression_filter)* }
//
fn parse_comparison_value(pair: Pair<Rule>) -> Expression {
    let primary = |pair| parse_comparison_value(pair);

    let infix = |lhs: Expression, op: Pair<Rule>, rhs: Expression| {
        let operator = match op.as_rule() {
            Rule::math_addition => MathOperator::Addition,
            Rule::math_subtraction => MathOperator::Subtraction,
            Rule::math_multiplication => MathOperator::Multiplication,
            Rule::math_division => MathOperator::Division,
            Rule::math_modulo => MathOperator::Modulo,
            _ => unreachable!("parse_comparison_value(infix): {:?}", op),
        };

        Expression::new(ExpressionValue::Math(MathExpression::new(lhs, rhs, operator)))
    };

    match pair.as_rule() {
        Rule::basic_expression_filter => parse_basic_expression_with_filters(pair),
        Rule::comparison_value => MATH_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("parse_comparison_value: {:?}", pair.as_rule()),
    }
}

//
// comparison_expression = { comparison_value ~ (comparison_operator ~ comparison_value)* }
//
fn parse_comparison_expression(pair: Pair<Rule>) -> Expression {
    let primary = |pair| parse_comparison_expression(pair);

    let infix = |lhs: Expression, op: Pair<Rule>, rhs: Expression| {
        let operator = match op.as_rule() {
            Rule::relational_lower_than => LogicalOperator::LowerThan,
            Rule::relational_lower_than_or_equal => LogicalOperator::LowerThanOrEqual,
            Rule::relational_greater_than => LogicalOperator::GreaterThan,
            Rule::relational_greater_than_or_equal => LogicalOperator::GreaterThanOrEqual,
            Rule::relational_not_equal => LogicalOperator::NotEqual,
            Rule::relational_equal => LogicalOperator::Equal,
            _ => unreachable!("parse_comparison_expression(infix): {:?}", op),
        };

        Expression::new(ExpressionValue::Logical(LogicalExpression::new(lhs, rhs, operator)))
    };

    match pair.as_rule() {
        Rule::comparison_value => parse_comparison_value(pair),
        Rule::comparison_expression => RELATIONAL_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("parse_comparison_expression: {:?}", pair.as_rule()),
    }
}

//
// content = {
//    logical_expression |
//    basic_expression_filter
// }
//
fn parse_content(pair: Pair<Rule>) -> Expression {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::logical_expression => parse_logical_expression(inner),
        Rule::basic_expression_filter => parse_basic_expression_with_filters(inner),
        _ => unreachable!(),
    }
}

pub fn parse(expression: &str) -> Result<Expression> {
    let mut pairs = ExpressionParser::parse(Rule::content, expression)?;
    Ok(parse_content(pairs.next().unwrap()))
}
