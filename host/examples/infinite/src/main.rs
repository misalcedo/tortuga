use std::io;
use std::io::Cursor;
use tortuga_guest::{Body, Request, Response};

fn run(_: Request<impl Body>) -> Result<Response<Cursor<Vec<u8>>>, io::Error> {
    loop {}
}

fn main() {
    tortuga_guest::invoke(run)
}
