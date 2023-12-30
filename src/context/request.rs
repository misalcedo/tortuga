use crate::context::{ClientContext, ServerContext};
use http::request::Parts;
use std::sync::Arc;

pub struct RequestContext {
    server: Arc<ServerContext>,
    client: Arc<ClientContext>,
    request: Parts,
}

impl RequestContext {
    pub fn new(server: Arc<ServerContext>, client: Arc<ClientContext>, request: Parts) -> Self {
        Self {
            server,
            client,
            request,
        }
    }

    pub fn server(&self) -> &ServerContext {
        self.server.as_ref()
    }

    pub fn client(&self) -> &ClientContext {
        self.client.as_ref()
    }

    pub fn request(&self) -> &Parts {
        &self.request
    }
}
