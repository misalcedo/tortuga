//! Parse a sequence of tokens into a syntax tree.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the nesting of the parsing functions to denote each precedence level.

mod error;
mod precedence;

use crate::grammar::{
    ExpressionReference, Identifier, Internal, InternalKind, Number, Program, Uri,
};
use crate::scanner::LexicalError;
use crate::{Location, Token, TokenKind};
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
    can_assign: bool,
    had_error: bool,
}

type SyntacticalResult<Output> = Result<Output, SyntacticalError>;
static OPERATOR_MAPPINGS: &[(TokenKind, InternalKind)] = &[
    (TokenKind::Equal, InternalKind::Assignment),
    (TokenKind::Plus, InternalKind::Add),
    (TokenKind::Minus, InternalKind::Subtract),
    (TokenKind::Star, InternalKind::Multiply),
    (TokenKind::Slash, InternalKind::Divide),
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
            can_assign: false,
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
        self.can_assign = precedence <= Precedence::Assignment;

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

        if self.can_assign && self.consume_conditionally(TokenKind::Equal) {
            Err(SyntacticalError::new(
                "Invalid assignment target.",
                self.end_location,
            ))
        } else {
            Ok(lhs)
        }
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

                let operation = Internal::new(*operator, self.children.drain(..).collect());

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
            ParseRule::placeholder(TokenKind::Identifier),
            ParseRule::placeholder(TokenKind::Uri),
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
            ParseRule::placeholder(TokenKind::LeftParenthesis),
            ParseRule::placeholder(TokenKind::RightParenthesis),
            ParseRule::placeholder(TokenKind::Underscore),
            ParseRule::new(
                TokenKind::Minus,
                Self::parse_negation,
                Self::parse_binary,
                Precedence::Term,
            ),
            ParseRule::new_infix(TokenKind::Plus, Self::parse_binary, Precedence::Term),
            ParseRule::placeholder(TokenKind::Equal),
            ParseRule::placeholder(TokenKind::LeftBrace),
            ParseRule::placeholder(TokenKind::LeftBracket),
            ParseRule::placeholder(TokenKind::RightBrace),
            ParseRule::placeholder(TokenKind::RightBracket),
            ParseRule::placeholder(TokenKind::VerticalPipe),
            ParseRule::placeholder(TokenKind::BackSlash),
            ParseRule::placeholder(TokenKind::Colon),
            ParseRule::placeholder(TokenKind::Semicolon),
            ParseRule::placeholder(TokenKind::SingleQuote),
            ParseRule::placeholder(TokenKind::LessThan),
            ParseRule::placeholder(TokenKind::Comma),
            ParseRule::placeholder(TokenKind::GreaterThan),
            ParseRule::placeholder(TokenKind::Dot),
            ParseRule::placeholder(TokenKind::Question),
            ParseRule::new_infix(TokenKind::Slash, Self::parse_binary, Precedence::Factor),
            ParseRule::placeholder(TokenKind::NotEqual),
            ParseRule::placeholder(TokenKind::LessThanOrEqualTo),
            ParseRule::placeholder(TokenKind::GreaterThanOrEqualTo),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Internal, InternalKind, Number};
    use crate::Scanner;

    #[test]
    fn from_scanner() {
        let input = "-3 + 2 + 1";
        let scanner: Scanner<'_> = input.into();
        let parser = Parser::new(scanner, Vec::new());

        let program = parser.parse().unwrap();

        let mut expected = Program::default();

        let left = Number::negative("3");
        let left_index = expected.insert(left.clone());

        let middle = Number::positive("2");
        let middle_index = expected.insert(middle.clone());

        let inner_add = Internal::new(InternalKind::Add, vec![left_index, middle_index]);
        let inner_add_index = expected.insert(inner_add.clone());

        let right = Number::positive("1");
        let right_index = expected.insert(right.clone());

        let add = Internal::new(InternalKind::Add, vec![inner_add_index, right_index]);
        let add_index = expected.insert(add.clone());

        expected.mark_root(add_index);

        assert_eq!(program, expected);
    }
}

// impl FromStr for Program {
//     type Err = SyntacticalError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let tokens = Tokens::try_from(Scanner::from(s))?;
//         let parser = Parser::from(tokens);
//
//         parser.parse()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_number() {
//         assert!("2".parse::<Program>().is_ok());
//     }
//
//     #[test]
//     fn parse_with_panic() {
//         let result = "+x".parse::<Program>();
//
//         assert_eq!(result, Err(SyntacticalError::Multiple));
//     }
//
//     #[test]
//     fn parse_negative_number() {
//         assert!("-2#100".parse::<Program>().is_ok());
//     }
//
//     #[test]
//     fn parse_identifier() {
//         assert!("xyz".parse::<Program>().is_ok());
//     }
//
//     #[test]
//     fn parse_example() {
//         assert!(include_str!("../../../examples/example.ta")
//             .parse::<Program>()
//             .is_ok())
//     }
//
//     #[test]
//     fn parse_factorial() {
//         assert!(include_str!("../../../examples/factorial.ta")
//             .parse::<Program>()
//             .is_ok())
//     }
//
//     #[test]
//     fn parse_bad() {
//         assert!(include_str!("../../../../examples/bad.ta")
//             .parse::<Program>()
//             .is_err())
//     }
// }
