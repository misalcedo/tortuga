//! The embedding runtime for the Tortuga WASM modules.

use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::num::NonZeroU64;
use std::sync::RwLock;
use tortuga_guest::{FrameType, Method, Request, Response, Status};
use wasmtime::{Caller, Config, Engine, ExternRef, Linker, Module, Store};

pub struct Runtime {
    linker: Linker<Connection>,
}

pub struct Shell {
    module: Module,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct BidirectionalStream {
    host_to_guest: Cursor<Vec<u8>>,
    guest_to_host: Cursor<Vec<u8>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connection {
    primary: BidirectionalStream,
    streams: HashMap<NonZeroU64, BidirectionalStream>,
}

impl Connection {
    pub fn stream(&mut self, stream: u64) -> Option<&mut BidirectionalStream> {
        match stream {
            0 => Some(&mut self.primary),
            _ => NonZeroU64::new(stream).map(|id| self.streams.get_mut(&id)),
        }
    }

    pub fn start_stream(&mut self) -> u64 {
        let id = 1 + self.streams.len() as u64;

        self.streams
            .insert(NonZeroU64::new(stream).unwrap(), Default::default());

        id
    }
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
                    let body = connection.stream(stream).unwrap().host_to_guest.get_mut();
                    let end = body.len().min(start + length);
                    let body_slice = &body[start..end];

                    let destination = &mut view[offset..(offset + body_slice.len())];

                    destination.copy_from_slice(body_slice);
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

    pub fn execute(&mut self, shell: &Shell, data: Connection) -> Connection {
        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using the unit of work for.
        let mut store = Store::new(self.linker.engine(), data);

        let instance = self.linker.instantiate(&mut store, &shell.module).unwrap();
        let main = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap();

        // And finally we can call the wasm!
        main.call(&mut store, (0, 0)).unwrap();

        store.into_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_shell() {
        let code = include_str!("../../examples/status.wat");
        let mut runtime = Runtime::default();
        let mut expected = Connection::default();
        let shell = runtime.load(code);

        expected.response.status = Status::Ok;

        assert_eq!(runtime.execute(&shell, Connection::default()), expected)
    }

    #[test]
    fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let mut runtime = Runtime::default();
        let mut expected = Connection::default();

        expected.request.uri = "/".to_string();
        expected.request.message.body.bytes = Vec::from("Hello, World!");

        let actual = expected.clone();
        let shell = runtime.load(code);

        expected.response.message.body.bytes = expected.request.message.body.bytes.clone();
        expected.response.status = Status::Created;

        assert_eq!(runtime.execute(&shell, actual), expected)
    }
}
