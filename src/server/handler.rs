use crate::context::{ClientContext, RequestContext, ServerContext};
use crate::server::response::CgiResponse;
use crate::{script, Script};
use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::{Request, Response};
use std::io;
use std::sync::Arc;

pub struct CgiHandler {
    server: Arc<ServerContext>,
    client: Arc<ClientContext>,
}

impl CgiHandler {
    pub fn new(server: Arc<ServerContext>, client: Arc<ClientContext>) -> Self {
        Self { server, client }
    }

    pub async fn serve(&self, request: Request<Bytes>) -> io::Result<Response<Full<Bytes>>> {
        let context = RequestContext::new(self.server.clone(), self.client.clone(), &request);
        let body = request.into_body();

        let extension = context.script()?.extension();
        let output = if extension == Some("wcgi".as_ref()) {
            script::wasm::serve(context, body).await
        } else if extension == Some("cgi".as_ref()) {
            let script = script::Process::new();
            script.invoke(context, body).await
        } else {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Invalid file extension; must be either cgi or wcgi.",
            ))
        }?;

        let mut response = Response::new(Full::from(output.clone()));

        if output.is_empty() {
            *response.status_mut() = StatusCode::BAD_GATEWAY;
        }

        let offset = response.parse_headers(&output)?;

        if offset != 0 {
            *response.body_mut() = Full::from(output.slice(offset..));
        }

        Ok(response)
    }
}
