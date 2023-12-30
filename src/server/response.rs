use bytes::Bytes;
use http::{HeaderName, HeaderValue, Response, StatusCode};
use http_body_util::Full;
use httparse::Status;
use hyper::body::Body;
use std::io;
use std::str::FromStr;

pub trait CgiResponse {
    fn is_document(&self) -> bool;
    fn is_redirect(&self) -> bool;
    fn is_local_redirect(&self) -> bool;
    fn is_client_redirect(&self) -> bool;
    fn is_client_redirect_with_document(&self) -> bool;

    fn parse_headers(&mut self, output: &Bytes) -> io::Result<usize>;
}

impl CgiResponse for Response<Full<Bytes>> {
    fn is_document(&self) -> bool {
        (self.status().is_success() || self.status().is_client_error())
            && self.headers().contains_key(http::header::CONTENT_TYPE)
    }

    fn is_redirect(&self) -> bool {
        self.status() == StatusCode::OK
            && self.body().size_hint().lower() == 0
            && self.body().size_hint().exact() == Some(0)
            && self.headers().len() == 1
            && self.headers().contains_key(http::header::LOCATION)
    }

    fn is_local_redirect(&self) -> bool {
        self.is_redirect()
            && self
                .headers()
                .get(http::header::LOCATION)
                .map(|l| {
                    // Local URI's must either have an empty path and query, have a non-empty path or have a non-empty query.
                    l.is_empty() || l.as_bytes().starts_with(b"/") || l.as_bytes().starts_with(b"?")
                })
                .unwrap_or(false)
    }

    fn is_client_redirect(&self) -> bool {
        self.is_redirect()
            && self
                .headers()
                .get(http::header::LOCATION)
                .map(|l| {
                    l.as_bytes().starts_with(b"http://") || l.as_bytes().starts_with(b"https://")
                })
                .unwrap_or(false)
    }

    fn is_client_redirect_with_document(&self) -> bool {
        self.status().is_redirection()
            && self.headers().contains_key(http::header::LOCATION)
            && self.headers().contains_key(http::header::CONTENT_TYPE)
    }

    fn parse_headers(&mut self, output: &Bytes) -> io::Result<usize> {
        let mut headers = [httparse::EMPTY_HEADER; 256];

        match httparse::parse_headers(output, &mut headers) {
            Ok(Status::Complete((last, headers))) => {
                let offset = last;

                for header in headers {
                    match header.name {
                        "Status" => {
                            if let Ok(status_code) = StatusCode::from_bytes(header.value) {
                                *self.status_mut() = status_code;
                            }
                        }
                        _ => {
                            match (
                                HeaderName::from_str(header.name),
                                HeaderValue::from_bytes(header.value),
                            ) {
                                (Ok(name), Ok(value)) => {
                                    if !name.as_str().starts_with("x-cgi-") {
                                        self.headers_mut().insert(name, value);
                                    }
                                }
                                _ => {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Invalid response header name.",
                                    ))
                                }
                            }
                        }
                    }
                }

                Ok(offset)
            }
            Ok(Status::Partial) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Received partial response headers from the CGI script.",
            )),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Received invalid response headers from the CGI script.",
            )),
        }
    }
}
