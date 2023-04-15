use wasmtime::{Caller, InstancePre, Linker, Module, Store};

use crate::runtime::channel::ChannelStream;
use crate::Runtime;

use crate::runtime::message::Message;
use crate::runtime::{Connection, Identifier};

#[derive(Clone)]
pub struct Shell {
    factory: InstancePre<State>,
}

pub struct State {
    connection: Connection,
}

impl State {
    fn new(connection: Connection) -> Self {
        State { connection }
    }
}

impl Shell {
    pub fn new(runtime: &Runtime, code: impl AsRef<[u8]>, identifier: Identifier) -> Self {
        let mut linker = Linker::new(&runtime.engine);
        let module = Module::new(&runtime.engine, code).unwrap();
        let sender = runtime.channel.0.clone();

        linker
            .func_wrap3_async(
                "stream",
                "read",
                move |mut caller: Caller<'_, State>, stream: u64, pointer: u32, length: u32| {
                    Box::new(async move {
                        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                        let (data, state): (&mut [u8], &mut State) =
                            memory.data_and_store_mut(&mut caller);
                        let buffer =
                            &mut data[..(pointer as usize + length as usize)][pointer as usize..];

                        match state
                            .connection
                            .stream(stream)
                            .unwrap()
                            .read_async(buffer)
                            .await
                        {
                            Ok(bytes) => {
                                println!(
                                    "Reading bytes from stream {} of {:?}: {:?}",
                                    stream,
                                    identifier,
                                    String::from_utf8_lossy(buffer)
                                );
                                bytes as i64
                            }
                            Err(_) => -1,
                        }
                    })
                },
            )
            .unwrap();
        linker
            .func_wrap3_async(
                "stream",
                "write",
                move |mut caller: Caller<'_, State>, stream: u64, pointer: u32, length: u32| {
                    Box::new(async move {
                        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                        let (data, state): (&mut [u8], &mut State) =
                            memory.data_and_store_mut(&mut caller);
                        let buffer =
                            &data[..(pointer as usize + length as usize)][pointer as usize..];

                        println!(
                            "Writing bytes to stream {} of {:?}: {:?}",
                            stream,
                            identifier,
                            String::from_utf8_lossy(buffer)
                        );
                        match state
                            .connection
                            .stream(stream)
                            .unwrap()
                            .write_async(buffer)
                            .await
                        {
                            Ok(bytes) => bytes as i64,
                            Err(_) => -1,
                        }
                    })
                },
            )
            .unwrap();
        linker
            .func_wrap0_async("stream", "start", move |mut caller: Caller<'_, State>| {
                let sender = sender.clone();

                Box::new(async move {
                    let (client, server) = ChannelStream::new();

                    sender.send(Message::from(server)).await;
                    caller.data_mut().connection.add_stream(client)
                })
            })
            .unwrap();

        let factory = linker.instantiate_pre(&module).unwrap();

        Shell { factory }
    }

    pub async fn execute(&self, stream: ChannelStream) {
        let state = State::new(Connection::new(stream));
        let mut store = Store::new(self.factory.module().engine(), state);

        store.add_fuel(1000).unwrap();
        store.out_of_fuel_async_yield(100000000, 1000);

        let instance = self.factory.instantiate_async(&mut store).await.unwrap();

        instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap()
            .call_async(&mut store, (0, 0))
            .await
            .unwrap();
    }
}
