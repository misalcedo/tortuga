use crate::language::target::model::Module;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

/// Error type when emitting WebAssembly code.
#[derive(Debug, Eq, PartialEq)]
pub struct EmitterError {
    kind: ErrorKind
}

/// Kinds of errors when emitting WebAssembly code.
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    WriteFailure
}

impl Display for EmitterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for EmitterError {
}

impl EmitterError {
    pub fn new(kind: ErrorKind) -> EmitterError {
        EmitterError {
            kind
        }
    }
}

pub trait Emitter {
    /// Writes the WebAssembly module to this target.
    fn write(&self, module: &Module, output: &mut impl Write) -> Result<(), EmitterError>;
}