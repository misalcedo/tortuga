//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use tokio::task::JoinHandle;
use wasmtime::{Config, Engine};

pub use connection::Connection;

use channel::{new_channel, Receiver, Sender};
pub use guest::Guest;
pub use identifier::Identifier;
use message::Message;
pub use router::Router;
pub use shell::Shell;
use tortuga_guest::{Request, Source};
pub use uri::Uri;

mod channel;
mod connection;
mod guest;
mod identifier;
mod message;
mod router;
mod shell;
mod uri;

pub struct Runtime {
    engine: Engine,
    router: Router,
    shells: HashMap<Identifier, Shell>,
    channel: (Sender<Message>, Receiver<Message>),
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime::from(Router::default())
    }
}

impl From<Router> for Runtime {
    fn from(router: Router) -> Self {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.consume_fuel(true);

        let engine = Engine::new(&configuration).unwrap();

        Runtime {
            engine,
            router,
            shells: Default::default(),
            channel: new_channel(),
        }
    }
}

impl Runtime {
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub fn welcome_guest(&mut self, code: impl AsRef<[u8]>) -> Guest {
        let guest = Guest::new(self.channel.0.clone());
        let shell = Shell::new(&self, code);

        self.shells.insert(guest.identifier(), shell);

        guest
    }

    pub async fn run(&mut self) {
        if let Some(mut message) = self.channel.1.recv().await {
            let mut stream = message.take_body();
            let identifier = match message.to() {
                Some(identifier) => identifier,
                None => {
                    let reader = stream.peek();
                    let response: Request<_> = reader.read_message().unwrap();

                    self.router
                        .route(response.method(), response.uri())
                        .unwrap()
                        .identifier()
                }
            };

            let shell = self.shells.get(&identifier).unwrap().clone();
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
                    // TODO: map URI to identifier.
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
