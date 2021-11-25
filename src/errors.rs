use crate::grammar::ComparisonOperator;
use crate::token::{Location, Token, TokenKind};
use std::fmt;
use thiserror::Error;

/// An error that occurred while interacting with Tortuga.
#[derive(Error, Debug)]
pub enum TortugaError {
    #[error("An IO error occurred.")]
    IO(#[from] std::io::Error),
    #[error("Unable to set global default tracing collector.")]
    Tracing(#[from] tracing::dispatcher::SetGlobalDefaultError),
    #[error("Unable to set log tracing redirection.")]
    Logging(#[from] tracing_log::log_tracer::SetLoggerError),
    #[error("Unable to walk the input directory.")]
    Walk(#[from] walkdir::Error),
    #[error("Unable to remove the input path from the file name.")]
    InvalidPath(#[from] std::path::StripPrefixError),
    #[error("A runtime error occurred while interpreting the source code. {0}")]
    Runtime(#[from] RuntimeError),
    #[error("Encountered an error prompting the user for input. {0}")]
    PromptError(#[from] rustyline::error::ReadlineError),
}

/// An error that occurred while interpreting a Tortuga expression.
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("An unknown runtime error.")]
    Unknown,
    #[error("Expected value of type {expected}, but found {actual}.")]
    InvalidType { expected: String, actual: String },
    #[error("Unable to determine whether {left} {comparison} {right}.")]
    NotComparable {
        left: String,
        comparison: ComparisonOperator,
        right: String,
    },
}

impl RuntimeError {
    pub fn invalid_type(expected: impl fmt::Display, actual: impl fmt::Display) -> Self {
        Self::InvalidType {
            expected: format!("{}", expected),
            actual: format!("{}", actual),
        }
    }

    pub fn not_comparable(
        left: impl fmt::Display,
        comparison: ComparisonOperator,
        right: impl fmt::Display,
    ) -> Self {
        Self::NotComparable {
            left: format!("{}", left),
            comparison,
            right: format!("{}", right),
        }
    }
}

/// An error that occurred during lexical analysis while validating a lexem.
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Expected a digit (e.g. 0-9, a-z, A-Z) but none was found.")]
    ExpectedDigits,
    #[error("Numeric literal has more than 1 decimal point.")]
    DuplicateDecimal,
    #[error("Numeric literal is missing the radix.")]
    MissingRadix,
    #[error("Text reference is missing the closing quote.")]
    MissingClosingQuote,
    #[error("Found a blank (empty or only non-visible characters) text reference.")]
    BlankTextReference,
    #[error("An identifier was found ending with an underscore .")]
    TerminalUnderscore,
    #[error("Unable to parse the numeric literal.")]
    InvalidNumber,
    #[error("Radix of {0} is too large; maximum supported is {1}.")]
    RadixTooLarge(u32, u32),
    #[error("Fraction contains {0} digits, but the maximum supported is {1}.")]
    FractionTooLong(usize, u32),
    #[error("Unable to parse the integer portion of a numeric literal.")]
    InvalidInteger(#[source] std::num::ParseIntError),
    #[error("Unable to parse the fraction portion of a numeric literal.")]
    InvalidFraction(#[source] std::num::ParseIntError),
    #[error("Unable to parse the radix of a numeric literal.")]
    InvalidRadix(#[source] std::num::ParseIntError),
    #[error("Encountered an unexpected character while scanning for an identifier.")]
    UnexpectedCharacter,
}

/// An error that occurred while parsing a stream of tokens.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Found one or more lexical errors while scanning {kind} token '{lexeme}' on {location}: {errors}")]
    Lexical {
        location: Location,
        kind: TokenKind,
        lexeme: String,
        errors: MultipleErrors<ValidationError>,
    },
    #[error("Failed to validate the current token. {0}")]
    Validation(#[from] ValidationError),
    #[error("Expected token '{lexeme}' ({actual}) on {location} to be of type {expected}.")]
    Syntax {
        location: Location,
        expected: TokenKinds,
        actual: TokenKind,
        lexeme: String,
    },
    #[error("Expected a token with type {0}. Instead, reached the end of the file.")]
    EndOfFile(TokenKinds),
    #[error("No grammar rule was found to match the token kind {1} on {0}.")]
    NoMatchingGrammar(Location, TokenKind),
    #[error("No grammar rule was found to match the sequence of comparison operators {1} on {0}. Valid comparison operators are: <, =, >, <=, >=, <=>.")]
    InvalidComparator(Location, TokenKinds),
    #[error("One or more syntax errors found while parsing the source code. {0}")]
    MultipleErrors(MultipleErrors<ParseError>),
}

impl ParseError {
    /// Creates an error for mismatched token kinds.
    pub fn mismatched_kind(expected: &[TokenKind], token: Option<&Token<'_>>) -> Self {
        match token {
            Some(token) => Self::Syntax {
                location: token.start(),
                expected: TokenKinds(expected.to_vec()),
                actual: token.kind(),
                lexeme: token.lexeme().to_string(),
            },
            None => Self::EndOfFile(TokenKinds(expected.to_vec())),
        }
    }

    /// Creates an error for mismatched token kinds.
    pub fn multiple_errors(errors: Vec<ParseError>) -> Self {
        ParseError::MultipleErrors(MultipleErrors(errors))
    }

    /// Creates an error for mismatched token kinds.
    pub fn validate(mut token: Token<'_>) -> Result<Token<'_>, Self> {
        if token.validations().is_empty() {
            Ok(token)
        } else {
            Err(Self::Lexical {
                location: token.start(),
                kind: token.kind(),
                lexeme: token.lexeme().to_string(),
                errors: MultipleErrors(token.take_validations()),
            })
        }
    }
}

/// Wrapper struct to define Display trait.
#[derive(Debug)]
pub struct TokenKinds(Vec<TokenKind>);

impl From<Vec<TokenKind>> for TokenKinds {
    fn from(kinds: Vec<TokenKind>) -> Self {
        TokenKinds(kinds)
    }
}

impl fmt::Display for TokenKinds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iterator = self.0.iter().peekable();

        while let Some(kind) = iterator.next() {
            write!(f, "{}", kind)?;

            if iterator.peek().is_some() {
                write!(f, ", or ")?;
            }
        }

        Ok(())
    }
}

/// Wrapper struct to define Display trait.
#[derive(Debug)]
pub struct MultipleErrors<E: std::error::Error>(Vec<E>);

impl<E> fmt::Display for MultipleErrors<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iterator = self.0.iter().enumerate().peekable();

        while let Some((index, kind)) = iterator.next() {
            write!(f, "{}) {}", index + 1, kind)?;

            if iterator.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}
