use std::future::Future;
use tokio::sync::oneshot;

use tortuga_guest::{Body, Destination, Request, Response};

use crate::runtime::channel::{ChannelStream, Sender};
use crate::runtime::connection::FromGuest;
use crate::runtime::message::Message;
use crate::runtime::Identifier;

#[derive(Clone, Debug)]
pub struct Plugin {
    identifier: Identifier,
    sender: Sender<Message>,
}

impl AsRef<Identifier> for Plugin {
    fn as_ref(&self) -> &Identifier {
        &self.identifier
    }
}

impl Plugin {
    pub fn new(sender: Sender<Message>) -> Self {
        Plugin {
            identifier: Default::default(),
            sender,
        }
    }

    pub fn queue(&self, request: Request<impl Body>) -> impl Future<Output = Response<FromGuest>> {
        let (sender, receiver) = oneshot::channel();
        let (mut guest, host) = ChannelStream::new();

        guest.write_message(request).unwrap();

        let message = Message::new(self, guest, sender);

        self.sender.try_send(message).ok_or(()).err().unwrap();

        Message::await_response(receiver, host)
    }
}
