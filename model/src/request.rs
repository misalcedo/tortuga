use crate::{Headers, Method, Uri};
use serde::{Deserialize, Serialize};

/// A cursor into the current request being processed.
/// An embedded process handles a single request at a time.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Request {
    method: Method,
    uri: Uri,
    headers: Headers,
}

impl Request {
    pub fn new(method: Method, uri: Uri, headers: Headers) -> Self {
        Request {
            method,
            uri,
            headers,
        }
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn method(&self) -> Method {
        self.method
    }
}
