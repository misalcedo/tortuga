use crate::stream::memory;
use crate::wasm::{self, wasmtime};

pub struct Runtime<Acceptor, Host> {
    acceptor: Acceptor,
    host: Host,
}

impl Default for Runtime<memory::Factory, wasmtime::Host<memory::Factory>> {
    fn default() -> Self {
        let acceptor = memory::Factory::default();
        let host = wasmtime::Host::from(acceptor.clone());

        Runtime { acceptor, host }
    }
}

impl<Acceptor, Error, Guest, Host, Stream> Runtime<Acceptor, Host>
where
    Acceptor: wasm::Acceptor<Stream = Stream>,
    Host: wasm::Host<Error = Error, Guest = Guest>,
    Guest: wasm::Guest<Error = Error, Stream = Stream>,
    Stream: wasm::Stream,
{
    pub async fn run(&mut self) {}

    pub async fn step(&mut self) {}
}
