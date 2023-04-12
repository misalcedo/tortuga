use std::io;
use std::io::Read;
use tortuga_guest::{Body, Request, Response, Status};

fn run(mut request: Request<impl Read>) -> Result<Response<impl Body>, io::Error> {
    let mut response = Response::from(Status::Created);

    io::copy(request.body(), response.body())?;

    Ok(response)
}

fn main() {
    tortuga_guest::invoke(run)
}
