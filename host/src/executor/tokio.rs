use crate::executor::{self, acceptor::RoutingAcceptor, Identifier, Router};
use crate::stream::memory;
use crate::wasm;
use std::time::Duration;
use tokio::task::{yield_now, JoinHandle, JoinSet};
use tortuga_guest::Header;

pub struct Executor<Acceptor, Host> {
    acceptor: Acceptor,
    host: Host,
    ticker: JoinHandle<()>,
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

impl<Acceptor, Host> Drop for Executor<Acceptor, Host> {
    fn drop(&mut self) {
        self.ticker.abort_handle().abort()
    }
}

impl<Acceptor, Host> Executor<Acceptor, Host> {
    pub fn acceptor_mut(&mut self) -> &mut Acceptor {
        &mut self.acceptor
    }

    pub fn host_mut(&mut self) -> &mut Host {
        &mut self.host
    }
}

/// Schedules ticking of the epoch.
/// Must be scheduled in a separate Runtime from the one running the guest code to prevent starvation of this task.
pub fn schedule_tick<Host, Ticker>(host: &Host) -> JoinHandle<()>
where
    Host: wasm::Host<Ticker = Ticker>,
    Ticker: wasm::Ticker + 'static,
{
    let mut epoch = host.ticker();

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(1));

        loop {
            interval.tick().await;
            epoch.tick();
        }
    })
}

impl<Acceptor, Guest, Host, Stream, Ticker> Executor<Acceptor, Host>
where
    Acceptor: executor::Acceptor<Stream = Stream>,
    Guest: wasm::Guest<Stream = Stream> + 'static,
    Host: wasm::Host<Guest = Guest, Ticker = Ticker>,
    Stream: wasm::Stream + 'static,
    Ticker: wasm::Ticker + 'static,
{
    pub fn new(acceptor: Acceptor, host: Host) -> Self {
        let ticker = schedule_tick(&host);

        Executor {
            acceptor,
            host,
            ticker,
        }
    }

    pub async fn run(&mut self) {}

    pub async fn step(&mut self) {
        let mut invocations = JoinSet::default();

        while let Some(message) = self.acceptor.try_accept() {
            let guest = self.host.guest(&message.to()).unwrap();

            invocations.spawn(async move { guest.invoke(message.into_inner()).await.unwrap() });

            yield_now().await;
        }

        while let Some(result) = invocations.join_next().await {
            result.unwrap();
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
        let mut executor = Executor::default();
        let mut bridge = executor.acceptor_mut().provider_mut().clone();
        let mut router = executor.acceptor_mut().router_mut().clone();
        let mut host = executor.host_mut().clone();

        let ping_code = include_bytes!(env!("CARGO_BIN_FILE_PING"));
        let ping = host.welcome(ping_code).unwrap();
        router.define("/ping".into(), ping);

        let pong_code = include_bytes!(env!("CARGO_BIN_FILE_PONG"));
        let pong = host.welcome(pong_code).unwrap();
        router.define("/pong".into(), pong);

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

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn sample_timeout() {
        let mut bridge = memory::Bridge::default();
        let mut router = Router::default();
        let mut host = wasm::wasmtime::Host::new(bridge.clone(), 3);
        let acceptor = RoutingAcceptor::new(bridge.clone(), router.clone());

        let infinite_code = include_bytes!(env!("CARGO_BIN_FILE_INFINITE"));
        let infinite = host.welcome(infinite_code).unwrap();
        router.define("/infinite".into(), infinite);

        let mut executor = Executor::new(acceptor, host);
        let mut client = bridge.create();
        let request = Request::new(Method::Get, "/infinite".into(), 0, Cursor::new(Vec::new()));

        client.write_message(request).unwrap();

        executor.step().await;
    }
}
