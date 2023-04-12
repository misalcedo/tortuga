use std::io;
use std::io::Cursor;
use tortuga_guest::{
    Body, Destination, FromHost, Method, Request, Response, Source, Status, Stream,
};

fn run(_: Request<FromHost>) -> Result<Response<impl Body>, io::Error> {
    let mut stream = Stream::new();
    let request = Request::new(Method::Get, "/pong", Cursor::new(b"PING!".to_vec()));

    stream.write_message(request)?;

    let mut pong: Response<_> = stream.read_message()?;
    let mut response = Response::from(Status::Ok);

    io::copy(pong.body(), response.body())?;
    response.body().set_position(0);

    Ok(response)
}

fn main() {
    tortuga_guest::invoke(run)
}
