//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use tokio::task::JoinHandle;
use wasmtime::{Config, Engine};

pub use connection::Connection;

use channel::{new_channel, Receiver, Sender};
use guest::Guest;
pub use identifier::Identifier;
use message::Message;
use plugin::Plugin;
pub use shell::Shell;
pub use uri::Uri;

mod channel;
mod connection;
mod guest;
mod identifier;
mod message;
mod plugin;
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
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.consume_fuel(true);

        let engine = Engine::new(&configuration).unwrap();

        Runtime {
            engine,
            guests: Default::default(),
            plugins: Default::default(),
            shells: Default::default(),
            channel: new_channel(),
        }
    }
}

impl Runtime {
    pub fn load_plugin(&mut self, code: impl AsRef<[u8]>) -> Plugin {
        let plugin = Plugin::new(self.channel.0.clone());

        self.plugins.insert(plugin.as_ref().clone(), plugin.clone());
        self.compile(&plugin, code);
        self.shells.get_mut(plugin.as_ref()).unwrap();

        plugin
    }

    pub fn welcome_guest(&mut self, code: impl AsRef<[u8]>) -> Guest {
        let guest = Guest::new(self.channel.0.clone());

        self.guests.insert(guest.as_ref().clone(), guest.clone());
        self.compile(&guest, code);

        guest
    }

    pub async fn run(&mut self) {
        if let Some(mut message) = self.channel.1.recv().await {
            let shell = self.shells.get(&message.to().unwrap()).cloned().unwrap();
            let stream = message.take_body();
            let root_handle = tokio::spawn(async move {
                shell.execute(stream).await;
                message.complete();
            });
            let mut child_handles = Vec::new();

            while !root_handle.is_finished()
                || child_handles
                    .iter()
                    .any(|handle: &JoinHandle<()>| !handle.is_finished())
            {
                if let Ok(mut child_message) = self.channel.1.try_recv() {
                    let child_shell = self
                        .shells
                        .get(&child_message.to().unwrap())
                        .cloned()
                        .unwrap();
                    let child_stream = child_message.take_body();

                    child_handles.push(tokio::spawn(async move {
                        child_shell.execute(child_stream).await;
                        child_message.complete();
                    }));
                }

                tokio::task::yield_now().await;
            }
        }
    }

    pub async fn start(mut self) {
        while let Some(mut message) = self.channel.1.recv().await {
            let shell = self
                .shells
                .get_mut(&message.to().unwrap())
                .cloned()
                .unwrap();

            tokio::spawn(async move {
                shell.execute(message.take_body()).await;
                message.complete();
            });
            tokio::task::yield_now().await;
        }
    }

    fn compile(&mut self, identifier: impl AsRef<Identifier>, code: impl AsRef<[u8]>) {
        let shell = Shell::new(&self, code);

        self.shells.insert(identifier.as_ref().clone(), shell);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use tortuga_guest::{Method, Request, Response, Status};

    use super::*;

    #[tokio::test]
    async fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let body = Vec::from("Hello, World!");

        let mut runtime = Runtime::default();
        let request = Request::new(Method::Get, "/", Cursor::new(body.to_vec()));
        let response = Response::new(Status::Created, Cursor::new(body.to_vec()));

        let guest = runtime.welcome_guest(code);
        let actual = guest.queue(request);

        runtime.run().await;

        let mut actual = actual.await;
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
        let ping = runtime.welcome_guest(ping_code);

        runtime.welcome_guest(pong_code);

        let request = Request::new(Method::Get, "/ping".to_string(), Cursor::new(body.to_vec()));
        let response = Response::new(Status::Ok, Cursor::new(body.to_vec()));
        let actual = ping.queue(request);

        runtime.run().await;

        let mut actual = actual.await;
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
