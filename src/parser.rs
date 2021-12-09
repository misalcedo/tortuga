//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::{ParseError, SyntaxError};
use crate::grammar::{
    BinaryOperation, Block, ComparisonOperation, ComparisonOperator, Expression, Grouping,
    Operator, Program, Variable,
};
use crate::number::{Number, Sign};
use crate::token::{Attachment, Kind, LexicalToken, Token, ValidToken};
use std::iter::{IntoIterator, Iterator};
use tracing::{debug, error};

const BLOCK_END_TOKEN_KINDS: [Kind; 1] = [Kind::RightBracket];
const COMPARISON_TOKEN_KINDS: [Kind; 3] = [Kind::LessThan, Kind::GreaterThan, Kind::Equals];
const TERM_TOKEN_KINDS: [Kind; 2] = [Kind::Plus, Kind::Minus];
const FACTOR_TOKEN_KINDS: [Kind; 2] = [Kind::Star, Kind::ForwardSlash];
const EXPONENT_TOKEN_KINDS: [Kind; 1] = [Kind::Caret];
const PRIMARY_TOKEN_KINDS: [Kind; 5] = [
    Kind::LeftParenthesis,
    Kind::Plus,
    Kind::Minus,
    Kind::Number,
    Kind::Identifier,
];

/// A stream of `Token`s to be consumed by the `Parser`.
struct TokenStream<'source, I: Iterator<Item = Token<'source>>> {
    tokens: I,
    peeked: Option<ValidToken<'source>>,
}

impl<'source, I: Iterator<Item = Token<'source>>> TokenStream<'source, I> {
    /// Creates a new instance of a `TokenStream`.
    fn new(tokens: I) -> Self {
        TokenStream {
            tokens,
            peeked: None,
        }
    }

    fn peek(&mut self) -> Result<Option<&ValidToken<'source>>, SyntaxError<'source>> {
        self.peeked = match self.peeked.take() {
            Some(token) => Some(token),
            None => self.next()?,
        };

        Ok(self.peeked.as_ref())
    }

    fn next(&mut self) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        match self.peeked.take() {
            Some(token) => Ok(Some(token)),
            None => match self.tokens.next() {
                Some(Token::Valid(token)) => Ok(Some(token)),
                Some(Token::Invalid(token)) => Err(SyntaxError::InvalidToken(token)),
                None => Ok(None),
            },
        }
    }

    /// Gets the next `Token` if it matches the expected kind. Otherwise, returns an error.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn next_kind(&mut self, expected: &[Kind]) -> Result<ValidToken<'source>, SyntaxError<'source>> {
        let token = self
            .next()?
            .ok_or_else(|| SyntaxError::IncompleteRule(expected.to_vec()))?;

        if expected.contains(&token.kind()) {
            Ok(token)
        } else {
            Err(SyntaxError::NoMatchingRule(token, expected.to_vec()))
        }
    }

    /// Gets the next `Token` only if it has the given kind. Otherwise, returns an empty `Option`.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn next_if_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        if expected.contains(&self.peek_kind()?) {
            self.next()
        } else {
            Ok(None)
        }
    }

    /// Peeks the next `Token`'s `Kind`.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn peek_kind(&mut self) -> Result<Kind, SyntaxError<'source>> {
        match self.peek()? {
            Some(token) => Ok(token.kind()),
            None => Err(SyntaxError::IncompleteRule(Vec::new())),
        }
    }

    /// Tests if the next token's kind matches one of the expected ones.
    pub fn next_matches_kind(&mut self, expected: &[Kind]) -> bool {
        false
    }

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    pub fn is_empty(&mut self) -> bool {
        match self.peek() {
            Ok(None) => true,
            _ => false,
        }
    }
}

