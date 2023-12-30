use crate::context::{ClientContext, RequestContext, ServerContext};
use crate::script;
use crate::server::response::CgiResponse;
use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::{Request, Response};
use std::io;
use std::mem::size_of;
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
            script::process::serve(context, body).await
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

fn decode_path(s: &str) -> Result<String, &str> {
    if !s.contains('%') {
        return Err(s);
    }

    let mut path = Vec::with_capacity(s.len());
    let mut buffer = [0u8; size_of::<char>()];
    let mut character = String::with_capacity(2);
    let mut characters = s.chars();

    while let Some(c) = characters.next() {
        match c {
            '+' => {
                path.extend_from_slice(' '.encode_utf8(&mut buffer).as_bytes());
            }
            '%' => match (characters.next(), characters.next()) {
                (Some(a), Some(b)) => {
                    character.clear();
                    character.push(a);
                    character.push(b);

                    match u8::from_str_radix(character.as_str(), 16) {
                        Ok(decoded) => path.push(decoded),
                        Err(_) => return Err(s),
                    }
                }
                _ => return Err(s),
            },
            _ => {
                path.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
            }
        }
    }

    String::from_utf8(path).map_err(|_| s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_characters() {
        assert_eq!(decode_path("abc"), Err("abc"));
        assert_eq!(decode_path("%2"), Err("%2"));
        assert_eq!(decode_path("%20%26").unwrap(), " &");
        assert_eq!(decode_path("%C6%92").unwrap(), "ƒ");
    }

    #[test]
    fn empty() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"\r\n";
        let result = httparse::parse_headers(input, &mut headers).unwrap();

        assert!(result.is_complete());
        assert_eq!(headers[0], httparse::EMPTY_HEADER);
    }

    #[test]
    fn header_per_line() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"Content-Length: 42\n";
        let result = httparse::parse_headers(input, &mut headers).unwrap();

        assert!(result.is_partial());
        assert_eq!(headers[0].name, "Content-Length");
        assert_eq!(headers[0].value, b"42");
    }

    #[test]
    fn header_per_line_complete() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"Content-Length: 42\r\n\r\n";
        let result = httparse::parse_headers(input, &mut headers).unwrap();

        assert!(result.is_complete());
        assert_eq!(result.unwrap().0, input.len());
        assert_eq!(headers[0].name, "Content-Length");
        assert_eq!(headers[0].value, b"42");
    }

    #[test]
    fn complete_with_body() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"Foo: Bar\r\n\r\nbody";
        let result = httparse::parse_headers(input, &mut headers).unwrap();
        let start_index = result.unwrap().0;

        assert!(result.is_complete());
        assert_eq!(start_index, input.strip_suffix(b"body").unwrap().len());
        assert_eq!(&input[start_index..], b"body");
        assert_eq!(headers[0].name, "Foo");
        assert_eq!(headers[0].value, b"Bar");
    }
}
