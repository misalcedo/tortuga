extern crate core;

use std::error::Error;
use std::io::Cursor;

pub use frame::{Frame, FrameType};
pub use limiter::IoLimiter;

pub use crate::wire::{Destination, Source};
pub use message::{Body, FrameIo};
pub use request::{Method, Request};
pub use response::{Response, Status};
pub use stream::{Bidirectional, ReadOnly, Stream, WriteOnly};
pub use uri::Uri;

mod frame;
mod limiter;
mod message;
mod request;
mod response;
mod stream;
mod uri;
pub mod wire;

type FromHost = FrameIo<Stream<ReadOnly>>;

pub fn invoke<B, E>(entrypoint: fn(Request<FromHost>) -> Result<Response<B>, E>)
where
    B: Body,
    E: Error,
{
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
            let response = Response::new(
                Status::InternalServerError,
                0,
                Cursor::new(error.to_string().as_bytes().to_vec()),
            );

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
        let actual = Request::new(
            Method::Post,
            "/ping".into(),
            body.len(),
            Cursor::new(body.clone()),
        );
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
        let actual = Response::new(Status::Conflict, body.len(), Cursor::new(body.clone()));
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
