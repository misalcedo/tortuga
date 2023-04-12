use crate::runtime::channel::ChannelStream;
use crate::runtime::connection::FromGuest;
use crate::runtime::promise::Promise;
use crate::runtime::Identifier;
use tortuga_guest::{Body, Destination, Request, Response};

pub struct Message {
    to: Identifier,
    body: Option<ChannelStream>,
    promise: Promise<Response<FromGuest>>,
}

impl Message {
    pub fn new(
        identifier: impl AsRef<Identifier>,
        request: Request<impl Body>,
        promise: Promise<Response<FromGuest>>,
    ) -> Self {
        let mut body = ChannelStream::default();

        body.write_message(request).unwrap();

        Message {
            to: identifier.as_ref().clone(),
            body: Some(body),
            promise,
        }
    }

    pub fn to(&self) -> &Identifier {
        &self.to
    }

    pub fn body(&mut self) -> ChannelStream {
        self.body.take().unwrap()
    }

    pub fn promise(&mut self) -> &mut Promise<Response<FromGuest>> {
        &mut self.promise
    }
}
