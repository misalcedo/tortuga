use std::collections::HashSet;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use crate::message::Envelope;
use crate::errors::{Error, Result};
use crate::reference::Reference;

pub struct Broker {
    sender: UnboundedSender<Envelope>,
    receiver: UnboundedReceiver<Envelope>,
    references: HashSet<Reference>
}

impl Broker {
    pub fn new() -> Broker {
        let (sender, receiver) = unbounded_channel();

        Broker {
            sender, 
            receiver,
            references: HashSet::new()
        }
    }

    pub fn register(&mut self, actor: Reference) -> Result<Reference> {
        let already_registered = self.references.insert(actor);

        if already_registered {
            Ok(actor)
        } else {
            Err(Error::AlreadyRegistered(actor))
        }
    }

    pub fn sender(&self) -> UnboundedSender<Envelope> {
        self.sender.clone()
    }

    /// Retrieves the next message to be processed by the actor system.
    pub async fn pop(&mut self) -> Option<Envelope> {
        self.receiver.recv().await.filter(|envelope| self.references.contains(&envelope.to()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_twice() {
        let mut broker = Broker::new();
        let reference = Reference::new();

        assert_eq!(reference, broker.register(reference).unwrap());

        match broker.register(reference).unwrap_err() {
            Error::AlreadyRegistered(other_reference) => assert_eq!(reference, other_reference),
            e => panic!("Unexpected error type: {}", e)
        }
    }

    #[tokio::test]
    async fn send_to_unregistered() {
        let mut broker = Broker::new();

        broker.sender().send(Envelope::new(Reference::new(), b"Hello, World!")).unwrap();
        
        assert!(broker.pop().await.is_none());
    }

    #[tokio::test]
    async fn send_and_receive_a_message() {
        let mut broker = Broker::new();
        let reference = Reference::new();
        let message = b"Hello, World!";

        broker.register(reference).unwrap();
        broker.sender().send(Envelope::new(reference, message)).unwrap();
        
        let envelope = broker.pop().await;

        assert_eq!(Some(Envelope::new(reference, message)), envelope);
    }
}