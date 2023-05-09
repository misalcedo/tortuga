mod status;

pub use status::Status;
use std::fmt::{Debug, Formatter};

#[derive(Default, Clone)]
pub struct Response<B> {
    status: u16,
    content_length: usize,
    body: B,
}

impl<B> Debug for Response<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .finish()
    }
}

impl<A, B> PartialEq<Response<B>> for Response<A> {
    fn eq(&self, other: &Response<B>) -> bool {
        self.status == other.status
    }
}

impl<B> Response<B> {
    pub fn new(status: impl Into<u16>, content_length: usize, body: B) -> Self {
        Response {
            status: status.into(),
            content_length,
            body,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn content_length(&self) -> usize {
        self.content_length
    }

    pub fn body(&mut self) -> &mut B {
        &mut self.body
    }

    pub fn consume_body(self) -> B {
        self.body
    }
}
