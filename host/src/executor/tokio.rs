use crate::executor::{self, acceptor::RoutingAcceptor, Identifier, Router};
use crate::stream::memory;
use crate::wasm;
use tokio::task::yield_now;
use tortuga_guest::Header;

pub struct Executor<Acceptor, Host>
where
    Host: wasm::Host,
{
    acceptor: Acceptor,
    host: Host,
}

impl Default
    for Executor<
        RoutingAcceptor<memory::Bridge, Identifier>,
        wasm::wasmtime::Host<Header<memory::Stream>, memory::Bridge, memory::Stream>,
    >
{
    fn default() -> Self {
        let bridge = memory::Bridge::default();
        let host = wasm::wasmtime::Host::from(bridge.clone());
        let router = Router::default();
        let acceptor = RoutingAcceptor::new(bridge, router.clone());

        Executor::new(acceptor, host)
    }
}

impl<Acceptor, Guest, Host, Stream> Executor<Acceptor, Host>
where
    Acceptor: executor::Acceptor<Stream = Stream>,
    Guest: wasm::Guest<Stream = Stream>,
    Host: wasm::Host<Guest = Guest>,
    Stream: wasm::Stream,
{
    pub fn new(acceptor: Acceptor, host: Host) -> Self {
        Executor { acceptor, host }
    }

    pub async fn run(&mut self) {}

    pub async fn step(&mut self) {
        while let Some(message) = self.acceptor.try_accept() {
            match self.host.guest(&message.to()) {
                None => {}
                Some(guest) => {
                    guest.invoke(message.into_inner()).await.unwrap();
                }
            }

            yield_now().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::{Factory, Host};
    use std::io::Cursor;
    use tortuga_guest::{Destination, Method, Request, Response, Source, Status};

    #[tokio::test]
    async fn sample() {
        let mut bridge = memory::Bridge::default();
        let mut router = Router::default();
        let mut host = wasm::wasmtime::Host::from(bridge.clone());
        let acceptor = RoutingAcceptor::new(bridge.clone(), router.clone());

        let ping_code = include_bytes!(env!("CARGO_BIN_FILE_PING"));
        let ping = host.welcome(ping_code).unwrap();
        router.define("/ping".into(), ping);

        let pong_code = include_bytes!(env!("CARGO_BIN_FILE_PONG"));
        let pong = host.welcome(pong_code).unwrap();
        router.define("/pong".into(), pong);

        let mut executor = Executor::new(acceptor, host);
        let mut client = bridge.create();
        let request = Request::new(Method::Get, "/ping".into(), 0, Cursor::new(Vec::new()));

        client.write_message(request).unwrap();

        executor.step().await;

        let body = b"PONG!";
        let response = Response::new(Status::Ok, body.len(), Cursor::new(body.to_vec()));
        let mut actual: Response<_> = client.read_message().unwrap();
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(response, actual);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
