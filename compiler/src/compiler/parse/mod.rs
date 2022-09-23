//! Parse a sequence of tokens into a syntax tree.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the nesting of the parsing functions to denote each precedence level.

mod error;
mod precedence;

use crate::collections::Tree;
use crate::compiler::scan::LexicalError;
use crate::compiler::{CompilationError, ErrorReporter, Excerpt};
use crate::compiler::{Location, Scanner, Token, TokenKind};
use crate::grammar::{Expression, ExpressionKind, SyntaxTree};
pub use error::{SyntaxError, SyntaxErrorKind};
use precedence::{InfixParseFunction, ParseRule, Precedence, PrefixParseFunction};
use std::collections::HashMap;

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
pub struct Parser<'a, Iterator, Reporter> {
    reporter: Reporter,
    tokens: Iterator,
    source: &'a str,
    rules: HashMap<TokenKind, ParseRule<'a, Iterator, Reporter>>,
    current: Option<Token<'a>>,
    end: Location,
}

type SyntacticalResult<Output> = Result<Output, SyntaxError>;
type ParseResult<'a> = Result<Tree<Expression<'a>>, SyntaxError>;
static OPERATOR_MAPPINGS: &[(TokenKind, ExpressionKind)] = &[
    (TokenKind::Equal, ExpressionKind::Equality),
    (TokenKind::Plus, ExpressionKind::Add),
    (TokenKind::Minus, ExpressionKind::Subtract),
    (TokenKind::Star, ExpressionKind::Multiply),
    (TokenKind::Slash, ExpressionKind::Divide),
    (TokenKind::Caret, ExpressionKind::Power),
    (TokenKind::Percent, ExpressionKind::Modulo),
    (TokenKind::NotEqual, ExpressionKind::Inequality),
    (TokenKind::LessThan, ExpressionKind::LessThan),
    (
        TokenKind::LessThanOrEqualTo,
        ExpressionKind::LessThanOrEqualTo,
    ),
    (TokenKind::GreaterThan, ExpressionKind::GreaterThan),
    (
        TokenKind::GreaterThanOrEqualTo,
        ExpressionKind::GreaterThanOrEqualTo,
    ),
];

