use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to find the exported '{0}' memory.")]
    NoMatchingMemory(String),
    #[error("The guest module does not export the '{0}' function.")]
    NoMatchingFunction(String),
    #[error("The guest module triggered a trap.")]
    GuestTrap(#[from] wasmtime::Trap),
    #[error("Guest module returned an error while executing exported function.")]
    AnyHow(#[from] anyhow::Error),
}
