use std::convert::Infallible;
use tortuga_compiler::SyntacticalError;
use tracing::subscriber::SetGlobalDefaultError;

pub(crate) enum Error {
    IO(std::io::Error),
    Text(&'static str),
    Syntax(SyntacticalError),
    Tracing(SetGlobalDefaultError),
    Infallible,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<&'static str> for Error {
    fn from(error: &'static str) -> Self {
        Error::Text(error)
    }
}

impl From<SyntacticalError> for Error {
    fn from(error: SyntacticalError) -> Self {
        Error::Syntax(error)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}

impl From<SetGlobalDefaultError> for Error {
    fn from(error: SetGlobalDefaultError) -> Self {
        Error::Tracing(error)
    }
}