impl<'a, I, R> Parser<'a, I, R>
where
    I: Iterator<Item = Result<Token<'a>, LexicalError>>,
    R: ErrorReporter,
{
    pub fn new<II>(source: &'a str, tokens: II, reporter: R) -> Self
    where
        II: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        Parser {
            reporter,
            source,
            tokens: tokens.into_iter(),
            rules: Self::rules(),
            current: None,
            end: Location::default(),
        }
    }

    /// Generate a [`Program`] syntax tree for this [`Parser`]'s sequence of [`Token`]s.
    pub fn parse(mut self) -> Result<SyntaxTree<'a>, R> {
        let mut program = SyntaxTree::new(self.source);

        self.advance();

        while self.current.is_some() {
            if self.consume_conditionally(TokenKind::Semicolon) {
                continue;
            }

            match self.parse_statement() {
                Ok(expression) => {
                    program.insert(expression);
                }
                Err(error) => self.synchronize(error),
            }
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(program)
        }
    }

    fn synchronize(&mut self, error: SyntaxError) {
        self.reporter.report_syntax_error(error);

        loop {
            self.advance();

            if self.current.is_none() || self.check(TokenKind::Semicolon) {
                return;
            }
        }
    }

    fn parse_statement(&mut self) -> ParseResult<'a> {
        self.parse_expression_statement()
    }

    fn parse_expression_statement(&mut self) -> ParseResult<'a> {
        let expression = self.parse_expression()?;

        self.consume_conditionally(TokenKind::Semicolon);

        Ok(expression)
    }

    fn parse_expression(&mut self) -> ParseResult<'a> {
        self.parse_precedence(Precedence::Comparison)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> ParseResult<'a> {
        let start = self.current_location();
        let mut kind = self.current_kind()?;
        let mut lhs = self.rule_prefix(&kind)?(self)?;

        while self.current.is_some() && precedence <= self.current_precedence() {
            kind = self.current_kind()?;
            lhs = self.rule_infix(&kind)?(self, start, lhs)?;
        }

        Ok(lhs)
    }

    fn parse_binary(&mut self, start: Location, lhs: Tree<Expression<'a>>) -> ParseResult<'a> {
        let (kind, operator) = OPERATOR_MAPPINGS
            .iter()
            .find(|(kind, _)| self.consume_conditionally(*kind))
            .ok_or_else(|| {
                SyntaxError::new(
                    SyntaxErrorKind::UnsupportedBinaryToken,
                    self.current_excerpt(),
                )
            })?;

        let precedence = self.rule_precedence(kind).next();
        let rhs = self.parse_precedence(precedence)?;
        let expression = Expression::new(*operator, &self.source[&Excerpt::from(start..self.end)]);
        let binary = Tree::new(expression, vec![lhs, rhs]);

        Ok(binary)
    }

    fn parse_negation(&mut self) -> ParseResult<'a> {
        let sign = self.consume(TokenKind::Minus)?;
        let number = self.consume(TokenKind::Number)?;
        let excerpt = Excerpt::from(*sign.start()..number.end());

        if number.lexeme() == "0" {
            Err(SyntaxError::new(SyntaxErrorKind::CannotNegateZero, excerpt))
        } else {
            let expression = Expression::new(ExpressionKind::Number, &self.source[&excerpt]);

            Ok(Tree::from(expression))
        }
    }

    fn parse_grouping(&mut self) -> ParseResult<'a> {
        let start = *self.consume(TokenKind::LeftParenthesis)?.start();
        let mut children = vec![];

        while !self.check(TokenKind::RightParenthesis) {
            let inner = self.parse_expression()?;

            children.push(inner);

            if !self.consume_conditionally(TokenKind::Comma) {
                break;
            }
        }

        let end = self.consume(TokenKind::RightParenthesis)?.end();
        let excerpt = &self.source[&Excerpt::from(start..end)];
        let expression = Expression::new(ExpressionKind::Grouping, excerpt);
        let mut grouping = Tree::from(expression);

        for child in children {
            grouping.insert(child);
        }

        Ok(grouping)
    }

    fn parse_call(&mut self, start: Location, callee: Tree<Expression<'a>>) -> ParseResult<'a> {
        let arguments = self.parse_grouping()?;
        let mut children = vec![callee, arguments];

        if self.check(TokenKind::Question) {
            let condition = self.parse_condition()?;

            children.push(condition);
        }

        let mut call = Tree::from(Expression::new(
            ExpressionKind::Call,
            &self.source[&Excerpt::from(start..self.end)],
        ));

        for child in children {
            call.insert(child);
        }

        Ok(call)
    }

    fn parse_condition(&mut self) -> ParseResult<'a> {
        let start = *self.consume(TokenKind::Question)?.start();

        let mut condition = self.parse_grouping()?;
        let lexeme = &self.source[&Excerpt::from(start..self.end)];
        let expression = Expression::new(ExpressionKind::Condition, lexeme);

        *condition.data_mut() = expression;

        Ok(condition)
    }

    fn parse_block(&mut self) -> ParseResult<'a> {
        let start = *self.consume(TokenKind::LeftBracket)?.start();
        let mut statements = vec![];

        while !self.check(TokenKind::RightBracket) {
            let statement = self.parse_statement()?;

            statements.push(statement);
        }

        let end = self.consume(TokenKind::RightBracket)?.end();
        let mut block = Tree::from(Expression::new(
            ExpressionKind::Block,
            &self.source[&Excerpt::from(start..end)],
        ));

        for child in statements {
            block.insert(child);
        }

        Ok(block)
    }

    fn parse_number(&mut self) -> ParseResult<'a> {
        let token = self.consume(TokenKind::Number)?;
        let expression = Expression::new(ExpressionKind::Number, token.lexeme());

        Ok(Tree::from(expression))
    }

    fn parse_identifier(&mut self) -> ParseResult<'a> {
        let token = self.consume(TokenKind::Identifier)?;
        let expression = Expression::new(ExpressionKind::Identifier, token.lexeme());

        Ok(Tree::from(expression))
    }

    fn parse_uri(&mut self) -> ParseResult<'a> {
        let token = self.consume(TokenKind::Uri)?;
        let expression = Expression::new(ExpressionKind::Uri, token.lexeme());

        Ok(Tree::from(expression))
    }

    fn advance(&mut self) {
        if let Some(current) = self.current {
            self.end = current.end();
        }

        self.current = self.next_token();
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            match self.tokens.next()? {
                Ok(token) => return Some(token),
                Err(error) => {
                    self.reporter.report_lexical_error(error);
                }
            }
        }
    }

    fn consume(&mut self, kind: TokenKind) -> SyntacticalResult<Token<'a>> {
        match self.current {
            Some(token) if token.kind() == &kind => {
                self.advance();
                Ok(token)
            }
            Some(ref token) => Err(SyntaxError::new(
                SyntaxErrorKind::ExpectedKind(kind),
                Excerpt::from(*token.start()..token.end()),
            )),
            None => Err(SyntaxError::new(
                SyntaxErrorKind::ExpectedKind(kind),
                Excerpt::from(self.end..),
            )),
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
        let token = self.current.ok_or_else(|| {
            SyntaxError::new(
                SyntaxErrorKind::ExpectedCurrentToken,
                Excerpt::from(self.end..),
            )
        })?;

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
            .as_ref()
            .map(Token::start)
            .copied()
            .unwrap_or_default()
    }

    fn current_excerpt(&mut self) -> Excerpt {
        match self.current {
            None => Excerpt::from(self.end..),
            Some(current) => Excerpt::from(*current.start()..self.end),
        }
    }

    fn rule_precedence(&mut self, kind: &TokenKind) -> Precedence {
        self.rules
            .get(kind)
            .map(|r| r.precedence())
            .unwrap_or_default()
    }

    fn rule_prefix(
        &mut self,
        kind: &TokenKind,
    ) -> SyntacticalResult<PrefixParseFunction<'a, I, R>> {
        let location = self.current_excerpt();
        let rule = self
            .rules
            .get(kind)
            .ok_or_else(|| SyntaxError::new(SyntaxErrorKind::NoParseRule, location))?;
        rule.prefix()
            .copied()
            .ok_or_else(|| SyntaxError::new(SyntaxErrorKind::InvalidPrefixToken, location))
    }

    fn rule_infix(&mut self, kind: &TokenKind) -> SyntacticalResult<InfixParseFunction<'a, I, R>> {
        let location = self.current_excerpt();
        let rule = self
            .rules
            .get(kind)
            .ok_or_else(|| SyntaxError::new(SyntaxErrorKind::NoParseRule, location))?;

        rule.infix()
            .copied()
            .ok_or_else(|| SyntaxError::new(SyntaxErrorKind::InvalidInfixToken, location))
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

