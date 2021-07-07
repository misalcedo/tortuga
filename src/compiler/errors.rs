use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// An error that occurred while interacting with a Bloom Fitler.
#[derive(Debug)]
pub struct CompilerError {
    pub kind: ErrorKind,
}

impl CompilerError {
    /// Creates a new error for the given error kind.
    pub fn new(kind: ErrorKind) -> Self {
        CompilerError { kind }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred: {}", self.kind)
    }
}

impl Error for CompilerError {}

/// The kind of error that occurred in interacting with a Bloom Filter.
#[derive(Debug)]
pub enum ErrorKind {
    IO(std::io::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ErrorKind> for CompilerError {
    fn from(kind: ErrorKind) -> Self {
        CompilerError::new(kind)
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::new(ErrorKind::IO(error))
    }
}
