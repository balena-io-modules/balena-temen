use std::collections::HashMap;

use lazy_static::lazy_static;
use pest::{
    iterators::Pair,
    Parser,
    prec_climber::{Assoc, Operator, PrecClimber},
};
use pest_derive::Parser;

use crate::{
    ast::*,
    error::*,
    utils::validate_f64,
};

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
fn parse_kwarg(pair: Pair<Rule>) -> Result<(String, Expression)> {
    let mut name = None;
    let mut value = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::logical_expression => value = Some(parse_logical_expression(p)?),
            Rule::basic_expression_filter => value = Some(parse_basic_expression_with_filters(p)?),
            _ => unreachable!("invalid grammar: {}", p.to_string()),
        };
    }

    Ok((
        name.expect("invalid grammar: no kwarg name"),
        value.expect("invalid grammar: no kwarg value"),
    ))
}

//
// function_call = { identifier ~ "(" ~ kwargs? ~ ")" }
//
fn parse_function_call(pair: Pair<Rule>) -> Result<FunctionCall> {
    let mut name = None;
    let mut args = HashMap::new();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::kwarg => {
                let (name, value) = parse_kwarg(p)?;
                args.insert(name, value);
            }
            _ => unreachable!("invalid grammar: {}", p.as_str()),
        };
    }

    Ok(FunctionCall::new(
        name.expect("invalid grammar: no function name"),
        args,
    ))
}

//
// filter  = { "|" ~ (function_call | identifier) }
//
fn parse_filter(pair: Pair<Rule>) -> Result<FunctionCall> {
    let mut name = None;
    let mut args = HashMap::new();
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::kwarg => {
                let (name, value) = parse_kwarg(p)?;
                args.insert(name, value);
            }
            Rule::function_call => {
                return parse_function_call(p);
            }
            _ => unreachable!("invalid grammar: {}", p.as_str()),
        };
    }

    Ok(FunctionCall::new(name.unwrap(), args))
}

//
// basic_expression = { ("(" ~ basic_expression ~ ")" | basic_value) ~ (basic_operator ~ ("(" ~ basic_expression ~ ")" | basic_value))* }
//
fn parse_basic_expression(pair: Pair<Rule>) -> Result<ExpressionValue> {
    let primary = |pair| parse_basic_expression(pair);

    let infix = |lhs: Result<ExpressionValue>, op: Pair<Rule>, rhs: Result<ExpressionValue>| {
        let operator = match op.as_rule() {
            Rule::math_addition => MathOperator::Addition,
            Rule::math_subtraction => MathOperator::Subtraction,
            Rule::math_multiplication => MathOperator::Multiplication,
            Rule::math_division => MathOperator::Division,
            Rule::math_modulo => MathOperator::Modulo,
            _ => unreachable!("invalid grammar: {}", op.as_str()),
        };

        Ok(ExpressionValue::Math(MathExpression::new(
            Expression::new(lhs?),
            Expression::new(rhs?),
            operator,
        )))
    };

    let result = match pair.as_rule() {
        Rule::integer => ExpressionValue::Integer(
            pair.as_str()
                .parse()
                .map_err(|_| Error::with_message("unable to parse i64").context("value", pair.to_string()))?,
        ),
        Rule::float => {
            ExpressionValue::Float(validate_f64(pair.as_str().parse().map_err(|_| {
                Error::with_message("unable to parse f64").context("value", pair.to_string())
            })?)?)
        }
        Rule::boolean => match pair.as_str() {
            "true" => ExpressionValue::Boolean(true),
            "false" => ExpressionValue::Boolean(false),
            _ => unreachable!("invalid grammar: {}", pair.as_str()),
        },
        Rule::function_call => ExpressionValue::FunctionCall(parse_function_call(pair)?),
        Rule::string => ExpressionValue::String(remove_string_quotes(pair.as_str())?),
        Rule::dotted_square_bracket_identifier => parse_dotted_square_bracket_identifier(pair)?,
        Rule::string_concat => parse_string_concat(pair)?,
        Rule::basic_expression => MATH_CLIMBER.climb(pair.into_inner(), primary, infix)?,
        _ => unreachable!("invalid grammar: {}", pair.as_str()),
    };

    Ok(result)
}

