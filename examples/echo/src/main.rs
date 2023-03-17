use tortuga_guest::{Request, Response, Status};

fn run(request: &mut Request, response: &mut Response) {
    std::io::copy(request, response).unwrap();

    response.set_status(Status::Created);
}

fn main() {
    tortuga_guest::invoke(run)
}
