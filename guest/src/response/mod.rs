mod status;

use crate::Body;
pub use status::Status;
use std::fmt::{Debug, Formatter};
use std::io::Cursor;

#[derive(Default, Copy, Clone)]
pub struct Response<B> {
    status: u16,
    body: B,
}

impl<B> Debug for Response<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .finish()
    }
}

impl From<Status> for Response<Cursor<Vec<u8>>> {
    fn from(value: Status) -> Self {
        Response {
            status: value.into(),
            body: Default::default(),
        }
    }
}

impl<A, B> PartialEq<Response<B>> for Response<A> {
    fn eq(&self, other: &Response<B>) -> bool {
        self.status == other.status
    }
}

impl<B: Body> From<B> for Response<B> {
    fn from(body: B) -> Self {
        Response {
            status: Default::default(),
            body,
        }
    }
}

impl<B> Response<B> {
    // TODO: Limit new instances to just those that are read or write.
    pub fn new(status: impl Into<u16>, body: B) -> Self {
        Response {
            status: status.into(),
            body,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&mut self) -> &mut B {
        &mut self.body
    }

    pub fn consume_body(self) -> B {
        self.body
    }
}
