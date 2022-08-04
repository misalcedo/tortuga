//! Parse a sequence of tokens into a syntax tree.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the nesting of the parsing functions to denote each precedence level.

mod error;
mod precedence;

use crate::grammar::{
    ExpressionReference, Identifier, Internal, InternalKind, Number, Program, Uri,
};
use crate::scanner::LexicalError;
use crate::{Location, Scanner, Token, TokenKind};
pub use error::SyntacticalError;
use precedence::{ParseFunction, ParseRule, Precedence};
use std::collections::HashMap;

pub trait ErrorReporter {
    fn report(&mut self, error: SyntacticalError);
}

impl ErrorReporter for Vec<SyntacticalError> {
    fn report(&mut self, error: SyntacticalError) {
        self.push(error)
    }
}

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
pub struct Parser<'a, Iterator, Reporter> {
    reporter: Reporter,
    tokens: Iterator,
    rules: HashMap<TokenKind, ParseRule<'a, Iterator, Reporter>>,
    current: Option<Token<'a>>,
    program: Program<'a>,
    children: Vec<ExpressionReference>,
    end_location: Location,
    had_error: bool,
}

type SyntacticalResult<Output> = Result<Output, SyntacticalError>;
static OPERATOR_MAPPINGS: &[(TokenKind, InternalKind)] = &[
    (TokenKind::Equal, InternalKind::Equality),
    (TokenKind::Plus, InternalKind::Add),
    (TokenKind::Minus, InternalKind::Subtract),
    (TokenKind::Star, InternalKind::Multiply),
    (TokenKind::Slash, InternalKind::Divide),
    (TokenKind::Caret, InternalKind::Power),
    (TokenKind::Percent, InternalKind::Modulo),
    (TokenKind::NotEqual, InternalKind::Inequality),
    (TokenKind::LessThan, InternalKind::LessThan),
    (
        TokenKind::LessThanOrEqualTo,
        InternalKind::LessThanOrEqualTo,
    ),
    (TokenKind::GreaterThan, InternalKind::GreaterThan),
    (
        TokenKind::GreaterThanOrEqualTo,
        InternalKind::GreaterThanOrEqualTo,
    ),
];

