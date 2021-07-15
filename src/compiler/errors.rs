use thiserror::Error;

/// An error that occurred while interacting with a Bloom Filter.
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("An IO error occurred.")]
    IO(#[from] std::io::Error),
    #[error("An error occurred during compilation.")]
    Other(#[from] anyhow::Error),
    #[error("A syntax error occurred.")]
    InvalidSyntax,
    #[error("Unknown error occurred.")]
    Unknown,
}