//
// logical_value  = { logical_not? ~ comparison_expression }
//
fn parse_logical_value(pair: Pair<Rule>) -> Result<Expression> {
    let mut negated = false;
    let mut expression = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::logical_not => negated = true,
            Rule::comparison_expression => expression = Some(parse_comparison_expression(p)?),
            _ => unreachable!("invalid grammar: {}", p.as_str()),
        };
    }

    let exp = expression.expect("invalid grammar: no expression");
    if negated {
        Ok(exp.into_negated())
    } else {
        Ok(exp)
    }
}

//
// logical_expression = { logical_value ~ ((logical_or | logical_and) ~ logical_value)* }
//
fn parse_logical_expression(pair: Pair<Rule>) -> Result<Expression> {
    let primary = |pair: Pair<Rule>| parse_logical_expression(pair);

    let infix = |lhs: Result<Expression>, op: Pair<Rule>, rhs: Result<Expression>| {
        let result = match op.as_rule() {
            Rule::logical_or => Expression::new(ExpressionValue::Logical(LogicalExpression::new(
                lhs?,
                rhs?,
                LogicalOperator::Or,
            ))),
            Rule::logical_and => Expression::new(ExpressionValue::Logical(LogicalExpression::new(
                lhs?,
                rhs?,
                LogicalOperator::And,
            ))),
            _ => unreachable!("invalid grammar"),
        };
        Ok(result)
    };

    match pair.as_rule() {
        Rule::logical_value => parse_logical_value(pair),
        Rule::logical_expression => LOGICAL_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("invalid grammar"),
    }
}

//
// basic_expression_filter = { basic_expression ~ filter* }
//
fn parse_basic_expression_with_filters(pair: Pair<Rule>) -> Result<Expression> {
    let mut expression = None;
    let mut filters = vec![];

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::basic_expression => expression = Some(parse_basic_expression(p)?),
            Rule::filter => filters.push(parse_filter(p)?),
            _ => unreachable!("invalid grammar"),
        };
    }

    let exp = expression.expect("invalid grammar: no expression");
    Ok(Expression::new_with_filters(exp, filters))
}

//
// double_quoted_string  = @{ "\"" ~ (!("\"") ~ ANY)* ~ "\""}
// single_quoted_string  = @{ "\'" ~ (!("\'") ~ ANY)* ~ "\'"}
// backquoted_quoted_string  = @{ "`" ~ (!("`") ~ ANY)* ~ "`"}
//
// string = @{ double_quoted_string | single_quoted_string | backquoted_quoted_string }
//
fn remove_string_quotes(input: &str) -> Result<String> {
    let result = match input.chars().next().expect("invalid grammar: no string quotes") {
        '"' => input.replace('"', "").to_string(),
        '\'' => input.replace('\'', "").to_string(),
        '`' => input.replace('`', "").to_string(),
        _ => unreachable!("invalid grammar"),
    };
    Ok(result)
}

// all_chars = _{'a'..'z' | 'A'..'Z' | "_" | '0'..'9'}
// identifier = @{
//     ('a'..'z' | 'A'..'Z' | "_") ~
//     all_chars*
// }
//
// square_brackets = _{
//     "[" ~ (integer | string | dotted_square_bracket_identifier) ~ "]"
// }
//
// dotted_square_bracket_identifier = ${
//     identifier ~ ( ("." ~ ( identifier | positive_integer ) ) | square_brackets )*
// }
//
fn parse_dotted_square_bracket_identifier_value(pair: Pair<Rule>) -> Result<Identifier> {
    let mut values = Vec::new();

    for p in pair.into_inner() {
        let value = match p.as_rule() {
            Rule::identifier => match p.as_str() {
                "this" => IdentifierValue::This,
                "super" => IdentifierValue::Super,
                _ => IdentifierValue::Name(p.as_str().to_string()),
            },
            Rule::string => IdentifierValue::Name(remove_string_quotes(p.as_str())?),
            Rule::integer | Rule::positive_integer => IdentifierValue::Index(
                p.as_str()
                    .parse()
                    .map_err(|_| Error::with_message("unable to parse i64").context("value", p.to_string()))?,
            ),
            Rule::dotted_square_bracket_identifier => {
                IdentifierValue::Identifier(parse_dotted_square_bracket_identifier_value(p)?)
            }
            _ => unreachable!("invalid grammar"),
        };
        values.push(value);
    }

    Ok(Identifier::new(values))
}

fn parse_dotted_square_bracket_identifier(pair: Pair<Rule>) -> Result<ExpressionValue> {
    Ok(ExpressionValue::Identifier(
        parse_dotted_square_bracket_identifier_value(pair)?,
    ))
}

