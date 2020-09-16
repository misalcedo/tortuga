use crate::wasm::Error;

/// Represents a single instance of a module running as a guest on the actor system.
/// Defines the interface between the actor system and the WASM runtime.
pub trait Guest {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    /// There is no guarantee that the allocated memory is greater than requested.
    fn allocate(&self, minimum_length: u32) -> Result<u32, Error>;

    /// Writes a message into an instance of a WebAssembly module.
    fn write(&self, offset: u32, message: &[u8]) -> Result<(), Error>;

    /// Signals to the guest module that a message of the given length in bytes
    /// can be found at the given offset in memory.
    fn receive(&self, offset: u32, length: u32) -> Result<(), Error>;
}
