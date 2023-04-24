use std::io;
use tortuga_guest::{Body, Request, Response, Status};

fn run(request: Request<impl Body>) -> Result<Response<impl Body>, io::Error> {
    Ok(Response::new(
        Status::Created,
        request.content_length(),
        request.into_body(),
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
        let body = b"Hellow, World!";
        let request = Request::new(
            Method::Post,
            "/echo".into(),
            body.len(),
            Cursor::new(Vec::from(&body[..])),
        );
        let expected = Response::new(
            Status::Created,
            body.len(),
            Cursor::new(Vec::from(&body[..])),
        );

        let mut response = run(request).unwrap();

        assert_eq!(response, expected);

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(response.body(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
