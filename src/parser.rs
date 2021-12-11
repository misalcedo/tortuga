//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::{ParseError, SyntaxError};
use crate::grammar::{
    BinaryOperation, Block, ComparisonOperation, ComparisonOperator, Expression, Grouping,
    Operator, Program, Variable,
};
use crate::number::{Number, Sign};
use crate::stream::TokenStream;
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

fn parse_expression<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Expression, SyntaxError<'source>> {
    match tokens.peek_kind()? {
        Kind::LeftBracket => Ok(parse_block(tokens)?.into()),
        _ => parse_comparison(tokens),
    }
}

fn parse_block<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Block, SyntaxError<'source>> {
    tokens.next_kind(&[Kind::LeftBracket])?;

    let mut expressions = vec![parse_comparison(tokens)?];

    while !tokens.next_matches_kind(&BLOCK_END_TOKEN_KINDS) {
        expressions.push(parse_comparison(tokens)?)
    }

    tokens.next_kind(&BLOCK_END_TOKEN_KINDS)?;

    Ok(Block::new(expressions))
}

/// Parse a comparison grammar rule.
fn parse_comparison<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Expression, SyntaxError<'source>> {
    let mut expression = parse_term(tokens)?;

    while tokens.next_matches_kind(&COMPARISON_TOKEN_KINDS) {
        let operator = parse_comparison_operator(tokens)?;
        let right = parse_term(tokens)?;

        expression = ComparisonOperation::new(expression, operator, right).into();
    }

    Ok(expression)
}

/// Parses a comparison operator of the expected token kind.
fn parse_comparison_operator<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<ComparisonOperator, SyntaxError<'source>> {
    let token = tokens.next_kind(&COMPARISON_TOKEN_KINDS)?;
    let mut operators = vec![token.kind()];

    while let Some(token) = tokens.next_if_kind(&COMPARISON_TOKEN_KINDS)? {
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
fn parse_term<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Expression, SyntaxError<'source>> {
    let mut expression = parse_factor(tokens)?;

    while tokens.next_matches_kind(&TERM_TOKEN_KINDS) {
        let operator = parse_operator(tokens, &TERM_TOKEN_KINDS)?;
        let right = parse_factor(tokens)?;

        expression = BinaryOperation::new(expression, operator, right).into();
    }

    Ok(expression)
}

/// Parse a factor grammar rule (i.e., multiply and divide).
fn parse_factor<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Expression, SyntaxError<'source>> {
    let mut expression = parse_exponent(tokens)?;

    while tokens.next_matches_kind(&FACTOR_TOKEN_KINDS) {
        let operator = parse_operator(tokens, &FACTOR_TOKEN_KINDS)?;
        let right = parse_exponent(tokens)?;

        expression = BinaryOperation::new(expression, operator, right).into();
    }

    Ok(expression)
}

/// Parse a exponent grammar rule (i.e., multiply and divide).
fn parse_exponent<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Expression, SyntaxError<'source>> {
    let mut expression = parse_primary(tokens)?;

    while tokens.next_matches_kind(&EXPONENT_TOKEN_KINDS) {
        let operator = parse_operator(tokens, &EXPONENT_TOKEN_KINDS)?;
        let right = parse_primary(tokens)?;

        expression = BinaryOperation::new(expression, operator, right).into();
    }

    Ok(expression)
}

/// Parses an operator of the expected token kind.
fn parse_operator<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
    expected: &[Kind],
) -> Result<Operator, SyntaxError<'source>> {
    let token = tokens.next_kind(expected)?;

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
fn parse_primary<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Expression, SyntaxError<'source>> {
    match tokens.peek_kind()? {
        Kind::LeftParenthesis => parse_grouping(tokens).map(Expression::Grouping),
        Kind::Plus | Kind::Minus | Kind::Number => parse_number(tokens).map(Expression::Number),
        Kind::Identifier => parse_variable(tokens).map(Expression::Variable),
        _ => match tokens.next()? {
            Some(token) => Err(SyntaxError::NoMatchingRule(
                token,
                PRIMARY_TOKEN_KINDS.to_vec(),
            )),
            None => Err(SyntaxError::IncompleteRule(PRIMARY_TOKEN_KINDS.to_vec())),
        },
    }
}

/// Parse a number literal with an optional plus or minus sign.
fn parse_number<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Number, SyntaxError<'source>> {
    let sign = match tokens
        .next_if_kind(&[Kind::Plus, Kind::Minus])?
        .as_ref()
        .map(ValidToken::kind)
    {
        Some(Kind::Minus) => Some(Sign::Negative),
        Some(Kind::Plus) => Some(Sign::Positive),
        _ => None,
    };

    let mut token = tokens.next_kind(&[Kind::Number])?;

    if let Attachment::Number(number) = token.attachment() {
        if number.has_sign() && sign.is_some() {
            return Err(SyntaxError::NoMatchingRule(token, vec![]));
        }
    }

    if let Attachment::Number(mut number) = token.take_attachment() {
        sign.map(|s| number.set_sign(s));

        Ok(number)
    } else {
        Err(SyntaxError::NoMatchingRule(token, vec![Kind::Number]))
    }
}

