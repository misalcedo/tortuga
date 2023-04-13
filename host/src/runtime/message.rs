use crate::runtime::channel::ChannelStream;
use crate::runtime::connection::FromGuest;
use crate::runtime::Identifier;
use tokio::sync::oneshot;
use tortuga_guest::{Response, Source};

#[derive(Debug)]
pub struct Message {
    to: Option<Identifier>,
    body: Option<ChannelStream>,
    promise: Option<oneshot::Sender<()>>,
}

impl From<ChannelStream> for Message {
    fn from(body: ChannelStream) -> Self {
        Message {
            to: None,
            body: Some(body),
            promise: None,
        }
    }
}

impl Message {
    pub fn new(
        identifier: impl AsRef<Identifier>,
        body: ChannelStream,
        sender: oneshot::Sender<()>,
    ) -> Self {
        Message {
            to: Some(identifier.as_ref().clone()),
            body: Some(body),
            promise: Some(sender),
        }
    }

    pub fn to(&self) -> Option<Identifier> {
        self.to
    }

    pub fn complete(&mut self) {
        if let Some(promise) = self.promise.take() {
            promise.send(()).unwrap();
        }
    }

    pub fn take_body(&mut self) -> ChannelStream {
        self.body.take().unwrap()
    }

    pub async fn await_response(
        receiver: oneshot::Receiver<()>,
        host: ChannelStream,
    ) -> Response<FromGuest> {
        receiver.await.unwrap();

        host.read_message().unwrap()
    }
}
