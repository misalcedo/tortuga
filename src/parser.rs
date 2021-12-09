//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::{ParseError, SyntaxError};
use crate::grammar::{
    BinaryOperation, Block, ComparisonOperation, ComparisonOperator, Expression, Grouping,
    Operator, Program, Variable,
};
use crate::number::{Number, Sign};
use crate::token::{Attachment, Kind, LexicalToken, Token, ValidToken};
use std::iter::{IntoIterator, Iterator, Peekable};

const BLOCK_END_TOKEN_KINDS: [Kind; 1] = [Kind::RightBracket];
const COMPARISON_TOKEN_KINDS: [Kind; 3] = [Kind::LessThan, Kind::GreaterThan, Kind::Equals];
const TERM_TOKEN_KINDS: [Kind; 2] = [Kind::Plus, Kind::Minus];
const FACTOR_TOKEN_KINDS: [Kind; 2] = [Kind::Star, Kind::ForwardSlash];
const EXPONENT_TOKEN_KINDS: [Kind; 1] = [Kind::Caret];

/// A stream of `Token`s to be consumed by the `Parser`.
trait TokenStream<'source> {
    /// Gets the next `Token` if it matches the expected kind. Otherwise, returns an error.
    /// Returns an error if there are not more `Token`s in the stream.
    fn next_kind(&mut self, expected: &[Kind])
        -> Result<ValidToken<'source>, SyntaxError<'source>>;

    /// Gets the next `Token` only if it has the given kind. Otherwise, returns an empty `Option`.
    /// Returns an error if there are not more `Token`s in the stream.
    fn next_if_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>>;

    /// Peeks the next `Token`'s `Kind`.
    /// Returns an error if there are not more `Token`s in the stream.
    fn peek_kind(&mut self) -> Result<Kind, SyntaxError<'source>>;

    /// Tests if the next token's kind matches one of the expected ones.
    fn next_matches_kind(&mut self, expected: &[Kind]) -> bool;

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    fn is_empty(&mut self) -> bool;
}

impl<'source, I> TokenStream<'source> for Parser<'source, I>
where
    I: Iterator<Item = Token<'source>>,
{
    fn next_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<ValidToken<'source>, SyntaxError<'source>> {
        Err(SyntaxError::IncompleteRule)
    }

    fn next_if_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        Err(SyntaxError::IncompleteRule)
    }

    fn peek_kind(&mut self) -> Result<Kind, SyntaxError<'source>> {
        match self.peek() {
            Some(Token::Valid(token)) => Ok(token.kind()),
            Some(Token::Invalid(token)) => Err(SyntaxError::InvalidToken(token.clone())),
            None => Err(SyntaxError::IncompleteRule),
        }
    }

    fn next_matches_kind(&mut self, expected: &[Kind]) -> bool {
        false
    }

    fn is_empty(&mut self) -> bool {
        self.peek().is_none() 
    }
}

/// A recursive descent parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item = Token<'source>>> {
    tokens: I,
    peeked: Option<Token<'source>>
}

