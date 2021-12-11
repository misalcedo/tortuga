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
    #[error("Unable to remove the input path from the file name.")]
    InvalidPath(#[from] std::path::StripPrefixError),
    #[error("A runtime error occurred while interpreting the source code. {0}")]
    Runtime(#[from] crate::interpret::RuntimeError),
    #[error("Encountered an error prompting the user for input. {0}")]
    PromptError(#[from] rustyline::error::ReadlineError),
}
