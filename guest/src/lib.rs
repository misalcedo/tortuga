use std::error::Error;

pub use frame::{Frame, FrameType};
pub use limiter::IoLimiter;
pub use message::{Body, FrameIo};
pub use request::{Method, Request};
pub use response::{Response, Status};
pub use stream::{ReadOnly, Stream};

pub use crate::wire::{Destination, Source};

mod frame;
mod limiter;
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
