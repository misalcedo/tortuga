use thiserror::Error;

/// An error that occurred while interacting with Tortuga.
#[derive(Error, Debug)]
pub enum TortugaError {
    #[error("An IO error occurred.")]
    IO(#[from] std::io::Error),
    #[error("An error occurred during compilation.")]
    CompilerError(#[from] crate::compiler::CompilerError),
    #[error("Unknown error occurred.")]
    Unknown,
}
