use std::io;
use std::io::Cursor;
use tortuga_guest::{Body, Destination, FromHost, Method, Request, Response, Source, Stream};

fn run(_: Request<FromHost>) -> Result<Response<impl Body>, io::Error> {
    let mut stream = Stream::new();
    let request = Request::new(Method::Get, "/pong", Cursor::new(b"PING!".to_vec()));

    stream.write_message(request)?;
    stream.read_message()
}

fn main() {
    tortuga_guest::invoke(run)
}
