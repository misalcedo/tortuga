//! Parse a sequence of tokens into a syntax tree.
//!
//! Relies on a Parser Expression Grammar to generate a parser for the language.
//!
//! See <https://en.wikipedia.org/wiki/Parsing_expression_grammar>

use crate::grammar::syntax::*;
use crate::SyntacticalError;

use pest::iterators::{Pair, Pairs};
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
        program.0.push(new_expressions(&mut pairs)?);
    }

    Ok(program)
}

fn new_expressions(pairs: &mut Pairs<Rule>) -> Result<Expression, SyntacticalError> {
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

impl FromStr for Continuation {
    type Err = SyntacticalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _pairs = Parser::parse(Rule::Continuation, s)?;

        Ok(Continuation::default())
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
