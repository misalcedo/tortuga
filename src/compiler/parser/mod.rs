//! Parse a sequence of tokens into a syntax tree.

mod tokens;

use crate::compiler::errors::syntactical::ErrorKind;
use crate::compiler::{Kind, Token};
use crate::grammar::syntax::Comparator::*;
use crate::grammar::syntax::{Comparator, Comparison, Comparisons, Expression, List, Program};
use crate::{Scanner, SyntacticalError};
use std::iter::Peekable;
use std::str::FromStr;
use tokens::Tokens;

const COMPARISON_KINDS: &[Kind] = &[
    Kind::LessThan,
    Kind::GreaterThan,
    Kind::LessThanOrEqualTo,
    Kind::GreaterThanOrEqualTo,
    Kind::Equal,
    Kind::NotEqual,
];

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
pub struct Parser<T: Tokens> {
    tokens: T,
}

impl<'a> From<&'a str> for Parser<Peekable<Scanner<'a>>> {
    fn from(source: &'a str) -> Self {
        Parser {
            tokens: Scanner::from(source).peekable(),
        }
    }
}

impl<T: Tokens> From<T> for Parser<T> {
    fn from(tokens: T) -> Self {
        Parser { tokens }
    }
}

impl<T: Tokens> Parser<T> {
    /// Advances the token sequence and returns the next value if the token is one of the expected [`Kind`]s.
    ///
    /// Returns [`Err`] when at the end of the sequence,
    /// if the token's kind does not match, or if the token is invalid.
    fn next_kind(&mut self, kinds: &[Kind]) -> Result<Token, SyntacticalError> {
        if self.tokens.has_next() {
            match self.tokens.next_if_kind(kinds) {
                Some(token) => Ok(token),
                None => Err(SyntacticalError::from(ErrorKind::NoMatch)),
            }
        } else {
            Err(SyntacticalError::from(ErrorKind::Incomplete))
        }
    }

    /// Generate a syntax tree rooted at a `Program` for this `Parser`'s sequence of tokens.
    pub fn parse(mut self) -> Result<Program, SyntacticalError> {
        let expression = self.parse_expression()?;

        match self
            .tokens
            .peek_kind()
            .ok_or_else(|| SyntacticalError::from(ErrorKind::Incomplete))?
        {
            Kind::LessThan
            | Kind::GreaterThan
            | Kind::LessThanOrEqualTo
            | Kind::GreaterThanOrEqualTo
            | Kind::Equal
            | Kind::NotEqual => self.parse_comparisons(expression),
            _ => self.parse_expressions(expression),
        }
    }

    fn parse_expressions(&mut self, expression: Expression) -> Result<Program, SyntacticalError> {
        let mut expressions = Vec::new();

        while self.tokens.has_next() {
            expressions.push(self.parse_expression()?);
        }

        Ok(List::new(expression, expressions).into())
    }

    fn parse_comparisons(&mut self, expression: Expression) -> Result<Program, SyntacticalError> {
        let head = self.parse_comparison()?;
        let mut comparisons = Vec::new();

        while self.tokens.has_next() {
            comparisons.push(self.parse_comparison()?);
        }

        Ok(Comparisons::new(expression, List::new(head, comparisons)).into())
    }

    fn parse_comparison(&mut self) -> Result<Comparison, SyntacticalError> {
        let operator = self.parse_comparison_operator(COMPARISON_KINDS)?;
        let expression = self.parse_expression()?;

        Ok(Comparison::new(operator, expression))
    }

    fn parse_comparison_operator(
        &mut self,
        kinds: &[Kind],
    ) -> Result<Comparator, SyntacticalError> {
        let operator = match self.next_kind(kinds)?.kind() {
            Kind::LessThan => LessThan,
            Kind::GreaterThan => GreaterThan,
            Kind::LessThanOrEqualTo => LessThanOrEqualTo,
            Kind::GreaterThanOrEqualTo => GreaterThanOrEqualTo,
            Kind::NotEqual => NotEqualTo,
            _ => NotEqualTo,
        };

        Ok(operator)
    }

    fn parse_expression(&mut self) -> Result<Expression, SyntacticalError> {
        Err(ErrorKind::NoMatch.into())
    }
}

impl FromStr for Program {
    type Err = SyntacticalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::from(s).parse()
    }
}
