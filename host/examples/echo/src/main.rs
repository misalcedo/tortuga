use std::io;
use tortuga_guest::{Body, FromHost, Request, Response, Status};

fn run(mut request: Request<FromHost>) -> Result<Response<impl Body>, io::Error> {
    let mut response = Response::with_status(Status::Created);

    io::copy(request.body(), response.body())?;

    Ok(response)
}

fn main() {
    tortuga_guest::invoke(run)
}
