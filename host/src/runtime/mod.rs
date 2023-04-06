//! The embedding runtime for the Tortuga WASM modules.

use std::io::Read;

use wasmtime::{Caller, Config, Engine, Linker, Module, Store};

use tortuga_guest::{Request, Response};

use crate::runtime::connection::{Connection, ForGuest, FromGuest};

mod connection;

pub struct Runtime {
    linker: Linker<Connection>,
}

pub struct Shell {
    module: Module,
}

impl Default for Runtime {
    fn default() -> Self {
        let configuration = Config::new();
        let engine = Engine::new(&configuration).unwrap();
        let mut linker = Linker::new(&engine);

        linker
            .func_wrap(
                "stream",
                "read",
                |mut caller: Caller<'_, Connection>, stream: u64, pointer: u32, length: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;

                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, connection): (&mut [u8], &mut Connection) =
                        memory.data_and_store_mut(&mut caller);
                    let body = &mut connection.stream(stream).unwrap().host_to_guest;
                    let size = (body.get_ref().len() - (body.position() as usize)).min(length);

                    let destination = &mut view[offset..(offset + size)];

                    body.read_exact(destination).unwrap();
                    destination.len() as u32
                },
            )
            .unwrap();
        linker
            .func_wrap(
                "stream",
                "write",
                |mut caller: Caller<'_, Connection>, stream: u64, pointer: u32, length: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;
                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, connection): (&mut [u8], &mut Connection) =
                        memory.data_and_store_mut(&mut caller);
                    let source = &view[offset..(offset + length)];

                    connection
                        .stream(stream)
                        .unwrap()
                        .guest_to_host
                        .get_mut()
                        .extend_from_slice(source);

                    source.len() as u32
                },
            )
            .unwrap();
        linker
            .func_wrap("stream", "start", |mut caller: Caller<'_, Connection>| {
                caller.data_mut().start_stream()
            })
            .unwrap();

        Runtime { linker }
    }
}

impl Runtime {
    pub fn load(&mut self, code: impl AsRef<[u8]>) -> Shell {
        // Modules can be compiled through either the text or binary format
        let module = Module::new(self.linker.engine(), code).unwrap();

        Shell { module }
    }

    pub fn execute(&mut self, shell: &Shell, request: Request<ForGuest>) -> Response<FromGuest> {
        let connection = Connection::new(request);
        let mut store = Store::new(self.linker.engine(), connection);

        let instance = self.linker.instantiate(&mut store, &shell.module).unwrap();
        let main = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap();

        // And finally we can call the wasm!
        main.call(&mut store, (0, 0)).unwrap();

        store.into_data().response()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

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
