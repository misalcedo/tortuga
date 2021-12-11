use crate::grammar::ComparisonOperator;
use crate::token::{InvalidToken, Kind, ValidToken};
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
#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("A block must have at least one expression.")]
    EmptyBlock,
    #[error("Expected value of type {expected}, but found {actual}.")]
    InvalidType { expected: String, actual: String },
    #[error("Unable to determine whether {left} {comparison} {right}.")]
    NotComparable {
        left: String,
        comparison: ComparisonOperator,
        right: String,
    },
    #[error("Attempted to use variable '{0}' that is not yet defined.")]
    UndefinedVariableUsed(String),
    #[error("Attempted to refine variable '{0}' with the operator {1} and value {2}, but the refinement is not valid.")]
    InvalidRefinement(String, ComparisonOperator, f64),
    #[error("Attempted to define variable '{0}' as equal to {2}, but the variable is already equal to {1}.")]
    AlreadyDefined(String, f64, f64),
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
        crate::number::MAX_RADIX
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
    #[error("Reached the end of the source code while parsing a grammar rule (expected {0:?}).")]
    IncompleteRule(Vec<Kind>),
    #[error("No grammar rule found to match the next lexical token: {0:?}.")]
    NoMatchingRule(ValidToken<'source>, Vec<Kind>),
    #[error(
        "Encountered a token with one or more lexical errors while that matches a grammar rule: {0:?}."
    )]
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
