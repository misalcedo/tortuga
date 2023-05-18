use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

use wasmtime::{Caller, Config, Engine, InstancePre, Linker, Module, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

#[derive(Debug)]
pub enum Error {
    Wasm(wasmtime::Error),
    WasiArgs(wasi_common::StringArrayError),
    MissingExport,
    MissingMemory,
    Poison,
    Internal,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<wasmtime::Error> for Error {
    fn from(value: wasmtime::Error) -> Self {
        Error::Wasm(value)
    }
}

impl From<wasi_common::StringArrayError> for Error {
    fn from(value: wasi_common::StringArrayError) -> Self {
        Error::WasiArgs(value)
    }
}

pub struct Host {
    engine: Engine,
    injection_count: u64,
    fuel_to_inject: u64,
}

pub struct Data {
    context: WasiCtx,
}

impl Data {
    pub fn new(context: WasiCtx) -> Self {
        Data { context }
    }

    pub fn context_mut(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

impl Host {
    pub fn new(injection_count: u64, fuel_to_inject: u64) -> Result<Self, Error> {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.consume_fuel(true);
        configuration.wasm_memory64(size_of::<usize>() == size_of::<u64>());

        let engine = Engine::new(&configuration)?;

        Ok(Host {
            engine,
            injection_count,
            fuel_to_inject,
        })
    }
}

impl Host {
    fn welcome<Code>(&self, code: Code) -> Result<Guest, Error>
    where
        Code: AsRef<[u8]>,
    {
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, Data::context_mut)?;

        linker.func_wrap2_async(
            "guest",
            "request",
            move |mut caller: Caller<'_, Data>, pointer: u32, length: u32| {
                Box::new(async move {
                    let memory = caller
                        .get_export("memory")
                        .ok_or(Error::MissingExport)?
                        .into_memory()
                        .ok_or(Error::MissingMemory)?;
                    let (data, store) = memory.data_and_store_mut(&mut caller);
                    let view = &data[..(pointer as usize + length as usize)][pointer as usize..];

                    Ok(0)
                })
            },
        )?;

        let module = Module::new(&self.engine, code)?;
        let instance = linker.instantiate_pre(&module)?;

        Ok(Guest {
            instance,
            injection_count: self.injection_count,
            fuel_to_inject: self.fuel_to_inject,
        })
    }
}

pub struct Guest {
    instance: InstancePre<Data>,
    injection_count: u64,
    fuel_to_inject: u64,
}

pub struct Configuration<Input, Output> {
    arguments: Vec<String>,
    environment: HashMap<String, String>,
    input: Input,
    output: Output,
}

impl<Input, Output> Configuration<Input, Output> {
    pub fn arguments(&self) -> &[String] {
        self.arguments.as_slice()
    }

    pub fn add_argument(&mut self, argument: String) {
        self.arguments.push(argument);
    }

    pub fn with_arguments(&mut self, arguments: Vec<String>) {
        self.arguments = arguments;
    }

    pub fn environment(&self) -> &HashMap<String, String> {
        &self.environment
    }

    pub fn set_environment_variable(&mut self, name: String, value: String) {
        self.environment.insert(name, value);
    }

    pub fn with_environment(&mut self, environment: HashMap<String, String>) {
        self.environment = environment;
    }
}

impl<Input, Output> Configuration<Input, Output>
where
    Input: Read + Send + Sync + 'static,
    Output: Write + Send + Sync + 'static,
{
    pub fn new(input: Input, output: Output) -> Self {
        Configuration {
            arguments: vec![],
            environment: HashMap::new(),
            input,
            output,
        }
    }
}

impl Guest {
    async fn invoke<Input, Output>(
        &self,
        configuration: Configuration<Input, Output>,
    ) -> Result<Output, Error>
    where
        Input: Read + Send + Sync + 'static,
        Output: Write + Send + Sync + 'static,
    {
        let output = Arc::new(RwLock::new(configuration.output));

        let context = WasiCtxBuilder::new()
            .stdin(Box::new(wasi_common::pipe::ReadPipe::new(
                configuration.input,
            )))
            .stdout(Box::new(wasi_common::pipe::WritePipe::from_shared(
                output.clone(),
            )))
            .inherit_stderr()
            .build();

        let mut store = Store::new(self.instance.module().engine(), Data::new(context));

        store.out_of_fuel_async_yield(self.injection_count, self.fuel_to_inject);

        let instance = self.instance.instantiate_async(&mut store).await?;

        instance
            .get_typed_func::<(), ()>(&mut store, "_start")?
            .call_async(&mut store, ())
            .await?;

        drop(store);

        match Arc::try_unwrap(output)
            .map_err(|_| Error::Internal)?
            .into_inner()
        {
            Ok(o) => Ok(o),
            Err(_) => Err(Error::Poison),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn error() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_WASI"));
        let host = Host::new(10, 1000).unwrap();
        let guest = host.welcome(code).unwrap();

        let input: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let output: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let configuration = Configuration::new(input, output);

        let response = guest.invoke(configuration).await.unwrap();

        assert_eq!(response.get_ref().as_slice(), b"Hello, world!\n");
    }
}
