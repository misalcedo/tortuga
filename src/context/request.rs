use crate::context::{ClientContext, ServerContext};
use crate::uri::decode_percent_encoded;
use crate::variable::ToMetaVariable;
use base64::Engine;
use bytes::Bytes;
use http::{HeaderValue, Request};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

pub struct RequestContext {
    server: Arc<ServerContext>,
    variables: HashMap<String, String>,
    arguments: Vec<String>,
    script: Option<PathBuf>,
}

impl RequestContext {
    pub fn new(
        server: Arc<ServerContext>,
        client: Arc<ClientContext>,
        request: &Request<Bytes>,
    ) -> Self {
        let (script, extra_path) = match server.script_filename(request.uri().path()) {
            Some((script, extra_path)) => (Some(script), extra_path),
            None => (None, request.uri().path()),
        };

        let mut variables = HashMap::with_capacity(32);

        let script_name = script
            .as_ref()
            .and_then(|s| s.strip_prefix(server.working_directory()).ok())
            .map(|s| format!("/cgi-bin/{}", s.display()))
            .unwrap_or_else(String::new);
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

        if !request.body().is_empty() {
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

        if let Some((auth_type, user)) = Self::extract_user(request) {
            variables.insert("AUTH_TYPE".to_string(), auth_type.as_str().to_string());
            variables.insert("REMOTE_USER".to_string(), user.to_string());
        }

        for (name, value) in request.headers().iter() {
            let key = name.as_str().to_meta_variable(Some("HTTP"));
            let value = String::from_utf8_lossy(value.as_bytes()).to_string();

            variables.insert(key, value);
        }

        let arguments = Self::extract_arguments(request);

        Self {
            server,
            variables,
            arguments,
            script,
        }
    }

    fn extract_user(request: &Request<Bytes>) -> Option<(AuthType, String)> {
        let header = request
            .headers()
            .get(hyper::header::AUTHORIZATION)?
            .as_bytes();
        let delimiter = header.iter().position(|b| b == &b' ')?;
        let (scheme, mut value) = header.split_at(delimiter);

        // skip the delimiter.
        value = &value[1..];

        let auth_type = AuthType::try_from(scheme).ok()?;

        match auth_type {
            AuthType::Basic => {
                let value = base64::engine::general_purpose::STANDARD
                    .decode(value)
                    .ok()?;
                let value = String::from_utf8(value).ok()?;

                let (user, _) = value.split_once(':')?;

                Some((auth_type, user.to_string()))
            }
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

    pub fn server(&self) -> &ServerContext {
        &self.server
    }

    pub fn working_directory(&self) -> &OsStr {
        self.server.working_directory()
    }

    pub fn script(&self) -> io::Result<&PathBuf> {
        self.script.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Unable to extract CGI script file path from URI path.",
            )
        })
    }

    pub fn arguments(&self) -> impl Iterator<Item = &str> {
        self.arguments.iter().map(String::as_str)
    }

    pub fn variables(&self) -> impl Iterator<Item = (&str, &str)> {
        self.variables.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum AuthType {
    Basic,
}

impl AuthType {
    fn as_str(&self) -> &str {
        match self {
            AuthType::Basic => "Basic",
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for AuthType {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        match value {
            [b'B' | b'b', b'A' | b'a', b'S' | b's', b'I' | b'i', b'C' | b'c'] => {
                Ok(AuthType::Basic)
            }
            _ => Err(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_scheme() {
        assert_eq!(AuthType::try_from(b"BaSic".as_slice()), Ok(AuthType::Basic));
        assert_eq!(AuthType::try_from(b"BASIC".as_slice()), Ok(AuthType::Basic));
        assert_eq!(AuthType::try_from(b"basic".as_slice()), Ok(AuthType::Basic));
        assert_eq!(AuthType::try_from(b"Basic".as_slice()), Ok(AuthType::Basic));
        assert_eq!(
            AuthType::try_from(b"Other".as_slice()),
            Err(b"Other".as_slice())
        );
    }
}
