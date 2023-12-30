use crate::context::{ClientContext, ServerContext};
use crate::uri::decode_percent_encoded;
use crate::variable::ToMetaVariable;
use bytes::Bytes;
use http::{HeaderValue, Request};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::Arc;

pub struct RequestContext {
    server: Arc<ServerContext>,
    variables: HashMap<String, String>,
    arguments: Vec<String>,
    script: String,
}

impl RequestContext {
    pub fn new(
        server: Arc<ServerContext>,
        client: Arc<ClientContext>,
        request: &Request<Bytes>,
    ) -> Self {
        let (script, extra_path) = match server.script_filename(request.uri().path()) {
            Ok((script, extra_path)) => (script.display().to_string(), extra_path),
            Err(extra_path) => (String::new(), extra_path),
        };

        let mut variables = HashMap::with_capacity(32);

        for (name, value) in request.headers().iter() {
            let key = name.as_str().to_meta_variable(Some("HTTP"));
            let value = String::from_utf8_lossy(value.as_bytes()).to_string();

            variables.insert(key, value);
        }

        let script_name = format!("/cgi-bin/{}", script);
        let script_uri = format!(
            "{}://{}:{}{}{}?{}",
            server.scheme(),
            server.server_name(),
            server.port(),
            &script_name,
            &extra_path,
            request.uri().query().unwrap_or("")
        );

        variables.insert("PATH".to_string(), server.path().to_string());
        variables.insert("SERVER_SOFTWARE".to_string(), server.software().to_string());
        variables.insert("GATEWAY_INTERFACE".to_string(), "CGI/1.1".to_string());
        variables.insert(
            "SERVER_PROTOCOL".to_string(),
            format!("{:?}", request.version()),
        );
        variables.insert("SCRIPT_URI".to_string(), script_uri);
        variables.insert("SCRIPT_NAME".to_string(), script_name);
        variables.insert("SERVER_NAME".to_string(), server.server_name().to_string());
        variables.insert("SERVER_ADDR".to_string(), server.ip_address().to_string());
        variables.insert("SERVER_PORT".to_string(), server.port().to_string());
        variables.insert(
            "REMOTE_ADDR".to_string(),
            client.remote_ip_address().to_string(),
        );
        variables.insert("REMOTE_PORT".to_string(), client.remote_port().to_string());
        variables.insert("REQUEST_METHOD".to_string(), request.method().to_string());

        if let Some(query) = request.uri().query() {
            variables.insert("QUERY_STRING".to_string(), query.to_string());
        }

        if !extra_path.is_empty() {
            match decode_percent_encoded(extra_path) {
                Ok(path_info) => {
                    variables.insert(
                        "PATH_TRANSLATED".to_string(),
                        server
                            .translate_path(path_info.as_str())
                            .display()
                            .to_string(),
                    );
                    variables.insert("PATH_INFO".to_string(), path_info);
                }
                Err(path_info) => {
                    variables.insert("PATH_INFO".to_string(), path_info.to_string());
                    variables.insert(
                        "PATH_TRANSLATED".to_string(),
                        server.translate_path(path_info).display().to_string(),
                    );
                }
            }
        }

        if request.body().len() > 0 {
            variables.insert(
                "CONTENT_LENGTH".to_string(),
                request.body().len().to_string(),
            );

            if let Some(value) = request
                .headers()
                .get(hyper::header::CONTENT_TYPE)
                .map(HeaderValue::as_bytes)
                .map(String::from_utf8_lossy)
            {
                variables.insert("CONTENT_TYPE".to_string(), value.to_string());
            }
        }

        let arguments = Self::extract_arguments(&request);

        Self {
            server,
            variables,
            arguments,
            script,
        }
    }

    fn extract_arguments(request: &Request<Bytes>) -> Vec<String> {
        let mut arguments = Vec::new();
        if request.method() == http::Method::GET || request.method() == http::Method::HEAD {
            if let Some(query) = request.uri().query() {
                if !query.contains('=') {
                    for search_word in query.split('+') {
                        match decode_percent_encoded(search_word) {
                            Ok(q) => {
                                arguments.extend(q.split(' ').map(String::from));
                            }
                            Err(q) => {
                                arguments.extend(q.split(' ').map(String::from));
                            }
                        }
                    }
                }
            }
        }
        arguments
    }

    pub fn working_directory(&self) -> &OsStr {
        self.server.working_directory()
    }

    pub fn script(&self) -> &str {
        self.script.as_str()
    }

    pub fn arguments(&self) -> impl Iterator<Item = &str> {
        self.arguments.iter().map(String::as_str)
    }

    pub fn variables(&self) -> impl Iterator<Item = (&str, &str)> {
        self.variables.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}