impl<'a, I, R> Parser<'a, I, R>
where
    I: Iterator<Item = Result<Token<'a>, LexicalError>>,
    R: ErrorReporter,
{
    fn new<II>(tokens: II, reporter: R) -> Self
    where
        II: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        Parser {
            reporter,
            tokens: tokens.into_iter(),
            rules: Self::rules(),
            current: None,
            program: Program::default(),
            children: Vec::default(),
            end_location: Location::default(),
            had_error: false,
        }
    }

    /// Generate a [`Program`] syntax tree for this [`Parser`]'s sequence of [`Token`]s.
    pub fn parse(mut self) -> Result<Program<'a>, R> {
        self.advance();

        while self.current.is_some() {
            match self.parse_statement() {
                Ok(expression) => self.program.mark_root(expression),
                Err(error) => self.synchronize(error),
            }
        }

        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(self.program)
        }
    }

    fn synchronize(&mut self, error: SyntacticalError) {
        self.report_error(error);

        loop {
            if self.current.is_some() {
                self.advance();
            }

            if self.current.is_none() || self.check(TokenKind::Semicolon) {
                return;
            }
        }
    }

    fn parse_statement(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_expression_statement()
    }

    fn parse_expression_statement(&mut self) -> SyntacticalResult<ExpressionReference> {
        let expression = self.parse_expression()?;

        self.consume_conditionally(TokenKind::Semicolon);

        Ok(expression)
    }

    fn parse_expression(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_precedence(Precedence::Comparison)?;
        self.pop_child("Child expression stack is empty.")
    }

    fn parse_precedence(
        &mut self,
        precedence: Precedence,
    ) -> SyntacticalResult<ExpressionReference> {
        let mut kind = self.current_kind()?;
        let mut lhs = self.rule_prefix(&kind)?(self)?;

        self.children.push(lhs);

        while self.current.is_some() && precedence <= self.current_precedence() {
            kind = self.current_kind()?;
            lhs = self.rule_infix(&kind)?(self)?;

            self.children.push(lhs);
        }

        Ok(lhs)
    }

    fn parse_binary(&mut self) -> SyntacticalResult<ExpressionReference> {
        let (kind, operator) = OPERATOR_MAPPINGS
            .iter()
            .find(|(kind, _)| self.consume_conditionally(*kind))
            .ok_or_else(|| {
                SyntacticalError::new("Unsupported binary token.", self.current_location())
            })?;

        let precedence = self.rule_precedence(kind).next();

        self.parse_precedence(precedence)?;

        let right = self.pop_child("Binary operator must have a right-hand side expression")?;
        let left = self.pop_child("Binary operator must have a left-hand side expression")?;
        let operation = Internal::new(*operator, vec![left, right]);

        Ok(self.program.insert(operation))
    }

    fn parse_negation(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(TokenKind::Minus, "Expected a unary '-' sign.")?;

        let token = self.consume(TokenKind::Number, "Expected a number after the '-' sign.")?;
        let number = Number::negative(token.lexeme());

        Ok(self.program.insert(number))
    }

    fn parse_grouping(&mut self) -> SyntacticalResult<ExpressionReference> {
        let parts = self.parse_grouping_children(Vec::default())?;
        let grouping = Internal::new(InternalKind::Grouping, parts);

        Ok(self.program.insert(grouping))
    }

    fn parse_call(&mut self) -> SyntacticalResult<ExpressionReference> {
        let callee = self.pop_child("Function call must have a callee.")?;
        let mut children = self.parse_grouping_children(vec![callee])?;

        if self.check(TokenKind::Question) {
            let conditions = self.parse_conditions()?;

            children.push(conditions);
        }

        let call = Internal::new(InternalKind::Call, children);

        Ok(self.program.insert(call))
    }

    fn parse_conditions(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(
            TokenKind::Question,
            "Expected '?' before list of conditions",
        )?;

        let children = self.parse_grouping_children(Vec::default())?;
        let condition = Internal::new(InternalKind::Condition, children);

        Ok(self.program.insert(condition))
    }

    fn parse_grouping_children(
        &mut self,
        mut parts: Vec<ExpressionReference>,
    ) -> SyntacticalResult<Vec<ExpressionReference>> {
        self.consume(TokenKind::LeftParenthesis, "Expected '('.")?;

        while !self.check(TokenKind::RightParenthesis) {
            let inner = self.parse_expression()?;

            parts.push(inner);

            if !self.consume_conditionally(TokenKind::Comma) {
                break;
            }
        }

        self.consume(
            TokenKind::RightParenthesis,
            "Expected ')' after expression list.",
        )?;

        Ok(parts)
    }

    fn parse_block(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(TokenKind::LeftBracket, "Expected '['.")?;

        let mut statements = Vec::default();

        while !self.check(TokenKind::RightBracket) {
            let statement = self.parse_statement()?;

            statements.push(statement);
        }

        self.consume(TokenKind::RightBracket, "Expected ']' after block.")?;

        let block = Internal::new(InternalKind::Block, statements);

        Ok(self.program.insert(block))
    }

    fn parse_number(&mut self) -> SyntacticalResult<ExpressionReference> {
        let token = self.consume(TokenKind::Number, "Expected a number.")?;
        let number = Number::positive(token.lexeme());

        Ok(self.program.insert(number))
    }

    fn parse_identifier(&mut self) -> SyntacticalResult<ExpressionReference> {
        let token = self.consume(TokenKind::Identifier, "Expected an identifier.")?;
        let identifier = Identifier::from(token.lexeme());

        Ok(self.program.insert(identifier))
    }

    fn parse_uri(&mut self) -> SyntacticalResult<ExpressionReference> {
        let token = self.consume(TokenKind::Uri, "Expected an URI.")?;
        let uri = Uri::from(token.lexeme());

        Ok(self.program.insert(uri))
    }

    fn advance(&mut self) {
        self.current = self.next_token();
        if let Some(token) = &self.current {
            self.end_location = token.end();
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            match self.tokens.next()? {
                Ok(token) => return Some(token),
                Err(error) => self.report_error(error.into()),
            }
        }
    }

    fn report_error(&mut self, error: SyntacticalError) {
        self.had_error = true;
        self.reporter.report(error);
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> SyntacticalResult<Token<'a>> {
        match self.current {
            Some(token) if token.kind() == &kind => {
                self.advance();
                Ok(token)
            }
            Some(ref token) => Err(SyntacticalError::new(message, *token.start())),
            None => Err(SyntacticalError::new(message, self.end_location)),
        }
    }

    fn consume_conditionally(&mut self, kind: TokenKind) -> bool {
        let same = self.check(kind);

        if same {
            self.advance();
        }

        same
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        match self.current {
            Some(token) => token.kind() == &kind,
            None => false,
        }
    }

    fn current_kind(&mut self) -> SyntacticalResult<TokenKind> {
        let token = self
            .current
            .ok_or_else(|| SyntacticalError::new("Expected current token.", self.end_location))?;

        Ok(*token.kind())
    }

    fn current_precedence(&mut self) -> Precedence {
        self.current
            .as_ref()
            .map(|t| t.kind())
            .and_then(|k| self.rules.get(k))
            .map(|r| r.precedence())
            .unwrap_or_default()
    }

    fn current_location(&mut self) -> Location {
        self.current
            .map(|t| *t.start())
            .unwrap_or(self.end_location)
    }

    fn pop_child(&mut self, message: &str) -> Result<ExpressionReference, SyntacticalError> {
        self.children
            .pop()
            .ok_or_else(|| SyntacticalError::new(message, self.current_location()))
    }

    fn rule_precedence(&mut self, kind: &TokenKind) -> Precedence {
        self.rules
            .get(kind)
            .map(|r| r.precedence())
            .unwrap_or_default()
    }

    fn rule_prefix(&mut self, kind: &TokenKind) -> SyntacticalResult<ParseFunction<'a, I, R>> {
        let location = self.current_location();
        let rule = self.rules.get(kind).ok_or_else(|| {
            SyntacticalError::new("No parse rule for the current token.", location)
        })?;
        rule.prefix()
            .copied()
            .ok_or_else(|| SyntacticalError::new("Unable to parse prefix token.", location))
    }

    fn rule_infix(&mut self, kind: &TokenKind) -> SyntacticalResult<ParseFunction<'a, I, R>> {
        let location = self.current_location();
        let rule = self.rules.get(kind).ok_or_else(|| {
            SyntacticalError::new("No parse rule for the current token.", location)
        })?;

        rule.infix()
            .copied()
            .ok_or_else(|| SyntacticalError::new("Unable to parse infix token.", location))
    }

    fn rules() -> HashMap<TokenKind, ParseRule<'a, I, R>> {
        HashMap::from([
            (TokenKind::Number, ParseRule::new_prefix(Self::parse_number)),
            (
                TokenKind::Identifier,
                ParseRule::new_prefix(Self::parse_identifier),
            ),
            (TokenKind::Uri, ParseRule::new_prefix(Self::parse_uri)),
            (
                TokenKind::Caret,
                ParseRule::new_infix(Self::parse_binary, Precedence::Power),
            ),
            (
                TokenKind::Star,
                ParseRule::new_infix(Self::parse_binary, Precedence::Factor),
            ),
            (
                TokenKind::LeftParenthesis,
                ParseRule::new(Self::parse_grouping, Self::parse_call, Precedence::Call),
            ),
            (
                TokenKind::LeftBracket,
                ParseRule::new_prefix(Self::parse_block),
            ),
            (
                TokenKind::Minus,
                ParseRule::new(Self::parse_negation, Self::parse_binary, Precedence::Term),
            ),
            (
                TokenKind::Plus,
                ParseRule::new_infix(Self::parse_binary, Precedence::Term),
            ),
            (
                TokenKind::Percent,
                ParseRule::new_infix(Self::parse_binary, Precedence::Modulo),
            ),
            (
                TokenKind::Equal,
                ParseRule::new_infix(Self::parse_binary, Precedence::Comparison),
            ),
            (
                TokenKind::LessThan,
                ParseRule::new_infix(Self::parse_binary, Precedence::Comparison),
            ),
            (
                TokenKind::GreaterThan,
                ParseRule::new_infix(Self::parse_binary, Precedence::Comparison),
            ),
            (
                TokenKind::Slash,
                ParseRule::new_infix(Self::parse_binary, Precedence::Factor),
            ),
            (
                TokenKind::NotEqual,
                ParseRule::new_infix(Self::parse_binary, Precedence::Comparison),
            ),
            (
                TokenKind::LessThanOrEqualTo,
                ParseRule::new_infix(Self::parse_binary, Precedence::Comparison),
            ),
            (
                TokenKind::GreaterThanOrEqualTo,
                ParseRule::new_infix(Self::parse_binary, Precedence::Comparison),
            ),
        ])
    }
}

