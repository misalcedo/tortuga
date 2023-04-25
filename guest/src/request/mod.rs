mod method;

use crate::Uri;
pub use method::Method;

/// A cursor into the current request being processed.
/// An embedded process handles a single request at a time.
#[derive(Debug, Default, Clone)]
pub struct Request<B> {
    method: Method,
    uri: Uri,
    content_length: usize,
    body: B,
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
