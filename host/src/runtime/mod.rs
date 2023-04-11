//! The embedding runtime for the Tortuga WASM modules.

use std::collections::{HashMap, VecDeque};
use std::future::Future;
use wasmtime::{Config, Engine};
use wasmtime_wasi::WasiCtxBuilder;

pub use connection::Connection;
use tortuga_guest::{Destination, MemoryStream, Request, Response};

use connection::{ForGuest, FromGuest};
use guest::Guest;
pub use identifier::Identifier;
use message::Message;
use plugin::Plugin;
use promise::Promise;
pub use shell::Shell;
pub use uri::Uri;

mod connection;
mod guest;
mod identifier;
mod message;
mod plugin;
mod promise;
mod shell;
mod uri;

pub struct Runtime {
    engine: Engine,
    guests: HashMap<Identifier, Guest>,
    plugins: HashMap<Identifier, Plugin>,
    shells: HashMap<Identifier, Shell>,
    queue: VecDeque<Message>,
}

impl Default for Runtime {
    fn default() -> Self {
        let configuration = Config::new();
        let engine = Engine::new(&configuration).unwrap();

        Runtime {
            engine,
            guests: Default::default(),
            plugins: Default::default(),
            shells: Default::default(),
            queue: Default::default(),
        }
    }
}

impl Runtime {
    pub fn load_plugin(&mut self, uri: impl Into<String>, code: impl AsRef<[u8]>) -> Plugin {
        let plugin = Plugin::new(uri.into());

        self.plugins.insert(plugin.identifier(), plugin.clone());
        self.compile(&plugin, code);

        let ctx = WasiCtxBuilder::new().build();

        self.shells.get_mut(plugin.as_ref()).unwrap().promote(ctx);

        plugin
    }

    pub fn welcome_guest(&mut self, uri: impl Into<String>, code: impl AsRef<[u8]>) -> Guest {
        let guest = Guest::new(uri.into());

        self.guests.insert(guest.identifier(), guest.clone());
        self.compile(&guest, code);

        guest
    }

    pub fn execute(
        &mut self,
        identifier: impl AsRef<Identifier>,
        request: Request<ForGuest>,
    ) -> Response<FromGuest> {
        let shell = self.shells.get_mut(identifier.as_ref()).unwrap();
        let mut primary = MemoryStream::default();

        primary.write_message(request).unwrap();
        primary.swap();

        shell.execute(primary)
    }

    pub fn queue(
        &mut self,
        identifier: impl AsRef<Identifier>,
        request: Request<ForGuest>,
    ) -> impl Future<Output = Response<FromGuest>> {
        let future = Promise::default();
        let message = Message::new(identifier, request, future.clone());

        self.queue.push_back(message);

        future
    }

    pub fn start(mut self) {
        for mut message in self.queue.drain(..) {
            let shell = self.shells.get_mut(message.to()).unwrap();
            let response = shell.execute(message.body());

            message.promise().complete(response);
        }
    }

    fn compile(&mut self, identifier: impl AsRef<Identifier>, code: impl AsRef<[u8]>) {
        let shell = Shell::new(&self.engine, code);

        self.shells.insert(identifier.as_ref().clone(), shell);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use tortuga_guest::{MemoryStream, Method, Status};

    use super::*;

    #[test]
    fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let body = Vec::from("Hello, World!");

        let mut runtime = Runtime::default();
        let request = Request::new(Method::Get, "/", MemoryStream::with_reader(&body));
        let response = Response::new(Status::Created, MemoryStream::with_reader(&body));

        let guest = runtime.welcome_guest("/".to_string(), code);
        let mut actual = runtime.execute(&guest, request);
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    #[test]
    fn execute_ping_pong() {
        let ping_code = include_bytes!(env!("CARGO_BIN_FILE_PING"));
        let pong_code = include_bytes!(env!("CARGO_BIN_FILE_PONG"));
        let body = b"PONG!";

        let mut runtime = Runtime::default();
        let ping = runtime.welcome_guest("/ping", ping_code);
        runtime.welcome_guest("/pong", pong_code);

        let request = Request::new(
            Method::Get,
            "/ping".to_string(),
            MemoryStream::with_reader(&body),
        );
        let response = Response::new(Status::Ok, MemoryStream::with_reader(&body));

        let waker = noop_waker();
        let mut binding = runtime.queue(&ping, request);
        let mut context = Context::from_waker(&waker);
        let mut actual = Pin::new(&mut binding);

        assert_eq!(actual.poll(&mut context), Poll::Pending);

        runtime.start();

        actual = Pin::new(&mut binding);

        let mut buffer = Cursor::new(Vec::new());
        let result = actual.poll(&mut context);

        if let Poll::Ready(mut result) = result {
            assert_eq!(result, response);

            std::io::copy(result.body(), &mut buffer).unwrap();
        }

        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    unsafe fn noop(_p: *const ()) {}

    unsafe fn noop_clone(_p: *const ()) -> RawWaker {
        noop_raw_waker()
    }

    fn noop_raw_waker() -> RawWaker {
        const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

        RawWaker::new(std::ptr::null(), &RAW_WAKER_VTABLE)
    }

    fn noop_waker() -> Waker {
        unsafe { Waker::from_raw(noop_raw_waker()) }
    }
}
