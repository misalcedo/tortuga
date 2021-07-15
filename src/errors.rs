use std::ffi::OsString;
use thiserror::Error;

/// An error that occurred while interacting with Tortuga.
#[derive(Error, Debug)]
pub enum TortugaError {
    #[error("An IO error occurred.")]
    IO(#[from] std::io::Error),
    #[error("An error occurred during compilation.")]
    Other(#[from] crate::compiler::CompilerError),
    #[error("File name is not valid UTF-8: {0:?}.")]
    InvalidFileName(OsString),
    #[error("Unable to walk the input directory.")]
    Walk(#[from] walkdir::Error),
    #[error("Unable to remove the input path from the file name.")]
    InvalidPath(#[from] std::path::StripPrefixError),
    #[error("Unknown error occurred.")]
    Unknown,
}
