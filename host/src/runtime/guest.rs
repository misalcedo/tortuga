use std::future::Future;
use std::sync::mpsc::Sender;

use tortuga_guest::{Request, Response};

use crate::runtime::connection::{ForGuest, FromGuest};
use crate::runtime::message::Message;
use crate::runtime::promise::Promise;
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

    pub fn execute(&self, request: Request<ForGuest>) -> impl Future<Output = Response<FromGuest>> {
        let future = Promise::default();
        let message = Message::new(self, request, future.clone());

        self.sender.send(message).unwrap();

        future
    }
}