impl<'source, I> Parser<'source, I>
where
    I: Iterator<Item = Token<'source>>,
{
    /// Creates a new `Parser`.
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Parser<'source, I> {
        Parser {
            tokens: tokens.into_iter(),
            peeked: None
        }
    }

    fn peek(&mut self) -> Option<&Token<'source>> {
        self.peeked = self.peeked.take().or_else(|| self.tokens.next());
        self.peeked.as_ref()
    }

    fn next(&mut self) -> Option<Token<'source>> {
        self.peek();
        self.peeked.take()
    }

    /// Parses the stream of tokens into a syntax tree.
    pub fn parse(mut self) -> Result<Program, ParseError<'source>> {
        let mut errors: Vec<SyntaxError<'source>> = Vec::new();
        let mut expressions = Vec::new();

        while !self.is_empty() {
            match self.parse_expression() {
                Err(SyntaxError::IncompleteRule) => return Err(ParseError::EndOfFile),
                Err(error) => {
                    errors.push(error);
                    self.synchronize();
                }
                Ok(expression) => {
                    expressions.push(expression);
                }
            }
        }

        if errors.is_empty() {
            Ok(Program::new(expressions))
        } else {
            Err(ParseError::MultipleErrors(errors.into()))
        }
    }

    /// Unwinds this parser's recursive descent into the grammar rules upon encountering an error parsing a rule.
    /// Some tokens may be skipped in order to allow the parser to identify additional errors in the source code.
    fn synchronize(&mut self) {
        // skip a single (potentially) bad token.
        self.tokens.next();
    }

    fn parse_expression(&mut self) -> Result<Expression, SyntaxError<'source>> {
        // TODO: Instead of using the kind here, could implement peek in the parser and implement the token stream trait for the parser.
        match self.peek_kind()? {
            Kind::LeftBracket => Ok(self.parse_block()?.into()),
            _ => self.parse_comparison(),
        }
    }

    fn parse_block(&mut self) -> Result<Block, SyntaxError<'source>> {
        self.next_kind(&[Kind::LeftBracket])?;

        let mut expressions = vec![self.parse_comparison()?];

        while !self.next_matches_kind(&BLOCK_END_TOKEN_KINDS) {
            expressions.push(self.parse_comparison()?)
        }

        self.next_kind(&BLOCK_END_TOKEN_KINDS)?;

        Ok(Block::new(expressions))
    }

    /// Parse a comparison grammar rule.
    fn parse_comparison(&mut self) -> Result<Expression, SyntaxError<'source>> {
        let mut expression = self.parse_term()?;

        while self.next_matches_kind(&COMPARISON_TOKEN_KINDS) {
            let operator = self.parse_comparison_operator()?;
            let right = self.parse_term()?;

            expression = ComparisonOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parses a comparison operator of the expected token kind.
    fn parse_comparison_operator(&mut self) -> Result<ComparisonOperator, SyntaxError<'source>> {
        let token = self.next_kind(&COMPARISON_TOKEN_KINDS)?;
        let mut operators = vec![token.kind()];

        while let Some(token) = self.next_if_kind(&COMPARISON_TOKEN_KINDS)? {
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
            _ => Err(SyntaxError::NoMatchingRule),
        }
    }

    /// Parse a term grammar rule (i.e., add and subtract).
    fn parse_term(&mut self) -> Result<Expression, SyntaxError<'source>> {
        let mut expression = self.parse_factor()?;

        while self.next_matches_kind(&TERM_TOKEN_KINDS) {
            let operator = self.parse_operator(&TERM_TOKEN_KINDS)?;
            let right = self.parse_factor()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parse a factor grammar rule (i.e., multiply and divide).
    fn parse_factor(&mut self) -> Result<Expression, SyntaxError<'source>> {
        let mut expression = self.parse_exponent()?;

        while self.next_matches_kind(&FACTOR_TOKEN_KINDS) {
            let operator = self.parse_operator(&FACTOR_TOKEN_KINDS)?;
            let right = self.parse_exponent()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parse a exponent grammar rule (i.e., multiply and divide).
    fn parse_exponent(&mut self) -> Result<Expression, SyntaxError<'source>> {
        let mut expression = self.parse_primary()?;

        while self.next_matches_kind(&EXPONENT_TOKEN_KINDS) {
            let operator = self.parse_operator(&EXPONENT_TOKEN_KINDS)?;
            let right = self.parse_primary()?;

            expression = BinaryOperation::new(expression, operator, right).into();
        }

        Ok(expression)
    }

    /// Parses an operator of the expected token kind.
    fn parse_operator(&mut self, expected: &[Kind]) -> Result<Operator, SyntaxError<'source>> {
        let token = self.next_kind(expected)?;

        match token.kind() {
            Kind::Plus => Ok(Operator::Add),
            Kind::Minus => Ok(Operator::Subtract),
            Kind::Star => Ok(Operator::Multiply),
            Kind::ForwardSlash => Ok(Operator::Divide),
            Kind::Caret => Ok(Operator::Exponent),
            _ => Err(SyntaxError::NoMatchingRule),
        }
    }

    /// Parse a primary grammar rule.
    fn parse_primary(&mut self) -> Result<Expression, SyntaxError<'source>> {
        match self.peek_kind()? {
            Kind::LeftParenthesis => self.parse_grouping().map(Expression::Grouping),
            Kind::Plus | Kind::Minus | Kind::Number => self.parse_number().map(Expression::Number),
            Kind::Identifier => self.parse_variable().map(Expression::Variable),
            _ => {
                self.next();
                Err(SyntaxError::NoMatchingRule)
            }
        }
    }

    /// Parse a grouping grammar rule.
    fn parse_grouping(&mut self) -> Result<Grouping, SyntaxError<'source>> {
        self.next_kind(&[Kind::LeftParenthesis])?;

        let expression = self.parse_expression()?;

        self.next_kind(&[Kind::RightParenthesis])?;

        Ok(Grouping::new(expression))
    }

    /// Parse a number literal with an optional plus or minus sign.
    fn parse_number(&mut self) -> Result<Number, SyntaxError<'source>> {
        let sign = match self
            .next_if_kind(&[Kind::Plus, Kind::Minus])?
            .as_ref()
            .map(ValidToken::kind)
        {
            Some(Kind::Minus) => Sign::Negative,
            _ => Sign::Positive,
        };

        let token = self.next_kind(&[Kind::Number])?;

        if let Attachment::Number(mut number) = token.attachment() {
            number.set_sign(sign);

            Ok(number.clone())
        } else {
            Err(SyntaxError::NoMatchingRule)
        }
    }

    /// Parses an identifier token as a variable.
    fn parse_variable(&mut self) -> Result<Variable, SyntaxError<'source>> {
        let token = self.next_kind(&[Kind::Identifier])?;

        Ok(Variable::new(token.source()))
    }
}
