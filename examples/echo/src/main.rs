use std::io::{Read, Write};
use tortuga_guest::{Request, Response, Status};

fn main() {
    let mut buffer = vec![0; 4098];
    let mut request = Request::default();
    let mut response = Response::default();

    let bytes = request.read(&mut buffer).unwrap();

    response.set_status(Status::Created);
    response.write(&buffer[..bytes]).unwrap();
}
