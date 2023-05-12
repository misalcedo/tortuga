use crate::asynchronous;
use crate::{Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message<Head, Body> {
    head: Head,
    body: Body,
}

impl<Head, Body> Message<Head, Body> {
    pub fn head(&self) -> &Head {
        &self.head
    }

    pub fn body(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn into_body(self) -> Body {
        self.body
    }
}

impl<Body> Message<Request, Body>
where
    Body: asynchronous::Body,
{
    pub fn new(request: Request, body: Body) -> Self {
        Message {
            head: request,
            body,
        }
    }
}

impl<Body> Message<Response, Body>
where
    Body: asynchronous::Body,
{
    pub fn new(response: Response, body: Body) -> Self {
        Message {
            head: response,
            body,
        }
    }
}
