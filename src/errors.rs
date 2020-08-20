use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to decode UTF8 string.")]
    DecodeError(#[from] std::str::Utf8Error),
    #[error("A generic error occurred.")]
    AnyError(#[from] anyhow::Error),
    #[error("A trap occurred in wasmtime.")]
    TrapError(#[from] wasmtime::Trap),
    #[error("Error serializing.")]
    Serialization(#[from] postcard::Error),
}
