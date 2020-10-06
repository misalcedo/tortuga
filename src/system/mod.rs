mod error;
mod root;

pub use error::Error;

/// A reference to an actor is a 128-bit identifier that is unique to an actor system.
type ActorReference = u128;

/// The contents of the message to be transmitted from sender to recipient.
type MessageBody<'a> = &'a [u8];

/// The intent an actor uses to respond to a message.
type Intent<'a> = &'a [u8];

/// The interface to the underlying actor system.
trait System<T: MailBox> {
    /// Create a new actor in the system with given intent.
    /// Returns the reference to the actor.
    fn instantiate(module: Intent) -> ActorReference;

    /// A handle to send messages to the given recipient.
    fn mailbox(recipient: ActorReference) -> T;
}

trait MailBox {
    fn send(message: MessageBody, sender: ActorReference) -> Result<(), Error>;
}
