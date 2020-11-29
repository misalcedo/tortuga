use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Attempted to convert a large integer into too small a size.")]
    IntegerTooBig(#[from] std::num::TryFromIntError),
    #[error("Could not write the message into the guest.")]
    WriteFailed(#[from] crate::wasm::Error),
    #[error("WASM runtime encountered an error.")]
    AnyHow(#[from] anyhow::Error),
    #[error("Encountered an unknown error. Message: {0}.")]
    Wrapped(String),
    #[error("No module registered for {0}.")]
    ModuleNotFound(u128),
    #[error("No module registered with the name {0}.")]
    ModuleNotFoundByName(String),
}
