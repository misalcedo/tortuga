use crate::{Headers, Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    status: Status,
    headers: Headers,
}

impl Response {
    pub fn new(status: Status, headers: Headers) -> Self {
        Response { status, headers }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn into_headers(self) -> Headers {
        self.headers
    }
}
