use crate::errors::{Error, Result};
use crate::reference::Reference;
use crate::wasm::Module;
use bytes::Bytes;
use crossbeam::channel::{self, Receiver, Sender};

struct Channel {
    sender: Sender<Bytes>,
    receiver: Receiver<Bytes>,
}

impl Channel {
    fn new() -> Channel {
        let (sender, receiver) = channel::unbounded();

        Channel { sender, receiver }
    }
}

pub(crate) struct Actor {
    reference: Reference,
    mailbox: Channel,
    module: Module,
}

impl Actor {
    pub(crate) fn new(module: Module) -> Actor {
        Actor {
            reference: Reference::new(),
            mailbox: Channel::new(),
            module,
        }
    }

    pub fn reference(&self) -> Reference {
        self.reference
    }

    pub fn module(&self) -> &Module {
        &self.module
    }

    pub fn send(&self, message: &[u8]) -> Result<()> {
        self.mailbox
            .sender
            .try_send(Bytes::copy_from_slice(message))
            .map_err(Error::UnableToSend)
    }

    pub fn receive(&self) -> Result<Bytes> {
        self.mailbox
            .receiver
            .try_recv()
            .map_err(Error::UnableToReceive)
    }
}