/// Parses an identifier token as a variable.
fn parse_variable<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Variable, SyntaxError<'source>> {
    let token = tokens.next_kind(&[Kind::Identifier])?;

    Ok(Variable::new(token.source()))
}

/// Parse a grouping grammar rule.
fn parse_grouping<'source, I: Iterator<Item = Token<'source>>>(
    tokens: &mut TokenStream<'source, I>,
) -> Result<Grouping, SyntaxError<'source>> {
    tokens.next_kind(&[Kind::LeftParenthesis])?;

    let expression = parse_expression(tokens)?;

    tokens.next_kind(&[Kind::RightParenthesis])?;

    Ok(Grouping::new(expression))
}

/// A recursive descent parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item = Token<'source>>> {
    tokens: TokenStream<'source, I>,
}

impl<'source, I: Iterator<Item = Token<'source>>> Parser<'source, I> {
    /// Creates a new `Parser`.
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Parser<'source, I> {
        Parser {
            tokens: TokenStream::from(tokens.into_iter()),
        }
    }

    /// Parses the stream of tokens into a syntax tree.
    pub fn parse(mut self) -> Result<Program, ParseError> {
        let mut errors: Vec<SyntaxError> = Vec::new();
        let mut expressions = Vec::new();

        while !self.tokens.is_empty() {
            match parse_expression(&mut self.tokens) {
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
            Ok(Program::from(expressions))
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
        if self.tokens.is_empty() {
            return None;
        }

        loop {
            match parse_expression(&mut self.tokens) {
                Ok(expression) => return Some(expression),
                Err(error) => debug!("Skipping an error encountered while parsing an expression during panic mode: {}", error)
            }

            match self.tokens.next() {
                Ok(Some(token)) => debug!("Skipped token during token mode: {:?}.", token),
                Ok(None) => return None,
                Err(error) => debug!(
                    "Skipping an error encountered while skipping tokens during panic mode: {}",
                    error
                ),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::number::Number;
    use test_log::test;

    #[test]
    fn parse_number_double_sign() {
        let parser = Parser::new(lex_tokens("-2#+01"));

        assert_eq!(parser.parse(), Err(ParseError::MultipleErrors));
    }

    #[test]
    fn parse_signed_number() {
        let parser = Parser::new(lex_tokens("-1"));

        assert_eq!(
            parser.parse().unwrap(),
            Program::from(vec![Expression::Number(Number::new_signed_integer(
                Sign::Negative,
                1
            ))])
        );
    }

    #[test]
    fn parse_radix_number_badly_signed() {
        let parser = Parser::new(lex_tokens("-2#01"));

        assert_eq!(parser.parse(), Err(ParseError::MultipleErrors));
    }

    #[test]
    fn parse_radix_number_signed() {
        let parser = Parser::new(lex_tokens("2#+01"));

        assert_eq!(
            parser.parse(),
            Ok(vec![Expression::Number(Number::new_signed_integer(
                Sign::Positive,
                1
            ))]
            .into())
        );
    }

    #[test]
    fn parse_radix_number_unsigned() {
        let parser = Parser::new(lex_tokens("2#01"));

        assert_eq!(
            parser.parse().unwrap(),
            Program::from(vec![Expression::Number(Number::new_signed_integer(
                Sign::Positive,
                1
            ))])
        );
    }

    #[test]
    fn parse_equals_expression() {
        let parser = Parser::new(lex_tokens("x=2#-01"));

        assert_eq!(
            parser.parse().unwrap(),
            Program::from(vec![Expression::ComparisonOperation(
                ComparisonOperation::new(
                    Expression::Variable(Variable::new("x")),
                    ComparisonOperator::EqualTo,
                    Expression::Number(Number::new_signed_integer(Sign::Negative, 1))
                )
            )])
        );
    }

    #[test]
    fn parse_math_expression() {
        let parser = Parser::new(lex_tokens("(2^3+5-10)*3/9"));
        let exponent = BinaryOperation::new(
            Number::new_integer(2).into(),
            Operator::Exponent,
            Number::new_integer(3).into(),
        )
        .into();
        let add =
            BinaryOperation::new(exponent, Operator::Add, Number::new_integer(5).into()).into();
        let subtract =
            BinaryOperation::new(add, Operator::Subtract, Number::new_integer(10).into()).into();
        let grouping = Grouping::new(subtract).into();
        let multiply =
            BinaryOperation::new(grouping, Operator::Multiply, Number::new_integer(3).into())
                .into();
        let divide =
            BinaryOperation::new(multiply, Operator::Divide, Number::new_integer(9).into()).into();
        let expected = Program::from(vec![divide]);

        let actual = parser.parse().unwrap();

        assert_eq!(actual, expected);
    }

    fn lex_tokens<'source>(source: &'source str) -> Vec<Token<'source>> {
        Lexer::new(&mut source.into()).collect()
    }
}
