use std::io::{self, Cursor};
use tortuga_guest::{Body, Request, Response, Status};

fn run(_: Request<impl Body>) -> io::Result<Response<impl Body>> {
    Ok(Response::new(
        Status::Ok,
        Cursor::new(Vec::from("Hello, World!")),
    ))
}

fn main() {
    tortuga_guest::invoke(run)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tortuga_guest::Method;

    #[test]
    fn in_memory() {
        let body = b"Hello, World!";
        let request = Request::new(Method::Post, "/static", Cursor::new(Vec::new()));
        let expected = Response::new(Status::Ok, Cursor::new(Vec::from(&body[..])));

        let mut response = run(request).unwrap();

        assert_eq!(response, expected);

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(response.body(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
