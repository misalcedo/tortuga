use std::io::{Cursor, Read, Write};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

use tortuga_model::encoding::Format;
use tortuga_model::{Headers, Message, Request, Response, Status};
use wasmtime::{Caller, Config, Engine, InstancePre, Linker, Module, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

#[derive(Debug)]
pub enum Error {
    Wasm(wasmtime::Error),
    WasiArgs(wasi_common::StringArrayError),
    Encoding(tortuga_model::encoding::Error),
    Internal,
}

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

impl From<tortuga_model::encoding::Error> for Error {
    fn from(value: tortuga_model::encoding::Error) -> Self {
        Error::Encoding(value)
    }
}

pub struct Host<Encoding> {
    engine: Engine,
    encoding: Encoding,
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

impl<Encoding> Host<Encoding>
where
    Encoding: Format<Request> + Format<Response>,
{
    pub fn new(
        encoding: Encoding,
        injection_count: u64,
        fuel_to_inject: u64,
    ) -> Result<Self, Error> {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.consume_fuel(true);
        configuration.wasm_memory64(size_of::<usize>() == size_of::<u64>());

        let engine = Engine::new(&configuration)?;

        Ok(Host {
            engine,
            encoding,
            injection_count,
            fuel_to_inject,
        })
    }
}

impl<Encoding> Host<Encoding>
where
    Encoding: Clone + Format<Request> + Format<Response>,
{
    fn welcome<Code>(&self, code: Code) -> Result<Guest<Encoding>, Error>
    where
        Code: AsRef<[u8]>,
    {
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, Data::context_mut)?;

        linker.func_wrap2_async(
            "stream",
            "start",
            move |mut caller: Caller<'_, Data>, pointer: u32, length: u32| {
                Box::new(async move {
                    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (data, state): (&mut [u8], &mut Data) =
                        memory.data_and_store_mut(&mut caller);
                    let view =
                        &mut data[..(pointer as usize + length as usize)][pointer as usize..];

                    Ok(0)
                })
            },
        )?;

        let module = Module::new(&self.engine, code)?;
        let instance = linker.instantiate_pre(&module)?;

        Ok(Guest {
            instance,
            encoding: self.encoding.clone(),
            injection_count: self.injection_count,
            fuel_to_inject: self.fuel_to_inject,
        })
    }
}

pub struct Guest<Encoding> {
    instance: InstancePre<Data>,
    encoding: Encoding,
    injection_count: u64,
    fuel_to_inject: u64,
}

impl<Encoding> Guest<Encoding>
where
    Encoding: Format<Request> + Format<Response>,
{
    async fn invoke<I, O>(
        &self,
        message: Message<Request, I>,
        output: O,
    ) -> Result<Message<Response, O>, Error>
    where
        I: Read + Send + Sync + 'static,
        O: Write + Send + Sync + 'static,
    {
        let request = Cursor::new(self.encoding.serialize(message.head())?);
        let input = request.chain(message.into_content());
        let output = Arc::new(RwLock::new(output));

        let context = WasiCtxBuilder::new()
            .stdin(Box::new(wasi_common::pipe::ReadPipe::new(input)))
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
            .map_err(|e| Error::Internal)?
            .into_inner()
        {
            Ok(o) => {
                let response = Response::new(Status::Ok, Headers::default());
                Ok(Message::new_response(response, o))
            }
            Err(e) => {
                let response = Response::new(Status::InternalServerError, Headers::default());
                Ok(Message::new_response(response, e.into_inner()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tortuga_model::encoding::Binary;
    use tortuga_model::{Headers, Method};

    #[tokio::test]
    async fn error() {
        let encoding = Binary::default();
        let host = Host::new(encoding, 10, 1000).unwrap();

        let code = include_bytes!(env!("CARGO_BIN_FILE_WASI"));
        let guest = host.welcome(code).unwrap();

        let request = Request::new(Method::Get, "/".into(), Headers::default());
        let message = Message::new_request(request, Cursor::new(Vec::new()));
        let output: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let mut response = guest.invoke(message, output).await.unwrap();

        assert_eq!(response.content().get_ref().as_slice(), b"Hello, world!\n")
    }
}
