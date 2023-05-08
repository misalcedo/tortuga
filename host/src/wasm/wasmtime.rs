use async_trait::async_trait;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::{Arc, RwLock};

use wasmtime::{Caller, Config, Engine, InstancePre, Linker, Module, Store};

use crate::wasm::{self, Connection, Data, Identifier};

const EPOCH_YIELD_TICKS: u64 = 1;
const EPOCH_DEADLINE_TICKS: u64 = 500;

pub struct Host<Primary, Factory, Rest> {
    engine: Engine,
    factory: Factory,
    guests: Arc<RwLock<HashMap<Identifier, Guest<Primary, Factory, Rest>>>>,
    epoch_deadline: u64,
}

impl<Primary, Factory, Rest> Clone for Host<Primary, Factory, Rest>
where
    Factory: Clone,
{
    fn clone(&self) -> Self {
        Host {
            engine: self.engine.clone(),
            factory: self.factory.clone(),
            guests: self.guests.clone(),
            epoch_deadline: self.epoch_deadline,
        }
    }
}

impl<Primary, Factory, Rest> Host<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    pub fn new(factory: Factory, epoch_deadline: u64) -> Self {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.epoch_interruption(true);

        let engine = Engine::new(&configuration).unwrap();

        Host {
            engine,
            factory,
            guests: Arc::new(RwLock::new(HashMap::new())),
            epoch_deadline,
        }
    }
}

impl<Primary, Factory, Rest> From<Factory> for Host<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    fn from(factory: Factory) -> Self {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.epoch_interruption(true);

        let engine = Engine::new(&configuration).unwrap();

        Host {
            engine,
            factory,
            guests: Arc::new(RwLock::new(HashMap::new())),
            epoch_deadline: EPOCH_DEADLINE_TICKS,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AdditionalState {
    ticks: u64,
    deadline: u64,
}

impl AdditionalState {
    fn new(deadline: u64) -> Self {
        AdditionalState { ticks: 0, deadline }
    }

    fn tick(&mut self) -> Result<u64, wasmtime::Error> {
        self.ticks += 1;

        if self.ticks <= self.deadline {
            Ok(1)
        } else {
            Err(wasmtime::Error::msg("exceeded allotted execution time"))
        }
    }
}

impl<Primary, Factory, Rest> wasm::Ticker for Host<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    fn tick(&mut self) {
        self.engine.increment_epoch()
    }
}

impl<Primary, Factory, Rest> wasm::Host for Host<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    type Guest = Guest<Primary, Factory, Rest>;
    type Error = wasmtime::Error;
    type Ticker = Self;

    fn welcome<Code>(&mut self, code: Code) -> Result<Identifier, Self::Error>
    where
        Code: AsRef<[u8]>,
    {
        let mut linker = Linker::new(&self.engine);
        let module = Module::new(&self.engine, code)?;

        linker.func_wrap3_async(
            "stream",
            "read",
            move |caller: Caller<'_, Data<AdditionalState, Connection<Primary, Factory, Rest>>>,
                  stream: u64,
                  pointer: u32,
                  length: u32| {
                Box::new(stream_read_write(
                    caller,
                    stream,
                    pointer,
                    length,
                    Action::Read,
                ))
            },
        )?;
        linker.func_wrap3_async(
            "stream",
            "write",
            move |caller: Caller<'_, Data<AdditionalState, Connection<Primary, Factory, Rest>>>,
                  stream: u64,
                  pointer: u32,
                  length: u32| {
                Box::new(stream_read_write(
                    caller,
                    stream,
                    pointer,
                    length,
                    Action::Write,
                ))
            },
        )?;
        linker.func_wrap0_async(
            "stream",
            "start",
            move |mut caller: Caller<
                '_,
                Data<AdditionalState, Connection<Primary, Factory, Rest>>,
            >| {
                Box::new(async move { caller.data_mut().connection_mut().start_stream() })
            },
        )?;

        let instance = linker.instantiate_pre(&module)?;
        let identifier = Identifier::default();
        let guest = Guest {
            instance,
            factory: self.factory.clone(),
            epoch_deadline: self.epoch_deadline,
        };

        let mut guests = match self.guests.write() {
            Ok(guests) => guests,
            Err(e) => e.into_inner(),
        };

        guests.insert(identifier, guest);

        Ok(identifier)
    }