impl<'a> From<&'a str> for Parser<'a, Scanner<'a>, Vec<CompilationError>> {
    fn from(source: &'a str) -> Self {
        Parser::new(source, Scanner::from(source), vec![])
    }
}

impl<'a> TryFrom<&'a str> for SyntaxTree<'a> {
    type Error = Vec<CompilationError>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let parser = Parser::from(input);

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math() {
        let input = "-3 + 2 + 1";
        let program = SyntaxTree::try_from(input).unwrap();

        let mut expected = SyntaxTree::new(input);

        let mut left = Tree::from(Expression::new(ExpressionKind::Add, "-3 + 2"));
        left.insert(Expression::new(ExpressionKind::Number, "-3"));
        left.insert(Expression::new(ExpressionKind::Number, "2"));

        let right = expected.insert(Expression::new(ExpressionKind::Add, "-3 + 2 + 1"));
        right.insert(left);
        right.insert(Expression::new(ExpressionKind::Number, "1"));

        assert_eq!(program, expected);
    }

    #[test]
    fn functions() {
        let mut expected = SyntaxTree::new("f(x) = x * x\nf(2)");

        let assignment = expected.insert(Expression::new(ExpressionKind::Equality, "f(x) = x * x"));
        let declaration = assignment.insert(Expression::new(ExpressionKind::Call, "f(x)"));
        let function = declaration.insert(Expression::new(ExpressionKind::Identifier, "f"));
        let parameters = declaration.insert(Expression::new(ExpressionKind::Grouping, "(x)"));
        let parameter = parameters.insert(Expression::new(ExpressionKind::Identifier, "x"));

        let multiply = assignment.insert(Expression::new(ExpressionKind::Multiply, "x * x"));
        let lhs = multiply.insert(Expression::new(ExpressionKind::Identifier, "x"));
        let rhs = multiply.insert(Expression::new(ExpressionKind::Identifier, "x"));

        let invocation = expected.insert(Expression::new(ExpressionKind::Call, "f(2)"));
        let callee = invocation.insert(Expression::new(ExpressionKind::Identifier, "f"));
        let arguments = invocation.insert(Expression::new(ExpressionKind::Grouping, "(2)"));
        let argument = arguments.insert(Expression::new(ExpressionKind::Number, "2"));

        let actual = SyntaxTree::try_from(expected.as_str()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_number() {
        let input = "2";
        let program = SyntaxTree::try_from(input).unwrap();

        let mut expected = SyntaxTree::new(input);
        expected.insert(Expression::new(ExpressionKind::Number, "2"));

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_with_panic() {
        let result = SyntaxTree::try_from("+x");

        assert!(result.is_err());
    }

    #[test]
    fn parse_negative_number() {
        let input = "-3";
        let program = SyntaxTree::try_from(input).unwrap();

        let mut expected = SyntaxTree::new(input);
        expected.insert(Expression::new(ExpressionKind::Number, "-3"));

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_negative_zero() {
        assert_eq!(SyntaxTree::try_from("-0").unwrap_err().len(), 1);
    }

    #[test]
    fn parse_identifier() {
        let input = "xyz; This is a comment.";
        let program = SyntaxTree::try_from(input).unwrap();

        let mut expected = SyntaxTree::new(input);
        expected.insert(Expression::new(ExpressionKind::Identifier, "xyz"));

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_simple() {
        assert!(
            !SyntaxTree::try_from(include_str!("../../../../examples/simple.ta"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn parse_factorial() {
        assert!(
            !SyntaxTree::try_from(include_str!("../../../../examples/factorial.ta"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn parse_bad() {
        assert!(
            !SyntaxTree::try_from(include_str!("../../../../examples/bad.ta"))
                .unwrap_err()
                .is_empty()
        );
    }
}
