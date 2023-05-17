use async_trait::async_trait;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::{Arc, RwLock};

use wasmtime::{Caller, Config, Engine, InstancePre, Linker, Module, Store};

use crate::wasm::{self, Connection, Data, Identifier};

const FUEL_TO_INJECT: u64 = 1000;
const INJECTION_COUNT: u64 = 500;

pub struct Host<Primary, Factory, Rest> {
    engine: Engine,
    factory: Factory,
    guests: Arc<RwLock<HashMap<Identifier, Guest<Primary, Factory, Rest>>>>,
    fuel_to_inject: u64,
    injection_count: u64,
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
            fuel_to_inject: self.fuel_to_inject,
            injection_count: self.injection_count,
        }
    }
}

impl<Primary, Factory, Rest> Host<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    pub fn new(factory: Factory, fuel_to_inject: u64, injection_count: u64) -> Self {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.epoch_interruption(true);

        let engine = Engine::new(&configuration).unwrap();

        Host {
            engine,
            factory,
            guests: Arc::new(RwLock::new(HashMap::new())),
            fuel_to_inject,
            injection_count,
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
        configuration.consume_fuel(true);

        let engine = Engine::new(&configuration).unwrap();

        Host {
            engine,
            factory,
            guests: Arc::new(RwLock::new(HashMap::new())),
            fuel_to_inject: FUEL_TO_INJECT,
            injection_count: INJECTION_COUNT,
        }
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

    fn welcome<Code>(&mut self, code: Code) -> Result<Identifier, Self::Error>
    where
        Code: AsRef<[u8]>,
    {
        let mut linker = Linker::new(&self.engine);
        let module = Module::new(&self.engine, code)?;

        linker.func_wrap3_async(
            "stream",
            "read",
            move |caller: Caller<'_, Data<(), Connection<Primary, Factory, Rest>>>,
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
            move |caller: Caller<'_, Data<(), Connection<Primary, Factory, Rest>>>,
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
            move |mut caller: Caller<'_, Data<(), Connection<Primary, Factory, Rest>>>| {
                Box::new(async move { caller.data_mut().connection_mut().start_stream() })
            },
        )?;

        let instance = linker.instantiate_pre(&module)?;
        let identifier = Identifier::default();
        let guest = Guest {
            instance,
            factory: self.factory.clone(),
            fuel_to_inject: self.fuel_to_inject,
            injection_count: self.injection_count,
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
}

pub struct Guest<Primary, Factory, Rest> {
    instance: InstancePre<Data<(), Connection<Primary, Factory, Rest>>>,
    factory: Factory,
    fuel_to_inject: u64,
    injection_count: u64,
}

impl<Primary, Factory, Rest> Clone for Guest<Primary, Factory, Rest>
where
    Factory: Clone,
{
    fn clone(&self) -> Self {
        Guest {
            instance: self.instance.clone(),
            factory: self.factory.clone(),
            fuel_to_inject: self.fuel_to_inject,
            injection_count: self.injection_count,
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
        let data = Data::new(connection, ());

        let mut store = Store::new(self.instance.module().engine(), data);

        store.out_of_fuel_async_yield(self.injection_count, self.fuel_to_inject);

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
    mut caller: Caller<'_, Data<(), Connection<Primary, Factory, Rest>>>,
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

    #[tokio::test]
    async fn sample_timeout() {
        let mut bridge = memory::Bridge::default();
        let mut host = wasmtime::Host::new(bridge.clone(), 500, 1);

        let infinite_code = include_bytes!(env!("CARGO_BIN_FILE_INFINITE"));
        let infinite = host.welcome(infinite_code).unwrap();
        let guest = host.guest(&infinite).unwrap();

        let stream = bridge.create();
        let request = Request::new(Method::Get, "/infinite".into(), 0, Cursor::new(Vec::new()));

        bridge.write_message(0, request).unwrap();

        assert!(guest.invoke(stream).await.is_err());
    }
}
