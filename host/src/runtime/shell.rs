use std::io::{Read, Write};

use wasmtime::{Caller, Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtx;

use tortuga_guest::{Bidirectional, MemoryStream, Response};

use crate::runtime::connection::FromGuest;
use crate::runtime::Connection;

pub struct Shell {
    module: Module,
    ctx: Option<WasiCtx>,
    linker: Linker<State>,
}

struct State {
    connection: Connection,
    ctx: Option<WasiCtx>,
}

impl State {
    fn new(connection: Connection, ctx: Option<WasiCtx>) -> Self {
        State { connection, ctx }
    }
}

impl Shell {
    pub fn new(engine: &Engine, code: impl AsRef<[u8]>) -> Self {
        let module = Module::new(engine, code).unwrap();
        let mut linker = Linker::new(&engine);

        linker
            .func_wrap(
                "stream",
                "read",
                |mut caller: Caller<'_, State>, stream: u64, pointer: u32, length: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;

                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, state): (&mut [u8], &mut State) =
                        memory.data_and_store_mut(&mut caller);
                    let connection = &mut state.connection;
                    let body = connection.stream(stream).unwrap();
                    let size = body.remaining().min(length);

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
                |mut caller: Caller<'_, State>, stream: u64, pointer: u32, length: u32| {
                    let offset = pointer as usize;
                    let length = length as usize;

                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (view, state): (&mut [u8], &mut State) =
                        memory.data_and_store_mut(&mut caller);
                    let connection = &mut state.connection;
                    let source = &view[offset..(offset + length)];

                    connection
                        .stream(stream)
                        .unwrap()
                        .write_all(source)
                        .unwrap();

                    source.len() as u32
                },
            )
            .unwrap();
        linker
            .func_wrap("stream", "start", |mut caller: Caller<'_, State>| {
                caller.data_mut().connection.start_stream()
            })
            .unwrap();

        Shell {
            module,
            linker,
            ctx: None,
        }
    }

    pub fn promote(&mut self, ctx: WasiCtx) {
        wasmtime_wasi::add_to_linker(&mut self.linker, |s| s.ctx.as_mut().unwrap()).unwrap();
        self.ctx = Some(ctx);
    }

    pub fn execute(&mut self, stream: MemoryStream<Bidirectional>) -> Response<FromGuest> {
        let connection = Connection::new(stream);
        let ctx = self.ctx.take();
        let state = State::new(connection, ctx);

        let mut store = Store::new(self.linker.engine(), state);

        let instance = self.linker.instantiate(&mut store, &self.module).unwrap();
        let main = instance.get_typed_func::<(i32, i32), i32>(&mut store, "main");

        if main.is_ok() {
            let result = main.unwrap().call(&mut store, (0, 0));

            if result.is_ok() {
                return store.into_data().connection.response();
            } else {
                self.ctx = store.into_data().ctx;
                result.unwrap();
            }
        } else {
            self.ctx = store.into_data().ctx;
            main.unwrap();
        }

        unreachable!();
    }
}
