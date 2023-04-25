use async_trait::async_trait;

use wasmtime::{Caller, Config, Engine, InstancePre, Linker, Module, Store};

use crate::wasm::{self, Connection, Data};

const EPOCH_YIELD_TICKS: u64 = 1;
const EPOCH_DEADLINE_TICKS: u64 = 500;

#[derive(Clone)]
pub struct Host<Factory> {
    engine: Engine,
    factory: Factory,
}

impl<Factory> Host<Factory> {
    pub fn tick(&mut self) {
        self.engine.increment_epoch()
    }
}

impl<Factory> From<Factory> for Host<Factory> {
    fn from(factory: Factory) -> Self {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.epoch_interruption(true);

        let engine = Engine::new(&configuration).unwrap();

        Host { engine, factory }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AdditionalState {
    ticks: u64,
}

impl AdditionalState {
    fn tick(&mut self) -> Result<u64, wasmtime::Error> {
        self.ticks += 1;

        if self.ticks <= EPOCH_DEADLINE_TICKS {
            Ok(1)
        } else {
            Err(wasmtime::Error::msg("exceeded allotted execution time"))
        }
    }
}

impl<Factory> wasm::Host for Host<Factory>
where
    Factory: wasm::Factory,
{
    type Guest = Guest<Factory>;
    type Error = wasmtime::Error;

    fn welcome<Code>(&mut self, code: Code) -> Result<Self::Guest, Self::Error>
    where
        Code: AsRef<[u8]>,
    {
        let mut linker = Linker::new(&self.engine);
        let module = Module::new(&self.engine, code)?;

        linker.func_wrap3_async(
            "stream",
            "read",
            move |mut caller: Caller<'_, Data<AdditionalState, Connection<Factory>>>,
                  stream: u64,
                  pointer: u32,
                  length: u32| {
                Box::new(async move {
                    let (buffer, optional_stream) =
                        data_and_stream_mut(&mut caller, stream, pointer, length);

                    match optional_stream {
                        None => -1,
                        Some(stream) => match wasm::Stream::read(stream, buffer).await {
                            Ok(bytes) => bytes as i64,
                            Err(_) => -1,
                        },
                    }
                })
            },
        )?;
        linker.func_wrap3_async(
            "stream",
            "write",
            move |mut caller: Caller<'_, Data<AdditionalState, Connection<Factory>>>,
                  stream: u64,
                  pointer: u32,
                  length: u32| {
                Box::new(async move {
                    let (buffer, optional_stream) =
                        data_and_stream_mut(&mut caller, stream, pointer, length);

                    match optional_stream {
                        None => -1,
                        Some(stream) => match wasm::Stream::write(stream, buffer).await {
                            Ok(bytes) => bytes as i64,
                            Err(_) => -1,
                        },
                    }
                })
            },
        )?;
        linker.func_wrap0_async(
            "stream",
            "start",
            move |mut caller: Caller<'_, Data<AdditionalState, Connection<Factory>>>| {
                Box::new(async move { caller.data_mut().connection_mut().start_stream() })
            },
        )?;

        let instance = linker.instantiate_pre(&module)?;

        Ok(Guest {
            instance,
            stream: self.factory.clone(),
        })
    }
}

pub struct Guest<Factory>
where
    Factory: wasm::Factory,
{
    instance: InstancePre<Data<AdditionalState, Connection<Factory>>>,
    stream: Factory,
}

#[async_trait]
impl<Factory> wasm::Guest for Guest<Factory>
where
    Factory: wasm::Factory,
{
    type Error = wasmtime::Error;
    type Stream = Factory::Stream;

    async fn invoke(&self, stream: Factory::Stream) -> Result<i32, Self::Error> {
        let connection = Connection::new(stream, self.stream.clone());
        let data = Data::new(connection, AdditionalState::default());

        let mut store = Store::new(self.instance.module().engine(), data);

        store.set_epoch_deadline(EPOCH_YIELD_TICKS);
        store.epoch_deadline_callback(|mut ctx| ctx.data_mut().additional_mut().tick());

        let instance = self.instance.instantiate_async(&mut store).await?;

        instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "main")
            .unwrap()
            .call_async(&mut store, (0, 0))
            .await
    }
}

fn data_and_stream_mut<'a, Factory>(
    caller: &'a mut Caller<'_, Data<AdditionalState, Connection<Factory>>>,
    stream: u64,
    pointer: u32,
    length: u32,
) -> (&'a mut [u8], Option<&'a mut Factory::Stream>)
where
    Factory: wasm::Factory,
{
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, state) = memory.data_and_store_mut(caller);
    let view = &mut data[..(pointer as usize + length as usize)][pointer as usize..];
    let stream = state.connection_mut().get_mut(stream);

    (view, stream)
}

#[cfg(all(test, feature = "memory"))]
mod tests {
    use crate::stream::memory;
    use crate::wasm::{wasmtime, Factory, Guest, Host};
    use std::io::Cursor;
    use tortuga_guest::{Method, Request, Response, Status};

    #[tokio::test]
    async fn execute_static() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_STATIC"));
        let body = Vec::from("Hello, World!");

        let mut buffer = Cursor::new(Vec::new());
        let mut bridge = memory::Bridge::default();
        let mut host = wasmtime::Host::from(bridge.clone());

        let request = Request::new(
            Method::Get,
            "/".into(),
            body.len(),
            Cursor::new(body.to_vec()),
        );
        let response = Response::new(Status::Ok, body.len(), Cursor::new(body.to_vec()));

        let guest = host.welcome(code).unwrap();
        let stream = bridge.create();

        bridge.write_message(0, request).unwrap();
        guest.invoke(stream).await.unwrap();

        let mut actual: Response<_> = bridge.read_message(0).unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
