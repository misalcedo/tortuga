//! Compiler errors.

use crate::compile::token::{InvalidToken, Kind, ValidToken};
use thiserror::Error;

/// An error that occurred during lexical analysis while validating a lexem.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum LexicalError {
    #[error("Expected a digit (e.g. 0-9, a-z, A-Z) but none was found.")]
    ExpectedDigits,
    #[error("Numeric literal has more than 1 decimal point.")]
    DuplicateDecimal,
    #[error("An identifier was found ending with an underscore .")]
    TerminalUnderscore,
    #[error(
        "Radix of {0} is too large; maximum supported is {}.",
        crate::grammar::MAX_RADIX
    )]
    RadixTooLarge(u32),
    #[error(
        "Fraction contains {0} digits, but the maximum supported is {}.",
        u32::MAX
    )]
    FractionTooLong(usize),
    #[error("Unable to parse the integer portion of a numeric literal.")]
    InvalidInteger(#[source] std::num::ParseIntError),
    #[error("Unable to parse the fraction portion of a numeric literal.")]
    InvalidFraction(#[source] std::num::ParseIntError),
    #[error("Unable to parse the radix of a numeric literal.")]
    InvalidRadix(#[source] std::num::ParseIntError),
    #[error("Encountered an unexpected character while scanning for an identifier.")]
    UnexpectedCharacter,
}

/// A syntactal error that occurs when no grammar rule matches a sequence of lexical tokens.
#[derive(Error, Debug, PartialEq)]
pub enum SyntaxError<'source> {
    #[error("Reached the end of the source code while parsing a grammar rule. Expected a token with one of following types: {0:?}.")]
    IncompleteRule(Vec<Kind>),
    #[error("No grammar rule found to match the {0}. Expected a token with one of following types: {1:?}.")]
    NoMatchingRule(ValidToken<'source>, Vec<Kind>),
    #[error("One or more lexical errors were identified for the {0}.")]
    InvalidToken(InvalidToken<'source>),
}

/// An error that occurred while parsing a stream of tokens.
#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Expected a token, but reached the end of the file.")]
    EndOfFile,
    #[error("One or more syntax errors found while parsing the source code.")]
    MultipleErrors,
}