/// Parse a number literal with an optional plus or minus sign.
fn parse_number<'source, I: Iterator<Item = Token<'source>>>(tokens: &mut TokenStream<'source, I>) -> Result<Number, SyntaxError<'source>> {
    let sign = match tokens
        .next_if_kind(&[Kind::Plus, Kind::Minus])?
        .as_ref()
        .map(ValidToken::kind)
    {
        Some(Kind::Minus) => Sign::Negative,
        _ => Sign::Positive,
    };
    
    
    let token = tokens.next_kind(&[Kind::Number])?;

    if let Attachment::Number(mut number) = token.attachment() {
        number.set_sign(sign);

        Ok(number.clone())
    } else {
        Err(SyntaxError::NoMatchingRule(token, vec![Kind::Number]))
    }
}

/// A recursive descent parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item = Token<'source>>> {
    tokens: TokenStream<'source, I>,
}

impl<'source, I: Iterator<Item = Token<'source>>> Parser<'source, I> {
    /// Creates a new `Parser`.
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Parser<'source, I> {
        Parser {
            tokens: TokenStream::new(tokens.into_iter()),
        }
    }

    /// Parses the stream of tokens into a syntax tree.
    pub fn parse(mut self) -> Result<Program, ParseError> {
        let mut errors: Vec<SyntaxError> = Vec::new();
        let mut expressions = Vec::new();

        while !self.tokens.is_empty() {
            match self.parse_expression() {
                Err(SyntaxError::IncompleteRule(..)) => return Err(ParseError::EndOfFile),
                Err(error) => {
                    errors.push(error);
                    self.synchronize()
                        .map(|expression| expressions.push(expression));
                }
                Ok(expression) => {
                    expressions.push(expression);
                }
            }
        }

        if errors.is_empty() {
            Ok(Program::new(expressions))
        } else {
            for error in errors {
                error!("{}", error);
            }

            Err(ParseError::MultipleErrors)
        }
    }

    /// Unwinds this parser's recursive descent into the grammar rules upon encountering an error parsing a rule.
    /// Some tokens may be skipped in order to allow the parser to identify additional errors in the source code.
    fn synchronize(&mut self) -> Option<Expression> {
        loop {
            match self.parse_expression() {
                Ok(expression) => return Some(expression),
                Err(error) => debug!("Skipping an error encountered while parsing an expression during panic mode: {}", error)
            }

            match self.tokens.next() {
                Ok(token) => debug!("Skipped token during token mode: {:?}.", token?),
                Err(error) => debug!(
                    "Skipping an error encountered while skipping tokens during panic mode: {}",
                    error
                ),
            };
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        let kind = self.tokens.peek_kind()?;
        match kind {
            Kind::LeftBracket => Ok(self.parse_block()?.into()),
            _ => self.parse_comparison(),
        }
    }

    fn parse_block(&mut self) -> Result<Block, SyntaxError> {
        self.tokens.next_kind(&[Kind::LeftBracket])?;

        let mut expressions = vec![self.parse_comparison()?];

        while !self.tokens.next_matches_kind(&BLOCK_END_TOKEN_KINDS) {
            expressions.push(self.parse_comparison()?)
        }

        self.tokens.next_kind(&BLOCK_END_TOKEN_KINDS)?;

        Ok(Block::new(expressions))
    }

    /// Parse a comparison grammar rule.
    fn parse_comparison(&mut self) -> Result<Expression, SyntaxError> {
        let mut expression = self.parse_term()?;

        while self.tokens.next_matches_kind(&COMPARISON_TOKEN_KINDS) {
            let operator = self.parse_comparison_operator()?;
            let right = self.parse_term()?;

            expression = ComparisonOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parses a comparison operator of the expected token kind.
    fn parse_comparison_operator(&mut self) -> Result<ComparisonOperator, SyntaxError> {
        let token = self.tokens.next_kind(&COMPARISON_TOKEN_KINDS)?;
        let mut operators = vec![token.kind()];

        while let Some(token) = self.tokens.next_if_kind(&COMPARISON_TOKEN_KINDS)? {
            operators.push(token.kind())
        }

        match operators.as_slice() {
            [Kind::LessThan] => Ok(ComparisonOperator::LessThan),
            [Kind::LessThan, Kind::Equals] => Ok(ComparisonOperator::LessThanOrEqualTo),
            [Kind::GreaterThan] => Ok(ComparisonOperator::GreaterThan),
            [Kind::GreaterThan, Kind::Equals] => Ok(ComparisonOperator::GreaterThanOrEqualTo),
            [Kind::Equals] => Ok(ComparisonOperator::EqualTo),
            [Kind::LessThan, Kind::GreaterThan] => Ok(ComparisonOperator::NotEqualTo),
            [Kind::LessThan, Kind::Equals, Kind::GreaterThan] => Ok(ComparisonOperator::Comparable),
            _ => Err(SyntaxError::NoMatchingRule(
                token,
                COMPARISON_TOKEN_KINDS.to_vec(),
            )),
        }
    }

    /// Parse a term grammar rule (i.e., add and subtract).
    fn parse_term(&mut self) -> Result<Expression, SyntaxError> {
        let mut expression = self.parse_factor()?;

        while self.tokens.next_matches_kind(&TERM_TOKEN_KINDS) {
            let operator = self.parse_operator(&TERM_TOKEN_KINDS)?;
            let right = self.parse_factor()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parse a factor grammar rule (i.e., multiply and divide).
    fn parse_factor(&mut self) -> Result<Expression, SyntaxError> {
        let mut expression = self.parse_exponent()?;

        while self.tokens.next_matches_kind(&FACTOR_TOKEN_KINDS) {
            let operator = self.parse_operator(&FACTOR_TOKEN_KINDS)?;
            let right = self.parse_exponent()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parse a exponent grammar rule (i.e., multiply and divide).
    fn parse_exponent(&mut self) -> Result<Expression, SyntaxError> {
        let mut expression = self.parse_primary()?;

        while self.tokens.next_matches_kind(&EXPONENT_TOKEN_KINDS) {
            let operator = self.parse_operator(&EXPONENT_TOKEN_KINDS)?;
            let right = self.parse_primary()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parses an operator of the expected token kind.
    fn parse_operator(&mut self, expected: &[Kind]) -> Result<Operator, SyntaxError> {
        let token = self.tokens.next_kind(expected)?;

        match token.kind() {
            Kind::Plus => Ok(Operator::Add),
            Kind::Minus => Ok(Operator::Subtract),
            Kind::Star => Ok(Operator::Multiply),
            Kind::ForwardSlash => Ok(Operator::Divide),
            Kind::Caret => Ok(Operator::Exponent),
            _ => Err(SyntaxError::NoMatchingRule(token, expected.to_vec())),
        }
    }

    /// Parse a primary grammar rule.
    fn parse_primary(&mut self) -> Result<Expression, SyntaxError> {
        let token = self.tokens.next_kind(&PRIMARY_TOKEN_KINDS)?;

        match token.kind() {
            Kind::LeftParenthesis => self.parse_grouping().map(Expression::Grouping),
            Kind::Plus | Kind::Minus | Kind::Number => parse_number(&mut self.tokens).map(Expression::Number),
            Kind::Identifier => self.parse_variable().map(Expression::Variable),
            _ => {
                self.tokens.next();
                Err(SyntaxError::NoMatchingRule(
                    token,
                    PRIMARY_TOKEN_KINDS.to_vec(),
                ))
            }
        }
    }

    /// Parse a grouping grammar rule.
    fn parse_grouping(&mut self) -> Result<Grouping, SyntaxError> {
        self.tokens.next_kind(&[Kind::LeftParenthesis])?;

        let expression = self.parse_expression()?;

        self.tokens.next_kind(&[Kind::RightParenthesis])?;

        Ok(Grouping::new(expression))
    }

    

    /// Parses an identifier token as a variable.
    fn parse_variable(&mut self) -> Result<Variable, SyntaxError> {
        let token = self.tokens.next_kind(&[Kind::Identifier])?;

        Ok(Variable::new(token.source()))
    }
}
