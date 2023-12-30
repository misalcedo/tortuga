use bytes::Bytes;
use http::{Response, StatusCode};
use http_body_util::Full;
use hyper::body::Body;

pub trait CgiResponse {
    fn is_document(&self) -> bool;
    fn is_redirect(&self) -> bool;
    fn is_local_redirect(&self) -> bool;
    fn is_client_redirect(&self) -> bool;
    fn is_client_redirect_with_document(&self) -> bool;
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
}
