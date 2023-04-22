use crate::executor::{self, acceptor::RoutingAcceptor, Identifier, Router};
use crate::stream::memory;
use crate::wasm;
use std::collections::HashMap;
use tokio::task::yield_now;

pub struct Executor<Acceptor, Host>
where
    Host: wasm::Host,
{
    acceptor: Acceptor,
    host: Host,
    guests: HashMap<Identifier, Host::Guest>,
}

impl Default
    for Executor<RoutingAcceptor<memory::Bridge, Identifier>, wasm::wasmtime::Host<memory::Bridge>>
{
    fn default() -> Self {
        let bridge = memory::Bridge::default();
        let host = wasm::wasmtime::Host::from(bridge.clone());
        let router = Router::default();
        let acceptor = RoutingAcceptor::new(bridge, router.clone());

        Executor::new(acceptor, host)
    }
}

impl<Acceptor, Host> Executor<Acceptor, Host>
where
    Acceptor: executor::Acceptor,
    Host: wasm::Host,
{
    pub fn new(acceptor: Acceptor, host: Host) -> Self {
        Executor {
            acceptor,
            host,
            guests: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {}

    pub async fn step(&mut self) {
        while let Some(message) = self.acceptor.try_accept() {
            match self.guests.get(&message.to()) {
                None => {}
                Some(_) => {}
            }

            yield_now().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sample() {
        let bridge = memory::Bridge::default();
        let router = Router::default();
        let acceptor = RoutingAcceptor::new(bridge.clone(), router.clone());
        let host = wasm::wasmtime::Host::from(bridge);

        let mut executor = Executor::new(acceptor, host);

        executor.step().await;
    }
}