impl<'a> TryFrom<&'a str> for Program<'a> {
    type Error = Vec<SyntacticalError>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let scanner = Scanner::from(input);
        let parser = Parser::new(scanner, Vec::new());

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Internal, InternalKind, Number};

    #[test]
    fn math() {
        let input = "-3 + 2 + 1";
        let program = Program::try_from(input).unwrap();

        let mut expected = Program::default();

        let left = Number::negative("3");
        let left_index = expected.insert(left);

        let middle = Number::positive("2");
        let middle_index = expected.insert(middle);

        let inner_add = Internal::new(InternalKind::Add, vec![left_index, middle_index]);
        let inner_add_index = expected.insert(inner_add);

        let right = Number::positive("1");
        let right_index = expected.insert(right);

        let add = Internal::new(InternalKind::Add, vec![inner_add_index, right_index]);
        let add_index = expected.insert(add);

        expected.mark_root(add_index);

        assert_eq!(program, expected);
    }

    #[test]
    fn functions() {
        let input = "f(x) = x * x\nf(2)";
        let program = Program::try_from(input).unwrap();

        let mut expected = Program::default();

        let function = Identifier::from("f");
        let function_index = expected.insert(function);

        let parameter = Identifier::from("x");
        let parameter_index = expected.insert(parameter);

        let declaration = Internal::new(InternalKind::Call, vec![function_index, parameter_index]);
        let declaration_index = expected.insert(declaration);

        let left_index = expected.insert(parameter);
        let right_index = expected.insert(parameter);

        let multiply = Internal::new(InternalKind::Multiply, vec![left_index, right_index]);
        let multiply_index = expected.insert(multiply);

        let equality = Internal::new(
            InternalKind::Equality,
            vec![declaration_index, multiply_index],
        );
        let equality_index = expected.insert(equality);

        let invocation_index = expected.insert(function);
        let argument_index = expected.insert(Number::positive("2"));
        let call = Internal::new(InternalKind::Call, vec![invocation_index, argument_index]);
        let call_index = expected.insert(call);

        expected.mark_root(equality_index);
        expected.mark_root(call_index);

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_number() {
        let input = "2";
        let program = Program::try_from(input).unwrap();

        let mut expected = Program::default();

        let number = Number::positive("2");
        let index = expected.insert(number);

        expected.mark_root(index);

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_with_panic() {
        let result = Program::try_from("+x");

        assert!(result.is_err());
    }

    #[test]
    fn parse_negative_number() {
        let input = "-3";
        let program = Program::try_from(input).unwrap();

        let mut expected = Program::default();

        let number = Number::negative("3");
        let index = expected.insert(number);

        expected.mark_root(index);

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_identifier() {
        let input = "xyz; This is a comment.";
        let program = Program::try_from(input).unwrap();

        let mut expected = Program::default();

        let identifier = Identifier::from("xyz");
        let index = expected.insert(identifier);

        expected.mark_root(index);

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_simple() {
        assert!(Program::try_from(include_str!("../../../examples/simple.ta")).is_ok());
    }

    #[test]
    fn parse_factorial() {
        assert!(Program::try_from(include_str!("../../../examples/factorial.ta")).is_ok());
    }

    #[test]
    fn parse_bad() {
        assert!(Program::try_from(include_str!("../../../examples/bad.ta")).is_err());
    }
}
