//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use wasmtime::{Config, Engine};

pub use connection::Connection;
use tortuga_guest::{Request, Response};

use connection::{ForGuest, FromGuest};
use guest::Guest;
pub use identifier::Identifier;
use plugin::Plugin;
pub use shell::Shell;
pub use uri::Uri;

mod connection;
mod guest;
mod identifier;
mod plugin;
mod shell;
mod uri;

pub struct Runtime {
    engine: Engine,
    guests: HashMap<Identifier, Guest>,
    plugins: HashMap<Identifier, Plugin>,
    shells: HashMap<Identifier, Shell>,
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
        }
    }
}

impl Runtime {
    pub fn load_plugin(&mut self, uri: String, code: impl AsRef<[u8]>) -> Plugin {
        let plugin = Plugin::new(uri);

        self.plugins.insert(plugin.identifier(), plugin.clone());
        self.compile(&plugin, code);

        plugin
    }

    pub fn welcome_guest(&mut self, uri: String, code: impl AsRef<[u8]>) -> Guest {
        let guest = Guest::new(uri);

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

    fn compile(&mut self, identifier: impl AsRef<Identifier>, code: impl AsRef<[u8]>) {
        let shell = Shell::new(&self.engine, code);

        self.shells.insert(identifier.as_ref().clone(), shell);
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use tortuga_guest::Status;

    use super::*;

    #[test]
    fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let body = Vec::from("Hello, World!");

        let mut runtime = Runtime::default();
        let mut request = Request::default();
        let mut response = Response::with_status(Status::Created);

        request.body().write_all(&body).unwrap();
        response.body().write_all(&body).unwrap();

        let guest = runtime.welcome_guest("/".to_string(), code);
        let mut actual = runtime.execute(&guest, request);
        let mut buffer = vec![0; body.len()];

        actual.body().read_exact(buffer.as_mut_slice()).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer, body);
    }
}
