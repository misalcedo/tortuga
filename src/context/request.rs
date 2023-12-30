use crate::context::{ClientContext, ServerContext};
use std::sync::Arc;

pub struct RequestContext {
    client: Arc<ClientContext>,
    server: Arc<ServerContext>,
}
