mod method;

pub use method::Method;

/// A cursor into the current request being processed.
/// An embedded process handles a single request at a time.
#[derive(Debug, Default, Clone)]
pub struct Request<B> {
    method: Method,
    uri: String,
    body: B,
}

impl<A, B> PartialEq<Request<B>> for Request<A> {
    fn eq(&self, other: &Request<B>) -> bool {
        self.method == other.method && self.uri.as_str() == other.uri.as_str()
    }
}

impl<B> Request<B> {
    pub fn new(method: Method, uri: impl Into<String>, body: B) -> Self {
        Request {
            method,
            uri: uri.into(),
            body,
        }
    }

    pub fn uri(&self) -> &str {
        self.uri.as_str()
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn body(&mut self) -> &mut B {
        &mut self.body
    }

    pub fn into_body(self) -> B {
        self.body
    }
}
