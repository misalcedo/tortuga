//! Parse a sequence of tokens into a syntax tree.
//!
//! Relies on a Parser Expression Grammar to generate a parser for the language.
//!
//! See <https://en.wikipedia.org/wiki/Parsing_expression_grammar>

use crate::grammar::syntax::*;
use crate::SyntacticalError;

use pest::iterators::Pairs;
use pest::Parser as PEG;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// A Parser Expression Grammar parser that is auto-generated.
///
/// ## Examples
///
/// ### Program
/// ```rust
/// use tortuga::Program;
///
/// let program: Program = "1 + 2".parse().unwrap();
///
/// assert_eq!(program, Program::default());
/// ```
///
/// ### Continuation
/// /// ```rust
/// use tortuga::Continuation;
///
/// let continuation: Continuation = "1 + 2 <= 5".parse().unwrap();
///
/// assert_eq!(continuation, Continuation::default());
/// ```
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parser;

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn new_program(mut pairs: Pairs<Rule>) -> Result<Program, SyntacticalError> {
    let mut program = Program::default();

    while pairs.peek().is_some() {
        program.0.push(new_expression(&mut pairs)?);
    }

    Ok(program)
}

fn new_operation(pairs: &mut Pairs<Rule>) -> Result<Expression, SyntacticalError> {
    let mut lhs = new_expression(pairs)?;

    while pairs.peek().is_some() {
        let operator = new_operator(pairs)?;
        let rhs = new_expression(pairs)?;

        lhs = Expression::Operation(Box::new(Operation {
            lhs,
            operator,
            rhs
        }));
    }

    Ok(lhs)
}

fn new_comparison(pairs: &mut Pairs<Rule>) -> Result<Expression, SyntacticalError> {
    let mut lhs = new_expression(pairs)?;

    while pairs.peek().is_some() {
        let comparator = new_comparator(pairs)?;
        let rhs = new_expression(pairs)?;

        lhs = Expression::Comparison(Box::new(Comparison {
            lhs,
            comparator,
            rhs
        }));
    }

    Ok(lhs)
}

fn new_operator(pairs: &mut Pairs<Rule>) -> Result<Operator, SyntacticalError> {
    let pair = pairs.next().ok_or(SyntacticalError::Incomplete)?;
    
    match pair.as_str() {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Subtract),
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        "^" => Ok(Operator::Exponent),
        "%" => Ok(Operator::Modulo),
        "~" => Ok(Operator::Tolerance),
        _ => Err(SyntacticalError::NoMatch(pair.as_rule())),
    }
}

fn new_comparator(pairs: &mut Pairs<Rule>) -> Result<Comparator, SyntacticalError> {
    let pair = pairs.next().ok_or(SyntacticalError::Incomplete)?;
    
    match pair.as_str() {
        "=" => Ok(Comparator::Equal),
        "<" => Ok(Comparator::LessThan),
        "<=" => Ok(Comparator::LessThanOrEqualTo),
        ">" => Ok(Comparator::GreaterThan),
        ">=" => Ok(Comparator::GreaterThanOrEqualTo),
        "<>" => Ok(Comparator::NotEqual),
        _ => Err(SyntacticalError::NoMatch(pair.as_rule())),
    }
}

fn new_expression(pairs: &mut Pairs<Rule>) -> Result<Expression, SyntacticalError> {
    pairs.next();
    Ok(Expression::Tuple(Box::new(Tuple::default())))
}

impl FromStr for Program {
    type Err = SyntacticalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let program = Parser::parse(Rule::Program, s)?
            .next()
            .ok_or(Self::Err::Incomplete)?;

        match program.as_rule() {
            Rule::Program => new_program(program.into_inner()),
            rule => Err(Self::Err::NoMatch(rule)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            include_str!("../../examples/peg.ta")
                .parse::<Program>()
                .unwrap(),
            Program::default()
        );
    }

    #[test]
    fn parse_bad_example() {
        assert!(include_str!("../../examples/bad.ta")
            .parse::<Program>()
            .is_err())
    }
}
