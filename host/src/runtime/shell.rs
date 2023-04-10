use crate::runtime::connection::{ForGuest, FromGuest};
use crate::runtime::Connection;
use std::io::Read;
use tortuga_guest::{Request, Response};
use wasmtime::{Caller, Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtx;

pub struct Shell {
    module: Module,
    state: Option<State>,
    linker: Linker<State>,
}

struct State {
    connection: Connection,
    ctx: WasiCtx,
}

impl From<WasiCtx> for State {
    fn from(ctx: WasiCtx) -> Self {
        State {
            connection: Default::default(),
            ctx,
        }
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

        Shell {
            module,
            linker,
            state: None,
        }
    }

    pub fn promote(&mut self, ctx: WasiCtx) {
        wasmtime_wasi::add_to_linker(&mut self.linker, |s| &mut s.connection).unwrap();
        self.state = Some(State::new(ctx));
    }

    pub fn execute(&self, request: Request<ForGuest>) -> Response<FromGuest> {
        let connection = Connection::new(request);
        let mut store = Store::new(self.linker.engine(), connection);

        let instance = self.linker.instantiate(&mut store, &self.module).unwrap();
        let main = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap();

        main.call(&mut store, (0, 0)).unwrap();

        store.into_data().response()
    }
}
