use thiserror::Error;

/// An error that occurred while executing the Command-Line interface.
#[derive(Error, Debug)]
pub enum CommandLineError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Unable to set global default tracing collector.")]
    Tracing(#[from] tracing::dispatcher::SetGlobalDefaultError),
    #[error("Unable to set log tracing redirection.")]
    Logging(#[from] tracing_log::log_tracer::SetLoggerError),
    #[error("Unable to remove the input path from the file name.")]
    InvalidPath(#[from] std::path::StripPrefixError),
    #[error("Encountered an error prompting the user for input. {0}")]
    PromptError(#[from] rustyline::error::ReadlineError),
    #[error("Encountered compilation errors.")]
    Compiler(Vec<tortuga::CompilationError>),
    #[error("Encountered error during execution. {0}")]
    Runtime(#[from] tortuga::RuntimeError),
}

impl From<Vec<tortuga::CompilationError>> for CommandLineError {
    fn from(errors: Vec<tortuga::CompilationError>) -> Self {
        CommandLineError::Compiler(errors)
    }
}
