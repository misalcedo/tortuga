use crate::{Request, Response};

#[derive(Clone, Debug, PartialEq)]
pub struct Message<Head, Content> {
    head: Head,
    content: Content,
}

impl<Head, Content> Message<Head, Content> {
    pub fn head(&self) -> &Head {
        &self.head
    }

    pub fn content(&mut self) -> &mut Content {
        &mut self.content
    }

    pub fn into_content(self) -> Content {
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
    pub fn new(response: Response, content: Content) -> Self {
        Message {
            head: response,
            content,
        }
    }
}
