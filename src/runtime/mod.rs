//! The embedding runtime for the Tortuga WASM modules.

use std::collections::HashMap;
use wasmtime::{Caller, Engine, Linker, Module, Store};

#[derive(Default)]
pub struct Runtime {
    engine: Engine,
}

pub struct Shell {
    module: Module,
}

/// HTTP defines a set of request methods to indicate the desired action to be performed for a given resource.
/// Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs.
/// Each of them implements a different semantic, but some common features are shared by a group of them:
/// e.g. a request method can be safe, idempotent, or cacheable.
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Method {
    /// The `GET` method requests a representation of the specified resource. Requests using GET should only retrieve data.
    #[default]
    Get,
    /// The `HEAD` method asks for a response identical to a `GET` request, but without the response body.
    Head,
    /// The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    Post,
    /// The `PUT` method replaces all current representations of the target resource with the request payload.
    Put,
    /// The `DELETE` method deletes the specified resource.
    Delete,
    /// The `CONNECT` method establishes a tunnel to the server identified by the target resource.
    Connect,
    /// The `OPTIONS` method describes the communication options for the target resource.
    Options,
    /// The `TRACE` method performs a message loop-back test along the path to the target resource.
    Trace,
    /// The `PATCH` method applies partial modifications to a resource.
    Patch,
    Custom(String),
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(u16)]
pub enum Status {
    #[default]
    Ok = 200,
    BadRequest = 400,
    ServerError = 500,
}

impl From<u32> for Status {
    fn from(_: u32) -> Self {
        Status::Ok
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Request {
    method: Method,
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
pub struct UnitOfWork {
    request: Request,
    response: Response,
}

impl Runtime {
    pub fn load(&mut self, code: impl AsRef<[u8]>) -> Shell {
        // Modules can be compiled through either the text or binary format
        let module = Module::new(&self.engine, code).unwrap();

        Shell { module }
    }

    pub fn execute(&mut self, shell: &Shell, data: UnitOfWork) -> UnitOfWork {
        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using the unit of work for.
        let mut store = Store::new(&self.engine, data);

        // Create a `Linker` which will be later used to instantiate this module.
        // Host functionality is defined by name within the `Linker`.
        let mut linker = Linker::new(&self.engine);

        linker
            .func_wrap(
                "response",
                "set_status",
                |mut caller: Caller<'_, UnitOfWork>, x: u32| {
                    let data = caller.data_mut();
                    data.response.status = Status::from(x * 2);
                },
            )
            .unwrap();
        linker
            .func_wrap("response", "status", |caller: Caller<'_, UnitOfWork>| {
                caller.data().response.status as u32
            })
            .unwrap();

        let instance = linker.instantiate(&mut store, &shell.module).unwrap();
        let hello = instance
            .get_typed_func::<(), ()>(&mut store, "hello")
            .unwrap();

        // And finally we can call the wasm!
        hello.call(&mut store, ()).unwrap();

        store.into_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_shell() {
        let code = r#"
        (module
            (import "response" "set_status" (func $response_set_status (param i32)))

            (func (export "hello")
                i32.const 3
                call $response_set_status)
        )
    "#;
        let mut runtime = Runtime::default();
        let shell = runtime.load(code);
        let mut uow = UnitOfWork::default();

        uow.response.status = Status::Ok;

        assert_eq!(runtime.execute(&shell, UnitOfWork::default()), uow)
    }
}
