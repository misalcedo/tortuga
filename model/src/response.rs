use crate::{Headers, Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
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
}
