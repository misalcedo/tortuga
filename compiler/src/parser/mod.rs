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
use precedence::{ParseRule, Precedence};

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
    rules: Vec<ParseRule<'a, Iterator, Reporter>>,
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
            children: Vec::with_capacity(2),
            end_location: Location::default(),
            had_error: false,
        }
    }

    /// Generate a [`Program`] syntax tree for this [`Parser`]'s sequence of [`Token`]s.
    pub fn parse(mut self) -> Result<Program<'a>, R> {
        self.advance();

        while self.current.is_some() {
            match self.parse_expression() {
                Ok(expression) => {
                    self.children.pop();
                    self.program.mark_root(expression)
                }
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

    fn parse_expression(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(
        &mut self,
        precedence: Precedence,
    ) -> SyntacticalResult<ExpressionReference> {
        let mut token = self
            .current
            .ok_or_else(|| SyntacticalError::new("Expected current token.", self.end_location))?;
        let mut rule = &self.rules[*token.kind() as usize];
        let location = self.end_location.clone();
        let prefix = rule
            .prefix()
            .ok_or_else(|| SyntacticalError::new("Expect expression.", location))?;

        let mut lhs = prefix(self)?;

        self.children.push(lhs);

        loop {
            if self.current.is_none() {
                break;
            }

            token = self.current.ok_or_else(|| {
                SyntacticalError::new("Expected current token.", self.end_location)
            })?;
            rule = &self.rules[*token.kind() as usize];

            if precedence > rule.precedence() {
                break;
            }

            let location = self.end_location.clone();
            let infix = rule
                .infix()
                .ok_or_else(|| SyntacticalError::new("Expect expression.", location))?;

            lhs = infix(self)?;
            self.children.push(lhs);
        }

        Ok(lhs)
    }

    fn parse_binary(&mut self) -> SyntacticalResult<ExpressionReference> {
        let result = OPERATOR_MAPPINGS
            .iter()
            .find(|(kind, _)| self.consume_conditionally(*kind));

        match result {
            Some((kind, operator)) => {
                let rule = &self.rules[*kind as usize];
                let precedence = rule.precedence().next();

                self.parse_precedence(precedence)?;

                let right = self.children.pop().ok_or_else(|| {
                    SyntacticalError::new(
                        "Binary operator must have a right-hand side expression",
                        self.end_location,
                    )
                })?;
                let left = self.children.pop().ok_or_else(|| {
                    SyntacticalError::new(
                        "Binary operator must have a left-hand side expression",
                        self.end_location,
                    )
                })?;
                let operation = Internal::new(*operator, vec![left, right]);

                Ok(self.program.insert(operation))
            }
            None => Err(SyntacticalError::new(
                "Expected binary operator.",
                self.end_location,
            )),
        }
    }

    fn parse_negation(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(TokenKind::Minus, "Expected a unary '-' sign.")?;

        let token = self.consume(TokenKind::Number, "Expected a number after the '-' sign.")?;
        let number = Number::negative(token.lexeme());

        Ok(self.program.insert(number))
    }

    fn parse_call(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(TokenKind::LeftParenthesis, "Expected '('.")?;

        while !self.check(TokenKind::RightParenthesis) {
            self.parse_expression()?;

            if !self.consume_conditionally(TokenKind::Comma) {
                break;
            }
        }

        self.consume(TokenKind::RightParenthesis, "Expect ')' after arguments.")?;

        let call = Internal::new(InternalKind::Call, self.children.drain(..).collect());

        Ok(self.program.insert(call))
    }

    fn parse_grouping(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(TokenKind::LeftParenthesis, "Expected '('.")?;
        let expression = self.parse_expression()?;
        self.consume(
            TokenKind::RightParenthesis,
            "Expected ')' after expression.",
        )?;

        Ok(expression)
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

    fn rules() -> Vec<ParseRule<'a, I, R>> {
        vec![
            ParseRule::new_prefix(TokenKind::Number, Self::parse_number),
            ParseRule::new_prefix(TokenKind::Identifier, Self::parse_identifier),
            ParseRule::new_prefix(TokenKind::Uri, Self::parse_uri),
            ParseRule::placeholder(TokenKind::Tilde),
            ParseRule::placeholder(TokenKind::BackTick),
            ParseRule::placeholder(TokenKind::Exclamation),
            ParseRule::placeholder(TokenKind::At),
            ParseRule::placeholder(TokenKind::Pound),
            ParseRule::placeholder(TokenKind::Dollar),
            ParseRule::placeholder(TokenKind::Percent),
            ParseRule::placeholder(TokenKind::Caret),
            ParseRule::placeholder(TokenKind::Ampersand),
            ParseRule::new_infix(TokenKind::Star, Self::parse_binary, Precedence::Factor),
            ParseRule::new(
                TokenKind::LeftParenthesis,
                Self::parse_grouping,
                Self::parse_call,
                Precedence::Call,
            ),
            ParseRule::placeholder(TokenKind::RightParenthesis),
            ParseRule::placeholder(TokenKind::Underscore),
            ParseRule::new(
                TokenKind::Minus,
                Self::parse_negation,
                Self::parse_binary,
                Precedence::Term,
            ),
            ParseRule::new_infix(TokenKind::Plus, Self::parse_binary, Precedence::Term),
            ParseRule::new_infix(TokenKind::Equal, Self::parse_binary, Precedence::Comparison),
            ParseRule::placeholder(TokenKind::LeftBrace),
            ParseRule::placeholder(TokenKind::LeftBracket),
            ParseRule::placeholder(TokenKind::RightBrace),
            ParseRule::placeholder(TokenKind::RightBracket),
            ParseRule::placeholder(TokenKind::VerticalPipe),
            ParseRule::placeholder(TokenKind::BackSlash),
            ParseRule::placeholder(TokenKind::Colon),
            ParseRule::placeholder(TokenKind::Semicolon),
            ParseRule::placeholder(TokenKind::SingleQuote),
            ParseRule::new_infix(
                TokenKind::LessThan,
                Self::parse_binary,
                Precedence::Comparison,
            ),
            ParseRule::placeholder(TokenKind::Comma),
            ParseRule::new_infix(
                TokenKind::GreaterThan,
                Self::parse_binary,
                Precedence::Comparison,
            ),
            ParseRule::placeholder(TokenKind::Dot),
            ParseRule::placeholder(TokenKind::Question),
            ParseRule::new_infix(TokenKind::Slash, Self::parse_binary, Precedence::Factor),
            ParseRule::new_infix(
                TokenKind::NotEqual,
                Self::parse_binary,
                Precedence::Comparison,
            ),
            ParseRule::new_infix(
                TokenKind::LessThanOrEqualTo,
                Self::parse_binary,
                Precedence::Comparison,
            ),
            ParseRule::new_infix(
                TokenKind::GreaterThanOrEqualTo,
                Self::parse_binary,
                Precedence::Comparison,
            ),
        ]
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
        let input = "xyz";
        let program = Program::try_from(input).unwrap();

        let mut expected = Program::default();

        let identifier = Identifier::from("xyz");
        let index = expected.insert(identifier);

        expected.mark_root(index);

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_bad() {
        assert!(Program::try_from(include_str!("../../../examples/bad.ta")).is_err())
    }
}
