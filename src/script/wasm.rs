use crate::context::RequestContext;
use crate::Script;
use bytes::Bytes;
use std::future::Future;
use std::io;
use std::path::PathBuf;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

pub struct Wasm {
    engine: Engine,
}

impl Wasm {
    pub fn new(wasm_cache: Option<&PathBuf>) -> io::Result<Self> {
        let mut configuration = Config::new();

        if let Some(path) = wasm_cache {
            configuration
                .cache_config_load(path)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        }

        configuration
            .async_support(true)
            .consume_fuel(true)
            .parallel_compilation(true);

        let engine =
            Engine::new(&configuration).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self { engine })
    }

    pub async fn run(
        &self,
        context: RequestContext,
        body: Bytes,
    ) -> Result<Bytes, wasmtime::Error> {
        let file = context.script().map_err(wasmtime::Error::msg)?;

        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        let stdout = WritePipe::new_in_memory();
        let mut builder = WasiCtxBuilder::new();

        for argument in context.arguments() {
            builder.arg(argument)?;
        }

        for (variable, value) in context.variables() {
            builder.env(variable, value)?;
        }

        let wasi = builder
            .stdin(Box::new(ReadPipe::from(body.as_ref())))
            .stdout(Box::new(stdout.clone()))
            .inherit_stderr()
            .build();

        let mut store = Store::new(&self.engine, wasi);

        store.set_fuel(1_000_000)?;
        store.fuel_async_yield_interval(Some(1_000))?;

        let module = Module::from_file(&self.engine, file)?;
        linker.module_async(&mut store, "", &module).await?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), ()>(&store)?
            .call_async(&mut store, ())
            .await?;

        drop(store);

        match stdout.try_into_inner() {
            Ok(output) => Ok(Bytes::from(output.into_inner())),
            Err(_) => Err(wasmtime::Error::msg(
                "Unable to extract output from CGI script",
            )),
        }
    }
}

impl Script for Wasm {
    fn invoke(
        &self,
        context: RequestContext,
        body: Bytes,
    ) -> impl Future<Output = io::Result<Bytes>> + Send {
        self.run(context, body)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
