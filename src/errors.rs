use crate::token::Location;
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
}

/// An error that occurred while interacting with Tortuga.
#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Incomplete grapheme found in source code.")]
    IncompleteGrapheme(unicode_segmentation::GraphemeIncomplete),
    #[error("An unexpected grapheme was found on {0}: {1}.")]
    UnexpectedGrapheme(Location, String),
    #[error("A numeric literal was found with more than 1 decimal point on {0}: {1}.")]
    DuplicateDecimal(Location, String),
    #[error("A numeric literal was found ending with a decimal point on {0}: {1}.")]
    TerminalDecimal(Location, String),
    #[error("A text reference is missing the closing quote on {0}: {1}.")]
    MissingClosingQuote(Location, String),
    #[error("A text reference is empty on {0}.")]
    BlankTextReference(Location),
    #[error("An identifier was found ending with an underscore on {0}: {1}.")]
    TerminalUnderscore(Location, String),
}

impl From<unicode_segmentation::GraphemeIncomplete> for LexicalError {
    fn from(e: unicode_segmentation::GraphemeIncomplete) -> Self {
        LexicalError::IncompleteGrapheme(e)
    }
}
