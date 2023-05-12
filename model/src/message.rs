use crate::{Request, Response};

#[derive(Clone, Debug, PartialEq)]
pub struct Message<Head, Content> {
    head: Head,
    content: Content,
}

impl<Head, Body> Message<Head, Body> {
    pub fn head(&self) -> &Head {
        &self.head
    }

    pub fn content(&mut self) -> &mut Body {
        &mut self.content
    }

    pub fn into_content(self) -> Body {
        self.content
    }
}

impl<Content> Message<Request, Content> {
    pub fn new(request: Request, content: Content) -> Self {
        Message {
            head: request,
            content,
        }
    }
}

impl<Content> Message<Response, Content> {
    pub fn new(response: Response, body: Content) -> Self {
        Message {
            head: response,
            content: body,
        }
    }
}
