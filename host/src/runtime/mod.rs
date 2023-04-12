//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use wasmtime::{Config, Engine};
use wasmtime_wasi::WasiCtxBuilder;

pub use connection::Connection;
use tortuga_guest::{Body, Destination, Request, Response};

use crate::runtime::channel::ChannelStream;
use connection::FromGuest;
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
        request: Request<impl Body>,
    ) -> Response<FromGuest> {
        let shell = self.shells.get_mut(identifier.as_ref()).unwrap();
        let mut primary = ChannelStream::default();

        primary.write_message(request).unwrap();
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
        let shell = Shell::new(&self.engine, code, self.channel.0.clone());

        self.shells.insert(identifier.as_ref().clone(), shell);
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};
    use tortuga_guest::{Method, Status};
    use wasmtime::{Caller, Linker, Module, Store};

    use super::*;

    #[test]
    fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let body = Vec::from("Hello, World!");

        let mut runtime = Runtime::default();
        let request = Request::new(Method::Get, "/", Cursor::new(body.to_vec()));
        let response = Response::new(Status::Created, Cursor::new(body.to_vec()));

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

        let request = Request::new(Method::Get, "/ping".to_string(), Cursor::new(body.to_vec()));
        let response = Response::new(Status::Ok, Cursor::new(body.to_vec()));
        let actual = ping.execute(request);

        runtime.run();

        let mut actual = actual.await;
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    #[tokio::test]
    async fn runtime() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_PONG"));
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.consume_fuel(true);

        let engine = Engine::new(&configuration).unwrap();

        let module = Module::new(&engine, code).unwrap();
        let mut linker = Linker::new(&engine);

        linker
            .func_wrap3_async(
                "stream",
                "read",
                |mut caller: Caller<'_, Connection>, stream: u64, pointer: u32, length: u32| {
                    Box::new(async move {
                        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                        let (view, connection): (&mut [u8], &mut Connection) =
                            memory.data_and_store_mut(&mut caller);
                        let destination =
                            &mut view[..(pointer as usize + length as usize)][pointer as usize..];

                        connection
                            .stream(stream)
                            .unwrap()
                            .read(destination)
                            .unwrap() as i64
                    })
                },
            )
            .unwrap();
        linker
            .func_wrap3_async(
                "stream",
                "write",
                |mut caller: Caller<'_, Connection>, stream: u64, pointer: u32, length: u32| {
                    Box::new(async move {
                        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                        let (view, connection): (&mut [u8], &mut Connection) =
                            memory.data_and_store_mut(&mut caller);
                        let source =
                            &view[..(pointer as usize + length as usize)][pointer as usize..];

                        connection.stream(stream).unwrap().write(source).unwrap() as i64
                    })
                },
            )
            .unwrap();
        linker
            .func_wrap0_async("stream", "start", |mut caller: Caller<'_, Connection>| {
                // TODO: Needs to create an async mechanism in data that allows the runtime handle requests and return responses. A channel stream would probably suffice.
                Box::new(async move { caller.data_mut().start_stream() })
            })
            .unwrap();

        let body = b"PONG!";
        let request = Request::new(
            Method::Get,
            "/pong".to_string(),
            Cursor::new(b"PING!".to_vec()),
        );
        let mut primary = ChannelStream::default();

        primary.write_message(request).unwrap();

        let mut store = Store::new(&engine, Connection::new(primary));

        store.add_fuel(1000).unwrap();
        store.out_of_fuel_async_yield(1000, 1000);

        let instance_pre = linker.instantiate_pre(&module).unwrap();
        let instance = instance_pre.instantiate_async(&mut store).await.unwrap();

        instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap()
            .call_async(&mut store, (0, 0))
            .await
            .unwrap();

        let mut response = store.into_data().response();
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(response.body(), &mut buffer).unwrap();

        assert_eq!(response.status(), Status::Ok.into());
        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
