mod error;
mod wasm;

pub use error::Error;

/// A sender and receiver of messages.
/// Defines the contract between the intent and the system.
trait Actor {
    /// Receives a message from another actor. The system makes no guarantees about the contents.
    fn receive(&self, message: &[u8]) -> Result<(), Error>;
}
