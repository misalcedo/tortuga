use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Collected, Full};
use hyper::body::{Body, Incoming};

const MAX_BODY_BYTES: u64 = 1024 * 64;

pub trait CgiRequest {
    async fn buffer(self) -> Result<Request<Bytes>, Response<Full<Bytes>>>;
}

impl CgiRequest for Request<Incoming> {
    async fn buffer(self) -> Result<Request<Bytes>, Response<Full<Bytes>>> {
        let upper = self.body().size_hint().upper().unwrap_or(u64::MAX);
        if upper > MAX_BODY_BYTES {
            let mut response = Response::new(Full::from(format!(
                "Body size of {upper} bytes is too large. The largest supported body is {MAX_BODY_BYTES}"
            )));
            *response.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
            return Err(response);
        }

        let (parts, body) = self.into_parts();
        let body = match body.collect().await.map(Collected::to_bytes) {
            Ok(b) => b,
            Err(_) => {
                let mut response =
                    Response::new(Full::from("Unable to read the full request body."));
                *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
                return Err(response);
            }
        };

        Ok(Request::from_parts(parts, body))
    }
}
