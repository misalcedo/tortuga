use tortuga_guest::{Body, HostRequest, Response, Status};

fn run(request: &mut HostRequest) -> Result<Response<impl Body>, std::io::Error> {
    let mut response = Response::default();

    std::io::copy(request.body(), response.body())?;

    response.set_status(Status::Created);

    Ok(response)
}

fn main() {
    tortuga_guest::invoke(run)
}