    fn guest(&self, identifier: &Identifier) -> Option<Self::Guest> {
        let guests = match self.guests.read() {
            Ok(guests) => guests,
            Err(e) => e.into_inner(),
        };

        guests.get(identifier).cloned()
    }

    fn ticker(&self) -> Self::Ticker {
        self.clone()
    }
}

pub struct Guest<Primary, Factory, Rest> {
    instance: InstancePre<Data<AdditionalState, Connection<Primary, Factory, Rest>>>,
    factory: Factory,
    epoch_deadline: u64,
}

impl<Primary, Factory, Rest> Clone for Guest<Primary, Factory, Rest>
where
    Factory: Clone,
{
    fn clone(&self) -> Self {
        Guest {
            instance: self.instance.clone(),
            factory: self.factory.clone(),
            epoch_deadline: self.epoch_deadline,
        }
    }
}

#[async_trait]
impl<Primary, Factory, Rest> wasm::Guest for Guest<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    type Stream = Primary;
    type Error = wasmtime::Error;

    async fn invoke(&self, stream: Self::Stream) -> Result<i32, Self::Error> {
        let connection = Connection::new(stream, self.factory.clone());
        let data = Data::new(connection, AdditionalState::new(self.epoch_deadline));

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

enum Action {
    Read,
    Write,
}

impl Action {
    async fn perform<Stream>(&self, stream: &mut Stream, buffer: &mut [u8]) -> i64
    where
        Stream: wasm::Stream,
    {
        match self {
            Action::Read => match Stream::read(stream, buffer).await {
                Ok(bytes) => bytes as i64,
                Err(_) => -1,
            },
            Action::Write => match Stream::write(stream, buffer).await {
                Ok(bytes) => bytes as i64,
                Err(_) => -1,
            },
        }
    }
}

async fn stream_read_write<Primary, Factory, Rest>(
    mut caller: Caller<'_, Data<AdditionalState, Connection<Primary, Factory, Rest>>>,
    stream: u64,
    pointer: u32,
    length: u32,
    action: Action,
) -> i64
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, state) = memory.data_and_store_mut(&mut caller);
    let view = &mut data[..(pointer as usize + length as usize)][pointer as usize..];

    match NonZeroU64::new(stream) {
        None => {
            let stream = state.connection_mut().primary_mut();

            action.perform(stream, view).await
        }
        Some(stream) => match state.connection_mut().get_mut(stream) {
            None => -1,
            Some(stream) => action.perform(stream, view).await,
        },
    }
}

#[cfg(all(test, feature = "memory"))]
mod tests {
    use crate::executor;
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

        let identifier = host.welcome(code).unwrap();
        let guest = host.guest(&identifier).unwrap();
        let stream = bridge.create();

        bridge.write_message(0, request).unwrap();
        guest.invoke(stream).await.unwrap();

        let mut actual: Response<_> = bridge.read_message(0).unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(actual, response);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn sample_timeout() {
        let mut bridge = memory::Bridge::default();
        let mut host = wasmtime::Host::new(bridge.clone(), 3);

        let infinite_code = include_bytes!(env!("CARGO_BIN_FILE_INFINITE"));
        let infinite = host.welcome(infinite_code).unwrap();
        let guest = host.guest(&infinite).unwrap();

        let ticker = executor::tokio::schedule_tick(&host);

        let stream = bridge.create();
        let request = Request::new(Method::Get, "/infinite".into(), 0, Cursor::new(Vec::new()));

        bridge.write_message(0, request).unwrap();

        assert!(guest.invoke(stream).await.is_err());

        ticker.abort();
    }
}
