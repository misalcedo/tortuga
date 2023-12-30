use crate::context::RequestContext;
use bytes::Bytes;
use http::Response;
use http_body_util::Full;
use std::io;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

pub async fn serve(context: RequestContext, body: Bytes) -> io::Result<Response<Full<Bytes>>> {
    serve_wasmtime(context, body)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

async fn serve_wasmtime(
    context: RequestContext,
    body: Bytes,
) -> Result<Response<Full<Bytes>>, wasmtime::Error> {
    let mut configuration = Config::new();

    configuration
        .async_support(true)
        .consume_fuel(true)
        .parallel_compilation(true);

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let stdout = WritePipe::new_in_memory();
    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(ReadPipe::from(body.as_ref())))
        .stdout(Box::new(stdout.clone()))
        .inherit_stderr()
        .build();
    let mut store = Store::new(&engine, wasi);

    store.set_fuel(1_000_000)?;
    store.fuel_async_yield_interval(Some(1_000))?;

    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasi.wasm")?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(Response::new(Full::default()))
}
