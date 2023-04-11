extern crate core;

use std::error::Error;

pub use frame::{Frame, FrameType};
pub use limiter::IoLimiter;

#[cfg(feature = "memory")]
pub use memory::MemoryStream;

pub use message::{Body, FrameIo};
pub use request::{Method, Request};
pub use response::{Response, Status};
pub use stream::{Bidirectional, ReadOnly, Stream, WriteOnly};

pub use crate::wire::{Destination, Source};

mod frame;
mod limiter;

#[cfg(feature = "memory")]
mod memory;

mod message;
mod request;
mod response;
mod stream;
pub(crate) mod wire;

pub type FromHost = FrameIo<Stream<ReadOnly>>;

pub fn invoke<B: Body, E: Error>(entrypoint: fn(Request<FromHost>) -> Result<Response<B>, E>) {
    let (reader, mut writer) = Stream::primary().split();
    let request: Request<FromHost> = reader
        .read_message()
        .expect("Unable to parse request from host.");

    match entrypoint(request) {
        Ok(response) => {
            writer
                .write_message(response)
                .expect("Unable to send response to host.");
        }
        Err(error) => {
            let mut response = Response::with_status(Status::InternalServerError);

            std::io::copy(&mut error.to_string().as_bytes(), response.body())
                .expect("Unable to write error message to response body.");

            writer
                .write_message(response)
                .expect("Unable to send response to host.");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn transfer_request() {
        let body = Vec::from("Hello, World!");
        let actual = Request::new(Method::Post, "/ping".to_string(), Cursor::new(body.clone()));
        let mut stream = Cursor::new(Vec::new());

        stream.write_message(actual.clone()).unwrap();
        stream.set_position(0);

        let mut expected: Request<FrameIo<Cursor<Vec<u8>>>> = stream.read_message().unwrap();
        let mut buffer = vec![0; body.len()];

        expected.body().read_exact(&mut buffer).unwrap();

        assert_eq!(actual, expected);
        assert_eq!(body.as_slice(), buffer.as_slice());
    }

    #[test]
    fn transfer_response() {
        let body = Vec::from("Already exists!");
        let actual = Response::new(Status::Conflict, Cursor::new(body.clone()));
        let mut stream = Cursor::new(Vec::new());

        stream.write_message(actual.clone()).unwrap();
        stream.set_position(0);

        let mut expected: Response<FrameIo<Cursor<Vec<u8>>>> = stream.read_message().unwrap();
        let mut buffer = vec![0; body.len()];

        expected.body().read_exact(&mut buffer).unwrap();

        assert_eq!(actual, expected);
        assert_eq!(body.as_slice(), buffer.as_slice());
    }
}
