//! Parse a sequence of tokens into a syntax tree.

mod tokens;

use crate::compiler::errors::{syntactical::ErrorKind, LexicalError, Reporter};
use crate::compiler::{Kind, Token};
use crate::grammar::syntax::{Comparison, Comparisons, Expression, List, Program};
use crate::{Scanner, SyntacticalError};
use std::iter::Peekable;
use std::str::FromStr;
use tokens::Tokens;

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
pub struct Parser<'a, I: Iterator<Item = Result<Token, LexicalError>>> {
    tokens: Peekable<I>,
    reporter: Reporter<'a>,
}

impl<'a> From<&'a str> for Parser<'a, Scanner<'a>> {
    fn from(source: &'a str) -> Self {
        Parser {
            tokens: Scanner::from(source).peekable(),
            reporter: source.into(),
        }
    }
}

impl<'a, I: Iterator<Item = Result<Token, LexicalError>>> Parser<'a, I> {
    /// Creates a new `Parser`.
    pub fn new<II: IntoIterator<IntoIter = I, Item = I::Item>, R: Into<Reporter<'a>>>(
        tokens: II,
        reporter: R,
    ) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            reporter: reporter.into(),
        }
    }

    /// Generate a syntax tree rooted at a `Program` for this `Parser`'s sequence of tokens.
    pub fn parse(&mut self) -> Result<Program, SyntacticalError> {
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
        Err(ErrorKind::NoMatch.into())
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
