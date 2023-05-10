use std::fmt::{Debug, Formatter};

#[derive(Default, Clone, PartialEq)]
pub struct Response {
    status: u16,
}

impl Response {
    pub fn new(status: impl Into<u16>) -> Self {
        Response {
            status: status.into(),
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }
}
