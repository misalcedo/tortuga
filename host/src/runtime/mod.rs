//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use wasmtime::{Config, Engine};
use wasmtime_wasi::WasiCtxBuilder;

pub use connection::Connection;
use tortuga_guest::{Destination, MemoryStream, Request, Response};

use connection::{ForGuest, FromGuest};
use guest::Guest;
pub use identifier::Identifier;
use message::Message;
use plugin::Plugin;
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
    channel: (Sender<Message>, Receiver<Message>),
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
            channel: channel(),
        }
    }
}

impl Runtime {
    pub fn load_plugin(&mut self, uri: impl Into<String>, code: impl AsRef<[u8]>) -> Plugin {
        let plugin = Plugin::new(uri.into(), self.channel.0.clone());

        self.plugins.insert(plugin.as_ref().clone(), plugin.clone());
        self.compile(&plugin, code);

        let ctx = WasiCtxBuilder::new().build();

        self.shells.get_mut(plugin.as_ref()).unwrap().promote(ctx);

        plugin
    }

    pub fn welcome_guest(&mut self, uri: impl Into<String>, code: impl AsRef<[u8]>) -> Guest {
        let guest = Guest::new(uri.into(), self.channel.0.clone());

        self.guests.insert(guest.as_ref().clone(), guest.clone());
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

    pub fn run(&mut self) {
        while let Ok(mut message) = self.channel.1.try_recv() {
            let shell = self.shells.get_mut(message.to()).unwrap();
            let response = shell.execute(message.body());

            message.promise().complete(response);
        }
    }

    pub fn start(mut self) {
        for mut message in self.channel.1 {
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

    #[tokio::test]
    async fn execute_ping_pong() {
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
        let actual = ping.execute(request);

        runtime.run();

        let mut actual = actual.await;
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
