//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use std::sync::RwLock;
use tortuga_guest::{Method, Status};
use wasmtime::{Caller, Config, Engine, ExternRef, Linker, Module, Store};

pub struct Runtime {
    linker: Linker<Assignment>,
}

pub struct Shell {
    module: Module,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Request {
    method: Method,
    uri: String,
    message: Message,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Response {
    status: Status,
    message: Message,
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum HeaderName {
    Authorization,
    Accept,
    Link,
    Custom(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum HeaderValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Headers {
    headers: HashMap<HeaderName, HeaderValue>,
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Body {
    bytes: Vec<u8>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Message {
    headers: Headers,
    body: Body,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Assignment {
    request: Request,
    response: Response,
}

impl Default for Runtime {
    fn default() -> Self {
        let configuration = Config::new();
        let engine = Engine::new(&configuration).unwrap();
        let mut linker = Linker::new(&engine);

        linker
            .func_wrap(
                "request",
                "read_uri",
                |mut caller: Caller<'_, Assignment>, pointer: u32, length: u32, start: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;
                    let start = start as usize;

                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, assignment) = memory.data_and_store_mut(&mut caller);
                    let body = &mut assignment.request.uri.as_bytes();
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
                "request",
                "read_body",
                |mut caller: Caller<'_, Assignment>, pointer: u32, length: u32, start: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;
                    let start = start as usize;

                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, assignment) = memory.data_and_store_mut(&mut caller);
                    let body = &mut assignment.request.message.body.bytes;
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
                "response",
                "write_body",
                |mut caller: Caller<'_, Assignment>, pointer: u32, length: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;
                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, assignment) = memory.data_and_store_mut(&mut caller);
                    let source = &view[offset..(offset + length)];

                    assignment
                        .response
                        .message
                        .body
                        .bytes
                        .extend_from_slice(source);

                    source.len() as u32
                },
            )
            .unwrap();
        linker
            .func_wrap(
                "response",
                "set_status",
                |mut caller: Caller<'_, Assignment>, status: u32| {
                    let data = caller.data_mut();
                    data.response.status = Status::from(status);
                },
            )
            .unwrap();
        linker
            .func_wrap("response", "status", |caller: Caller<'_, Assignment>| {
                caller.data().response.status as u32
            })
            .unwrap();
        linker
            .func_wrap("message", "call", || {
                let mut response = Response::default();
                response.status = Status::MultipleChoices;
                Some(ExternRef::new(RwLock::new(response)))
            })
            .unwrap();
        linker
            .func_wrap(
                "message",
                "status",
                |message: Option<ExternRef>| match message {
                    None => 0,
                    Some(message) => {
                        let response: Option<&Response> = message.data().downcast_ref();
                        match response {
                            None => 0,
                            Some(response) => response.status as u32,
                        }
                    }
                },
            )
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

    pub fn execute(&mut self, shell: &Shell, data: Assignment) -> Assignment {
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
        let mut expected = Assignment::default();
        let shell = runtime.load(code);

        expected.response.status = Status::Ok;

        assert_eq!(runtime.execute(&shell, Assignment::default()), expected)
    }

    #[test]
    fn execute_echo() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_ECHO"));
        let mut runtime = Runtime::default();
        let mut expected = Assignment::default();

        expected.request.uri = "/".to_string();
        expected.request.message.body.bytes = Vec::from("Hello, World!");

        let actual = expected.clone();
        let shell = runtime.load(code);

        expected.response.message.body.bytes = expected.request.message.body.bytes.clone();
        expected.response.status = Status::Created;

        assert_eq!(runtime.execute(&shell, actual), expected)
    }
}
