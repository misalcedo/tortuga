use std::io;
use std::io::Write;
use tortuga_guest::{Body, FromHost, Request, Response, Status};

fn run(_: Request<FromHost>) -> Result<Response<impl Body>, io::Error> {
    let mut response = Response::with_status(Status::Ok);

    response.body().write_all(b"PONG!")?;

    Ok(response)
}

fn main() {
    tortuga_guest::invoke(run)
}
