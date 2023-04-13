use std::io::{Read, Write};

use wasmtime::{Caller, InstancePre, Linker, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use crate::runtime::channel::ChannelStream;
use crate::Runtime;
use tortuga_guest::Response;

use crate::runtime::connection::FromGuest;
use crate::runtime::message::Message;
use crate::runtime::Connection;

pub struct Shell {
    ctx: Option<WasiCtx>,
    factory: InstancePre<State>,
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
    pub fn new(runtime: &Runtime, code: impl AsRef<[u8]>, plugin: bool) -> Self {
        let mut ctx = None;
        let mut linker = Linker::new(&runtime.engine);
        let module = Module::new(&runtime.engine, code).unwrap();
        let sender = runtime.channel.0.clone();

        if plugin {
            wasmtime_wasi::add_to_linker(&mut linker, |s: &mut State| s.ctx.as_mut().unwrap())
                .unwrap();
            ctx = Some(WasiCtxBuilder::new().build());
        }

        linker
            .func_wrap3_async(
                "stream",
                "read",
                |mut caller: Caller<'_, State>, stream: u64, pointer: u32, length: u32| {
                    Box::new(async move {
                        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                        let (view, state): (&mut [u8], &mut State) =
                            memory.data_and_store_mut(&mut caller);
                        let destination =
                            &mut view[..(pointer as usize + length as usize)][pointer as usize..];

                        state
                            .connection
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
                |mut caller: Caller<'_, State>, stream: u64, pointer: u32, length: u32| {
                    Box::new(async move {
                        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                        let (view, state): (&mut [u8], &mut State) =
                            memory.data_and_store_mut(&mut caller);
                        let source =
                            &view[..(pointer as usize + length as usize)][pointer as usize..];

                        state
                            .connection
                            .stream(stream)
                            .unwrap()
                            .write(source)
                            .unwrap() as i64
                    })
                },
            )
            .unwrap();
        linker
            .func_wrap0_async("stream", "start", move |mut caller: Caller<'_, State>| {
                let sender = sender.clone();

                Box::new(async move {
                    let (guest, host) = ChannelStream::new();

                    sender.send(Message::from(guest)).await;
                    caller.data_mut().connection.add_stream(host)
                })
            })
            .unwrap();

        let factory = linker.instantiate_pre(&module).unwrap();

        Shell { factory, ctx }
    }

    pub async fn execute(&mut self, stream: ChannelStream) -> Response<FromGuest> {
        let connection = Connection::new(stream);
        let ctx = self.ctx.take();

        let mut state = State::new(connection, ctx);
        let mut store = Store::new(self.factory.module().engine(), state);

        store.add_fuel(1000).unwrap();
        store.out_of_fuel_async_yield(10000, 1000);

        let instance = self.factory.instantiate_async(&mut store).await.unwrap();

        instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap()
            .call_async(&mut store, (0, 0))
            .await
            .unwrap();

        state = store.into_data();

        self.ctx = state.ctx;

        state.connection.response()
    }
}
