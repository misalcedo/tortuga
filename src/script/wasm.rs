use crate::context::RequestContext;
use bytes::Bytes;
use std::io;
use std::path::PathBuf;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

pub async fn serve(context: RequestContext, body: Bytes) -> io::Result<Bytes> {
    serve_wasmtime(context.script()?, &context, body)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

async fn serve_wasmtime(
    file: &PathBuf,
    context: &RequestContext,
    body: Bytes,
) -> Result<Bytes, wasmtime::Error> {
    let mut configuration = Config::new();

    if let Some(path) = context.wasm_cache() {
        configuration.cache_config_load(path)?;
    }

    configuration
        .cache_config_load("/tmp/tortuga")?
        .async_support(true)
        .consume_fuel(true)
        .parallel_compilation(true);

    let engine = Engine::new(&configuration)?;
    let mut linker = Linker::new(&engine);
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

    let mut store = Store::new(&engine, wasi);

    store.set_fuel(1_000_000)?;
    store.fuel_async_yield_interval(Some(1_000))?;

    let module = Module::from_file(&engine, file)?;
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
