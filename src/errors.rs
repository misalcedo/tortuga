use crate::token::{Location, Token, TokenKind};
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
    #[error("A lexical error occurred while analyzing the source code. {0}")]
    Lexical(#[from] LexicalError),
    #[error("A syntax error occurred while parsing the source code. {0}")]
    Syntax(#[from] SyntaxError),
}

/// An error that occurred during lexical analysis.
#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Incomplete grapheme found in source code.")]
    IncompleteGrapheme(Location, unicode_segmentation::GraphemeIncomplete),
    #[error("An unexpected grapheme was found on {0}: {1}.")]
    UnexpectedGrapheme(Location, String),
    #[error("Expected a number (0-9) but none was found on {0}.")]
    ExpectedDigits(Location),
    #[error("A numeric literal was found with more than 1 decimal point on {0}: {1}.")]
    DuplicateDecimal(Location, String),
    #[error("A numeric literal is missing the radix on {0}: {1}.")]
    MissingRadix(Location, String),
    #[error("A text reference is missing the closing quote on {0}: {1}.")]
    MissingClosingQuote(Location, String),
    #[error("A text reference is empty on {0}.")]
    BlankTextReference(Location),
    #[error("An identifier was found ending with an underscore on {0}: {1}.")]
    TerminalUnderscore(Location, String),
}

/// An error that occurred while parsing a stream of tokens.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Unable to parse the numeric literal '{0}' on {1}.")]
    InvalidNumber(String, Location),
    #[error("Unable to parse the numeric literal '{1}' on {2}. Radix of {0} is too large; maximum supported is 36.")]
    RadixTooLarge(u32, String, Location),
    #[error("Unable to parse the numeric literal '{1}' on {2}. Fraction contains {0} digits, but the maximum supported is `u32::MAX`.")]
    FractionTooLong(usize, String, Location),
    #[error("Unable to parse the integer portion of a numeric literal '{1}' on {2}.")]
    InvalidInteger(#[source] std::num::ParseIntError, String, Location),
    #[error("Unable to parse the fraction portion of a numeric literal '{1}' on {2}.")]
    InvalidFraction(#[source] std::num::ParseIntError, String, Location),
    #[error("Unable to parse the radix of a numeric literal '{1}' on {2}.")]
    InvalidRadix(#[source] std::num::ParseIntError, String, Location),
    #[error("Expected token of type {0:?}, but found a lexical error. {1}")]
    Lexical(Vec<TokenKind>, #[source] LexicalError),
    #[error("Expected token '{lexeme}' ({actual}) on {location} to be of type {expected:?}.")]
    MismatchKind {
        location: Location,
        expected: Vec<TokenKind>,
        actual: TokenKind,
        lexeme: String,
    },
    #[error("Expected a token with type: {0:?}. Instead, reached the end of the file.")]
    EndOfFile(Vec<TokenKind>),
}

impl SyntaxError {
    /// Creates an error for mismatched token kinds.
    pub fn mismatched_kind(expected: &[TokenKind], token: Option<Result<Token<'_>, LexicalError>>) -> Self {
        match token {
            Some(Ok(token)) => SyntaxError::MismatchKind {
                location: token.start(),
                expected: expected.to_vec(),
                actual: token.kind(),
                lexeme: token.lexeme().to_string(),
            },
            Some(Err(error)) => SyntaxError::Lexical(expected.to_vec(), error),
            None => SyntaxError::EndOfFile(expected.to_vec())
        }
    }
}
