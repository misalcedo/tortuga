use std::io;
use std::io::{Cursor, Read};
use tortuga_guest::{Body, Destination, Method, Request, Response, Source, Stream};

fn run(_: Request<impl Read>) -> Result<Response<impl Body>, io::Error> {
    let mut stream = Stream::new();
    let request = Request::new(Method::Get, "/pong", Cursor::new(b"PING!".to_vec()));

    stream.write_message(request)?;
    stream.read_message()
}

fn main() {
    tortuga_guest::invoke(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory() {
        todo!("Need to define the testing story.")
    }
}
