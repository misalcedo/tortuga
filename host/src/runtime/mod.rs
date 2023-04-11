//! The embedding runtime for the Tortuga WASM modules.

use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::RwLock;
use wasmtime::{Config, Engine};
use wasmtime_wasi::WasiCtxBuilder;

pub use connection::Connection;
use tortuga_guest::{Request, Response};

use connection::{ForGuest, FromGuest};
use guest::Guest;
pub use identifier::Identifier;
use plugin::Plugin;
use response::ResponseFuture;
pub use shell::Shell;
pub use uri::Uri;

mod connection;
mod guest;
mod identifier;
mod plugin;
mod response;
mod shell;
mod uri;

pub struct Runtime {
    engine: Engine,
    guests: HashMap<Identifier, Guest>,
    plugins: HashMap<Identifier, Plugin>,
    shells: HashMap<Identifier, Shell>,
    lock: RwLock<VecDeque<Request<ForGuest>>>,
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
            lock: Default::default(),
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

        shell.execute(request)
    }

    pub fn queue(
        &mut self,
        identifier: impl AsRef<Identifier>,
        request: Request<ForGuest>,
    ) -> impl Future<Output = Response<FromGuest>> {
        let mut guard = self.lock.write().unwrap();

        guard.push_back(request);

        ResponseFuture
    }

    fn compile(&mut self, identifier: impl AsRef<Identifier>, code: impl AsRef<[u8]>) {
        let shell = Shell::new(&self.engine, code);

        self.shells.insert(identifier.as_ref().clone(), shell);
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};

    use tortuga_guest::{MemoryStream, Method, Status};

    use super::*;

    #[test]
    fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let body = Vec::from("Hello, World!");

        let mut runtime = Runtime::default();
        let mut request = Request::new(Method::Get, "/", MemoryStream::with_reader(&body));
        let mut response = Response::new(Status::Created, MemoryStream::with_reader(&body));

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

        let mut request = Request::new(Method::Get, "/ping".to_string(), MemoryStream::default());
        let mut response = Response::from(Status::Ok);

        request.body().write_all(body).unwrap();
        response.body().write_all(body).unwrap();

        let mut actual = runtime.execute(&ping, request);
        let mut buffer = vec![0; body.len()];

        actual.body().read_exact(buffer.as_mut_slice()).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer, body);
    }
}
