use thiserror::Error;

/// An error that occurred while interacting with Tortuga.
#[derive(Error, Debug)]
pub enum TortugaError {
    #[error("An IO error occurred.")]
    IO(#[from] std::io::Error),
    #[error("An error occurred during compilation.")]
    Compiler(#[from] crate::compiler::CompilerError),
    #[error("Unable to walk the input directory.")]
    Walk(#[from] walkdir::Error),
    #[error("Failed to build the project.")]
    Build(Vec<TortugaError>),
    #[error("Unable to remove the input path from the file name.")]
    InvalidPath(#[from] std::path::StripPrefixError),
    #[error("Invalid subcommand name: {0}")]
    InvalidSubcommand(String),
    #[error("Unknown error occurred.")]
    Unknown,
}
