use std::io;
use std::io::{Cursor, Read};
use tortuga_guest::{Body, Request, Response, Status};

const MESSAGE: &'static [u8; 5] = b"PONG!";

fn run(_: Request<impl Read>) -> Result<Response<impl Body>, io::Error> {
    Ok(Response::new(
        Status::Ok,
        MESSAGE.len(),
        Cursor::new(MESSAGE.to_vec()),
    ))
}

fn main() {
    tortuga_guest::invoke(run)
}
