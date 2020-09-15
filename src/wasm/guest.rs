use crate::wasm::memory::Address;
use crate::wasm::Error;

/// Represents a single instance of a module running as a guest on the actor system.
/// Defines the interface between the actor system and the WASM runtime.
pub trait Guest {
    /// Copies the given message into the returned address.
    fn copy(&self, message: &[u8]) -> Result<Address, Error>;

    /// Receives a message on a slice that was previously allocated by this guest.
    /// The system makes no guarantees about the contents.
    /// After the guest processes the message, the guest is free to deallocate the slice.
    /// The guest, may also reuse the slice for a future message.
    fn receive(&self, message: Address) -> Result<(), Error>;
}
