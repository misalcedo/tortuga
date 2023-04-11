use crate::runtime::connection::{ForGuest, FromGuest};
use crate::runtime::promise::Promise;
use crate::runtime::Identifier;
use tortuga_guest::{Bidirectional, Destination, MemoryStream, Request, Response};

pub struct Message {
    to: Identifier,
    body: Option<MemoryStream<Bidirectional>>,
    promise: Promise<Response<FromGuest>>,
}

impl Message {
    pub fn new(
        identifier: impl AsRef<Identifier>,
        request: Request<ForGuest>,
        promise: Promise<Response<FromGuest>>,
    ) -> Self {
        let mut body = MemoryStream::default();

        body.write_message(request).unwrap();
        body.swap();

        Message {
            to: identifier.as_ref().clone(),
            body: Some(body),
            promise,
        }
    }

    pub fn to(&self) -> &Identifier {
        &self.to
    }

    pub fn body(&mut self) -> MemoryStream<Bidirectional> {
        self.body.take().unwrap()
    }

    pub fn promise(&mut self) -> &mut Promise<Response<FromGuest>> {
        &mut self.promise
    }
}
