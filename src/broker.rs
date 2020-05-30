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
    async fn send_to_unregistere() {
        let mut broker = Broker::new();
        let sender = broker.sender();
        let reference = Reference::new();
        let message = b"Hello, World!";

        sender.send(Envelope::new(reference, message)).unwrap();
        
        // assert_eq!(Some(Envelope::new(reference, message)), envelope);
    }

    // #[test]
    // fn send_and_receive_a_message() {
    //     let mut broker = Broker::new();
    //     let sender = broker.sender();
    //     let reference = Reference::new();
    //     let message = b"Hello, World!";

    //     broker.register(reference);
    //     sender(reference, Envelope::new(reference, message));
        
    //     let envelope = broker.pop(reference);

    //     assert_eq!(Some(Envelope::new(reference, message)), envelope);
    // }
}