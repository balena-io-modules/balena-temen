use std::collections::HashMap;

use lazy_static::lazy_static;
use pest::{
    iterators::Pair,
    Parser,
    prec_climber::{Assoc, Operator, PrecClimber},
};
use pest_derive::Parser;

use crate::{
    error::{bail, Result},
    parser::ast::*,
    utils::validate_f64,
};

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
fn parse_kwarg(pair: Pair<Rule>) -> Result<(String, Expression)> {
    let mut name = None;
    let mut value = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => name = Some(p.into_span().as_str().to_string()),
            Rule::logical_expression => value = Some(parse_logical_expression(p)?),
            Rule::basic_expression_filter => value = Some(parse_basic_expression_with_filters(p)?),
            _ => bail!("parse_kwarg: invalid grammar? - {:?}", p.as_rule()),
        };
    }

    let n = name.ok_or_else(|| "parse_kwarg: invalid grammar? no name found")?;
    let v = value.ok_or_else(|| "parse_kwarg: invalid grammar? no value found")?;
    Ok((n, v))
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
            _ => bail!("parse_function_call: invalid grammar? {:?}", p.as_rule()),
        };
    }

    let n = name.ok_or_else(|| "parse_function_call: invalid grammar, no name found")?;
    Ok(FunctionCall::new(n, args))
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
            _ => bail!("parse_filter: invalid grammar? {:?}", p.as_rule()),
        };
    }

    let n = name.ok_or_else(|| "parse_filter: invalid grammar, no name found")?;
    Ok(FunctionCall::new(n, args))
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
            _ => bail!("parse_basic_expression(infix): invalid grammar? {:?}", op),
        };

        Ok(ExpressionValue::Math(MathExpression::new(
            Expression::new(lhs?),
            Expression::new(rhs?),
            operator,
        )))
    };

    let result = match pair.as_rule() {
        Rule::integer => ExpressionValue::Integer(pair.as_str().parse()?),
        Rule::float => ExpressionValue::Float(validate_f64(pair.as_str().parse()?)?),
        Rule::boolean => match pair.as_str() {
            "true" => ExpressionValue::Boolean(true),
            "false" => ExpressionValue::Boolean(false),
            _ => bail!("parse_basic_expression(boolean): invalid grammar? {:?}", pair.as_rule()),
        },
        Rule::function_call => ExpressionValue::FunctionCall(parse_function_call(pair)?),
        Rule::string => ExpressionValue::String(remove_string_quotes(pair.as_str())?),
        Rule::dotted_square_bracket_identifier => parse_dotted_square_bracket_identifier(pair)?,
        Rule::string_concat => parse_string_concat(pair)?,
        Rule::basic_expression => MATH_CLIMBER.climb(pair.into_inner(), primary, infix)?,
        _ => bail!("parse_basic_expression: invalid grammer? {:?}", pair.as_rule()),
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
            _ => bail!("parse_logical_value: invalid grammar? {:?}", p.as_rule()),
        };
    }

    let exp = expression.ok_or_else(|| "parse_logical_value: invalid grammar, unable to create expression")?;
    if negated {
        Ok(exp.negate())
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
            _ => bail!("parse_logical_expression(infix): invalid grammar? {:?}", op.as_rule()),
        };
        Ok(result)
    };

    match pair.as_rule() {
        Rule::logical_value => parse_logical_value(pair),
        Rule::logical_expression => LOGICAL_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => bail!("parse_logical_expression: invalid grammar? {:?}", pair.as_rule()),
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
            _ => bail!("parse_basic_expression_with_filters: invalid grammar? {:?}", p),
        };
    }

    let exp = expression
        .ok_or_else(|| "parse_basic_expression_with_filters: invalid grammar, unable to create expression")?;
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
    let result = match input
        .chars()
        .next()
        .ok_or_else(|| "remove_string_quotes: invalid grammar, unable to remove quotes")?
    {
        '"' => input.replace('"', "").to_string(),
        '\'' => input.replace('\'', "").to_string(),
        '`' => input.replace('`', "").to_string(),
        _ => bail!("remove_string_quotes: invalid grammar? {}", input),
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
            Rule::string => IdentifierValue::StringIndex(remove_string_quotes(p.as_str())?),
            Rule::integer | Rule::positive_integer => IdentifierValue::IntegerIndex(p.as_str().parse()?),
            Rule::dotted_square_bracket_identifier => {
                IdentifierValue::IdentifierIndex(parse_dotted_square_bracket_identifier_value(p)?)
            }
            _ => bail!("parse_dotted_square_bracket_identifier_value: invalid grammar?"),
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
            Rule::integer => ExpressionValue::Integer(p.as_str().parse()?),
            Rule::float => ExpressionValue::Float(validate_f64(p.as_str().parse()?)?),
            Rule::dotted_square_bracket_identifier => parse_dotted_square_bracket_identifier(p)?,
            _ => bail!("parse_string_concat: invalid grammar?"),
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
            _ => bail!("parse_comparison_value(infix): invalid grammar? {:?}", op),
        };

        Ok(Expression::new(ExpressionValue::Math(MathExpression::new(
            lhs?, rhs?, operator,
        ))))
    };

    match pair.as_rule() {
        Rule::basic_expression_filter => parse_basic_expression_with_filters(pair),
        Rule::comparison_value => MATH_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => bail!("parse_comparison_value: invalid grammar? {:?}", pair.as_rule()),
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
            _ => bail!("parse_comparison_expression(infix): invalid grammar? {:?}", op),
        };

        Ok(Expression::new(ExpressionValue::Logical(LogicalExpression::new(
            lhs?, rhs?, operator,
        ))))
    };

    match pair.as_rule() {
        Rule::comparison_value => parse_comparison_value(pair),
        Rule::comparison_expression => RELATIONAL_CLIMBER.climb(pair.into_inner(), primary, infix),
        _ => bail!("parse_comparison_expression: invalid grammar? {:?}", pair.as_rule()),
    }
}

//
// content = {
//    logical_expression |
//    basic_expression_filter
// }
//
fn parse_content(pair: Pair<Rule>) -> Result<Expression> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| "parse_content: invalid grammar, no inner pair?")?;

    match inner.as_rule() {
        Rule::logical_expression => parse_logical_expression(inner),
        Rule::basic_expression_filter => parse_basic_expression_with_filters(inner),
        _ => bail!("parse_content: invalid grammar?"),
    }
}

pub(crate) fn parse(expression: &str) -> Result<Expression> {
    let mut pairs = ExpressionParser::parse(Rule::content, expression)?;
    parse_content(pairs.next().ok_or_else(|| "parse: invalid grammar, no pair?")?)
}