//
// string_concat = { (string | dotted_square_bracket_identifier) ~ ("~" ~ (float | integer | string | dotted_square_bracket_identifier))+ }
//
fn parse_string_concat(pair: Pair<Rule>) -> Result<ExpressionValue> {
    let mut values = Vec::new();

    for p in pair.into_inner() {
        let result = match p.as_rule() {
            Rule::string => ExpressionValue::String(remove_string_quotes(p.as_str())?),
            Rule::integer => ExpressionValue::Integer(
                p.as_str()
                    .parse()
                    .map_err(|_| Error::with_message("unable to parse i64").context("value", p.to_string()))?,
            ),
            Rule::float => {
                ExpressionValue::Float(validate_f64(p.as_str().parse().map_err(|_| {
                    Error::with_message("unable to parse f64").context("value", p.to_string())
                })?)?)
            }
            Rule::dotted_square_bracket_identifier => parse_dotted_square_bracket_identifier(p)?,
            _ => unreachable!("invalid grammar"),
        };
        values.push(result);
    }

    Ok(ExpressionValue::StringConcat(StringConcat::new(values)))
}

//
// comparison_value  = { basic_expression_filter ~ (basic_operator ~ basic_expression_filter)* }
//
fn parse_comparison_value(pair: Pair<Rule>) -> Result<Expression> {
    let primary = |pair| parse_comparison_value(pair);

    let infix = |lhs: Result<Expression>, op: Pair<Rule>, rhs: Result<Expression>| {
        let operator = match op.as_rule() {
            Rule::math_addition => MathOperator::Addition,
            Rule::math_subtraction => MathOperator::Subtraction,
            Rule::math_multiplication => MathOperator::Multiplication,
            Rule::math_division => MathOperator::Division,
            Rule::math_modulo => MathOperator::Modulo,
            _ => unreachable!("invalid grammar"),
        };

        Ok(Expression::new(ExpressionValue::Math(MathExpression::new(
            lhs?, rhs?, operator,
        ))))
    };

    match pair.as_rule() {
        Rule::basic_expression_filter => parse_basic_expression_with_filters(pair),
        Rule::comparison_value => MATH_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("invalid grammar"),
    }
}

//
// comparison_expression = { comparison_value ~ (comparison_operator ~ comparison_value)* }
//
fn parse_comparison_expression(pair: Pair<Rule>) -> Result<Expression> {
    let primary = |pair| parse_comparison_expression(pair);

    let infix = |lhs: Result<Expression>, op: Pair<Rule>, rhs: Result<Expression>| {
        let operator = match op.as_rule() {
            Rule::relational_lower_than => LogicalOperator::LowerThan,
            Rule::relational_lower_than_or_equal => LogicalOperator::LowerThanOrEqual,
            Rule::relational_greater_than => LogicalOperator::GreaterThan,
            Rule::relational_greater_than_or_equal => LogicalOperator::GreaterThanOrEqual,
            Rule::relational_not_equal => LogicalOperator::NotEqual,
            Rule::relational_equal => LogicalOperator::Equal,
            _ => unreachable!("invalid grammar"),
        };

        Ok(Expression::new(ExpressionValue::Logical(LogicalExpression::new(
            lhs?, rhs?, operator,
        ))))
    };

    match pair.as_rule() {
        Rule::comparison_value => parse_comparison_value(pair),
        Rule::comparison_expression => RELATIONAL_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => unreachable!("invalid grammar"),
    }
}

//
// content = {
//    logical_expression |
//    basic_expression_filter
// }
//
fn parse_content(pair: Pair<Rule>) -> Result<Expression> {
    let inner = pair.into_inner().next().expect("invalid grammar");

    match inner.as_rule() {
        Rule::logical_expression => parse_logical_expression(inner),
        Rule::basic_expression_filter => parse_basic_expression_with_filters(inner),
        _ => unreachable!("invalid grammar"),
    }
}

pub(crate) fn parse(expression: &str) -> Result<Expression> {
    let mut pairs = ExpressionParser::parse(Rule::content, expression)
        .map_err(|_| Error::with_message("unable to parse expression").context("expression", expression.to_string()))?;
    let next = pairs.next().ok_or_else(|| {
        Error::with_message("unable to parse expression").context("expression", expression.to_string())
    })?;
    parse_content(next)
}
