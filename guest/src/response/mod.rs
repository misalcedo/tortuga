use std::io::Cursor;

mod status;

use crate::Body;
pub use status::Status;

#[derive(Debug, Eq, Ord, PartialOrd, Copy, Clone)]
pub struct Response<B> {
    status: u16,
    body: B,
}

impl Default for Response<Cursor<Vec<u8>>> {
    fn default() -> Self {
        Response {
            status: Default::default(),
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

impl Response<Cursor<Vec<u8>>> {
    pub fn with_status(status: impl Into<u16>) -> Self {
        Response {
            status: status.into(),
            body: Default::default(),
        }
    }
}

impl<B> Response<B> {
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
