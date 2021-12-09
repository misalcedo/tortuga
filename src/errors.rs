use crate::grammar::ComparisonOperator;
use crate::location::Location;
use crate::token::Kind;
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
#[derive(Debug, Error, PartialEq)]
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
    #[error("Fraction contains {0} digits, but the maximum supported is {}.", u32::MAX)]
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
#[derive(Error, Debug)]
pub enum SyntaxError {
}

/// An error that occurred while parsing a stream of tokens.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown.")]
    Unknown,
    #[error("Found one or more lexical errors while scanning {kind:?} token '{lexeme}' on {location}: {errors}")]
    Lexical {
        location: Location,
        kind: Option<Kind>,
        lexeme: String,
        errors: MultipleErrors<LexicalError>,
    },
    #[error("Failed to validate the current token. {0}")]
    Validation(#[from] LexicalError),
    #[error("Expected token '{lexeme}' ({actual}) on {location} to be of type {expected}.")]
    Syntax {
        location: Location,
        expected: Kinds,
        actual: Kind,
        lexeme: String,
    },
    #[error("Expected a token with type {0}. Instead, reached the end of the file.")]
    EndOfFile(Kinds),
    #[error("No grammar rule was found to match the token kind {1} on {0}.")]
    NoMatchingGrammar(Location, Kind),
    #[error("No grammar rule was found to match the sequence of comparison operators {1} on {0}. Valid comparison operators are: <, =, >, <=, >=, <=>.")]
    InvalidComparator(Location, Kinds),
    #[error("One or more syntax errors found while parsing the source code. {0}")]
    MultipleErrors(MultipleErrors<ParseError>),
}

/// Wrapper struct to define Display trait.
#[derive(Debug)]
pub struct Kinds(Vec<Kind>);

impl From<Vec<Kind>> for Kinds {
    fn from(kinds: Vec<Kind>) -> Self {
        Kinds(kinds)
    }
}

impl fmt::Display for Kinds {
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

impl<E: std::error::Error> From<Vec<E>> for MultipleErrors<E> {
    fn from(errors: Vec<E>) -> Self {
        MultipleErrors(errors)
    }
}

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
