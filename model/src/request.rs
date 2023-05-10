use crate::{Method, Uri};

/// A cursor into the current request being processed.
/// An embedded process handles a single request at a time.
#[derive(Default, Clone, PartialEq)]
pub struct Request {
    method: Method,
    uri: Uri,
}

impl Request {
    pub fn new(method: Method, uri: Uri) -> Self {
        Request { method, uri }
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn method(&self) -> Method {
        self.method
    }
}
