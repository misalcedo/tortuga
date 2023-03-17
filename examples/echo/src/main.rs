use tortuga_guest::{Request, Response, Status};

fn run(mut request: Request, mut response: Response) {
    std::io::copy(&mut request, &mut response).unwrap();

    response.set_status(Status::Created);
}

fn main() {
    tortuga_guest::invoke(run)
}
