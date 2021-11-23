//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::ParseError;
use crate::grammar::{
    BinaryOperation, ComparisonOperation, ComparisonOperator, Expression, Grouping, Operator,
    TextReference,
};
use crate::number::{Number, Sign};
use crate::token::{Token, TokenKind};
use std::convert::TryFrom;
use std::iter::{IntoIterator, Iterator, Peekable};

const COMPARISON_TOKEN_KINDS: [TokenKind; 3] = [
    TokenKind::LessThan,
    TokenKind::GreaterThan,
    TokenKind::Equals,
];
const TERM_TOKEN_KINDS: [TokenKind; 2] = [TokenKind::Plus, TokenKind::Minus];
const FACTOR_TOKEN_KINDS: [TokenKind; 2] = [TokenKind::Star, TokenKind::ForwardSlash];

/// A recursive descent parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item = Token<'source>>> {
    tokens: Peekable<I>,
}

impl<'source, I> Parser<'source, I>
where
    I: Iterator<Item = Token<'source>>,
{
    /// Creates a new parser.
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Parser<'source, I> {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Parses the stream of tokens into a syntax tree.
    pub fn parse(mut self) -> Result<Expression, ParseError> {
        match self.parse_expression() {
            error @ Err(_) => {
                self.synchronize();
                error
            }
            ok => ok,
        }
    }

    /// Unwinds this parser's recursive descent into the grammar rules upon encountering an error parsing a rule.
    /// Some tokens may be skipped in order to allow the parser to identify additional errors in the source code.
    fn synchronize(&mut self) {}

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_comparison()
    }

    /// Parse a comparison grammar rule.
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_term()?;

        while self.next_matches_kind(&COMPARISON_TOKEN_KINDS) {
            let operator = self.parse_comparison_operator()?;
            let right = self.parse_term()?;

            expression = ComparisonOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parses a comparison operator of the expected token kind.
    fn parse_comparison_operator(&mut self) -> Result<ComparisonOperator, ParseError> {
        let token = self.next_kind(&COMPARISON_TOKEN_KINDS)?;
        let mut operators = vec![token.kind()];

        while let Some(token) = self.next_if_kind(&COMPARISON_TOKEN_KINDS)? {
            operators.push(token.kind())
        }

        match operators.as_slice() {
            [TokenKind::LessThan] => Ok(ComparisonOperator::LessThan),
            [TokenKind::LessThan, TokenKind::Equals] => Ok(ComparisonOperator::LessThanOrEqualTo),
            [TokenKind::Equals, TokenKind::LessThan] => Ok(ComparisonOperator::LessThanOrEqualTo),
            [TokenKind::GreaterThan] => Ok(ComparisonOperator::GreaterThan),
            [TokenKind::GreaterThan, TokenKind::Equals] => {
                Ok(ComparisonOperator::GreaterThanOrEqualTo)
            }
            [TokenKind::Equals, TokenKind::GreaterThan] => {
                Ok(ComparisonOperator::GreaterThanOrEqualTo)
            }
            [TokenKind::Equals] => Ok(ComparisonOperator::EqualTo),
            [TokenKind::LessThan, TokenKind::GreaterThan] => Ok(ComparisonOperator::NotEqualTo),
            [TokenKind::GreaterThan, TokenKind::LessThan] => Ok(ComparisonOperator::NotEqualTo),
            [TokenKind::LessThan, TokenKind::Equals, TokenKind::GreaterThan] => {
                Ok(ComparisonOperator::Comparable)
            }
            [TokenKind::LessThan, TokenKind::GreaterThan, TokenKind::Equals] => {
                Ok(ComparisonOperator::Comparable)
            }
            [TokenKind::GreaterThan, TokenKind::Equals, TokenKind::LessThan] => {
                Ok(ComparisonOperator::Comparable)
            }
            [TokenKind::GreaterThan, TokenKind::LessThan, TokenKind::Equals] => {
                Ok(ComparisonOperator::Comparable)
            }
            [TokenKind::Equals, TokenKind::LessThan, TokenKind::GreaterThan] => {
                Ok(ComparisonOperator::Comparable)
            }
            [TokenKind::Equals, TokenKind::GreaterThan, TokenKind::LessThan] => {
                Ok(ComparisonOperator::Comparable)
            }
            _ => Err(ParseError::InvalidComparator(
                token.start(),
                operators.into(),
            )),
        }
    }

    /// Parse a term grammar rule (i.e., add and subtract).
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_factor()?;

        while self.next_matches_kind(&TERM_TOKEN_KINDS) {
            let operator = self.parse_operator(&TERM_TOKEN_KINDS)?;
            let right = self.parse_factor()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parse a factor grammar rule (i.e., multiply and divide).
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.parse_primary()?;

        while self.next_matches_kind(&FACTOR_TOKEN_KINDS) {
            let operator = self.parse_operator(&FACTOR_TOKEN_KINDS)?;
            let right = self.parse_primary()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parses an operator of the expected token kind.
    fn parse_operator(&mut self, expected: &[TokenKind]) -> Result<Operator, ParseError> {
        let token = self.next_kind(expected)?;

        match token.kind() {
            TokenKind::Plus => Ok(Operator::Add),
            TokenKind::Minus => Ok(Operator::Subtract),
            TokenKind::Star => Ok(Operator::Multiply),
            TokenKind::ForwardSlash => Ok(Operator::Divide),
            kind => Err(ParseError::NoMatchingGrammar(token.start(), kind)),
        }
    }

    /// Parse a primary grammar rule.
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::LeftParenthesis) => self.parse_grouping().map(Expression::Grouping),
            Some(TokenKind::Plus | TokenKind::Minus | TokenKind::Number) => {
                self.parse_number().map(Expression::Number)
            }
            Some(TokenKind::TextReference) => {
                self.parse_text_reference().map(Expression::TextReference)
            }
            Some(kind) => {
                let token = self.next_kind(&[kind])?;
                Err(ParseError::NoMatchingGrammar(token.start(), token.kind()))
            }
            None => Err(ParseError::mismatched_kind(
                &[
                    TokenKind::LeftParenthesis,
                    TokenKind::Plus,
                    TokenKind::Minus,
                    TokenKind::Number,
                    TokenKind::TextReference,
                ],
                None,
            )),
        }
    }

    /// Parse a grouping grammar rule.
    fn parse_grouping(&mut self) -> Result<Grouping, ParseError> {
        self.next_kind(&[TokenKind::LeftParenthesis])?;

        let expression = self.parse_expression()?;

        self.next_kind(&[TokenKind::RightParenthesis])?;

        Ok(Grouping::new(expression))
    }

    /// Parse a text reference literal.
    fn parse_text_reference(&mut self) -> Result<TextReference, ParseError> {
        let reference = self.next_kind(&[TokenKind::TextReference])?;
        let lexeme = reference.lexeme();

        Ok(TextReference::new(&lexeme[1..lexeme.len() - 1]))
    }

    /// Parse a number literal with an optional plus or minus sign.
    fn parse_number(&mut self) -> Result<Number, ParseError> {
        let sign = match self
            .next_if_kind(&[TokenKind::Plus, TokenKind::Minus])?
            .as_ref()
            .map(Token::kind)
        {
            Some(TokenKind::Minus) => Sign::Negative,
            _ => Sign::Positive,
        };

        let token = self.next_kind(&[TokenKind::Number])?;
        let mut number = Number::try_from(token)?;

        number.set_sign(sign);

        Ok(number)
    }

    /// Gets the next token if it matches the expected kind or returns an error.
    fn next_kind(&mut self, expected: &[TokenKind]) -> Result<Token<'source>, ParseError> {
        self.next_if_kind(expected)?
            .ok_or_else(|| ParseError::mismatched_kind(expected, self.tokens.peek()))
    }

    /// Gets the next token only if it has one of the given kind.
    fn next_if_kind(
        &mut self,
        expected: &[TokenKind],
    ) -> Result<Option<Token<'source>>, ParseError> {
        self.tokens
            .next_if(|token| expected.contains(&token.kind()))
            .map(ParseError::validate)
            .transpose()
    }

    /// Peeks the next token's kind.
    fn peek_kind(&mut self) -> Option<TokenKind> {
        self.tokens.peek().map(|token| token.kind())
    }

    /// Tests if the next token's kind matches one of the expected ones.
    fn next_matches_kind(&mut self, expected: &[TokenKind]) -> bool {
        self.peek_kind()
            .map(|kind| expected.contains(&kind))
            .unwrap_or(false)
    }
}
