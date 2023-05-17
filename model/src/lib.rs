pub use header::Headers;
pub use message::Message;
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status::Status;
pub use uri::Uri;

pub mod asynchronous;
pub mod encoding;
mod header;
mod message;
mod method;
mod request;
mod response;
mod size;
mod status;
pub mod synchronous;
mod uri;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::Encoder;

    #[test]
    fn request() {
        let uri = Uri::from("https://www.example.com/form");
        let mut headers = Headers::default();

        headers.set(header::Name::ContentType, "application/json");

        let request = Request::new(Method::Post, uri, headers);
        let encoding = encoding::Binary::default();
        let encoded_bytes = encoding.serialize(&request).unwrap();
        let decoded_request = encoding.deserialize(encoded_bytes.as_slice()).unwrap();

        assert_eq!(request, decoded_request);
    }

    #[test]
    fn response() {
        let mut headers = Headers::default();

        headers.set(header::Name::ContentType, "text/html");

        let response = Response::new(Status::Ok, headers);
        let encoding = encoding::Binary::default();
        let encoded_bytes = encoding.serialize(&response).unwrap();
        let decoded_response = encoding.deserialize(encoded_bytes.as_slice()).unwrap();

        assert_eq!(response, decoded_response);
    }
}
