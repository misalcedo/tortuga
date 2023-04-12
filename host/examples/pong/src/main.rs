use std::io;
use std::io::{Cursor, Read};
use tortuga_guest::{Body, Request, Response, Status};

fn run(_: Request<impl Read>) -> Result<Response<impl Body>, io::Error> {
    Ok(Response::new(Status::Ok, Cursor::new(b"PONG!".to_vec())))
}

fn main() {
    tortuga_guest::invoke(run)
}
