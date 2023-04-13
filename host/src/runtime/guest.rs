use tokio::sync::oneshot;

use crate::runtime::channel::{ChannelStream, Sender};
use tortuga_guest::{Body, Destination, Request, Response, Source};

use crate::runtime::connection::FromGuest;
use crate::runtime::message::Message;
use crate::runtime::Identifier;
use crate::runtime::Uri;

#[derive(Clone, Debug)]
pub struct Guest {
    identifier: Identifier,
    uri: Uri,
    sender: Sender<Message>,
}

impl AsRef<Identifier> for Guest {
    fn as_ref(&self) -> &Identifier {
        &self.identifier
    }
}

impl Guest {
    pub fn new(uri: String, sender: Sender<Message>) -> Self {
        Guest {
            identifier: Identifier::from(uri.as_str()),
            uri: Uri::from(uri),
            sender,
        }
    }

    pub async fn execute(&self, request: Request<impl Body>) -> Response<FromGuest> {
        let (sender, receiver) = oneshot::channel();
        let (mut guest, host) = ChannelStream::new();

        guest.write_message(request).unwrap();

        let message = Message::new(self, guest, sender);

        self.sender.send(message).await;

        receiver.await.unwrap();

        host.read_message().unwrap()
    }
}
