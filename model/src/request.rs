mod method;

use crate::Uri;
pub use method::Method;
use std::fmt::{Debug, Formatter};

/// A cursor into the current request being processed.
/// An embedded process handles a single request at a time.
#[derive(Default, Clone)]
pub struct Request<B> {
    method: Method,
    uri: Uri,
    content_length: usize,
    body: B,
}

impl<B> Debug for Request<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("content_length", &self.content_length)
            .finish()
    }
}

impl<A, B> PartialEq<Request<B>> for Request<A> {
    fn eq(&self, other: &Request<B>) -> bool {
        self.method == other.method && self.uri == other.uri
    }
}

impl<B> Request<B> {
    pub fn new(method: Method, uri: Uri, content_length: usize, body: B) -> Self {
        Request {
            method,
            uri,
            content_length,
            body,
        }
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn content_length(&self) -> usize {
        self.content_length
    }

    pub fn body(&mut self) -> &mut B {
        &mut self.body
    }

    pub fn into_body(self) -> B {
        self.body
    }
}
