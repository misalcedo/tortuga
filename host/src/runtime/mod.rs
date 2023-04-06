//! The embedding runtime for the Tortuga WASM modules.

use wasmtime::{Config, Engine};

pub use connection::Connection;
use tortuga_guest::{Request, Response};

use crate::runtime::connection::{ForGuest, FromGuest};
use crate::runtime::shell::Shell;

mod connection;
mod shell;

pub struct Runtime {
    engine: Engine,
}

impl Default for Runtime {
    fn default() -> Self {
        let configuration = Config::new();
        let engine = Engine::new(&configuration).unwrap();

        Runtime { engine }
    }
}

impl Runtime {
    pub fn load(&mut self, code: impl AsRef<[u8]>) -> Shell {
        Shell::new(self, code)
    }

    pub fn execute(&mut self, shell: &Shell, request: Request<ForGuest>) -> Response<FromGuest> {
        shell.execute(request)
    }

    fn engine(&self) -> &Engine {
        &self.engine
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

        let shell = runtime.load(code);
        let mut actual = runtime.execute(&shell, request);
        let mut buffer = vec![0; body.len()];

        actual.body().read_exact(buffer.as_mut_slice()).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer, body);
    }
}
