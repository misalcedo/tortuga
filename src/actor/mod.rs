mod error;
mod guest;

pub use error::Error;

/// A sender and receiver of messages.
/// Defines the contract between the intent and the system.
trait Actor {
    /// Receives a message from another actor. The system makes no guarantees about the contents.
    fn receive(&self, source: u128, message: &[u8]) -> Result<(), Error>;
}
