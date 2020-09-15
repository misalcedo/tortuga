use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not copy the message into the guest.")]
    CopyFailed(#[from] crate::wasm::Error),
}
