use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Attempted to convert a large integer into too small a size.")]
    IntegerTooBig(#[from] std::num::TryFromIntError),
    #[error("Could not write the message into the guest.")]
    WriteFailed(#[from] crate::wasm::Error),
}
