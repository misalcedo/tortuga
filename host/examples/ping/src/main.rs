use std::io;
use tortuga_guest::{
    Body, Destination, FromHost, MemoryStream, Method, Request, Response, Source, Status, Stream,
};

fn run(_: Request<FromHost>) -> Result<Response<impl Body>, io::Error> {
    let mut stream = Stream::new();
    let request = Request::new(Method::Get, "/pong", MemoryStream::with_reader(b"PING!"));

    stream.write_message(request)?;

    let mut pong: Response<_> = stream.read_message()?;
    let mut response = Response::from(Status::Ok);

    io::copy(pong.body(), response.body())?;

    Ok(response)
}

fn main() {
    tortuga_guest::invoke(run)
}
