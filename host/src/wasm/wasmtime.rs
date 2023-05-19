use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, IoSlice, IoSliceMut, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::file::{FdFlags, FileCaps, FileType, RiFlags, RoFlags, SdFlags, SiFlags};
use wasi_common::{ErrorExt, WasiFile};

use wasmtime::{Caller, Config, Engine, InstancePre, Linker, Module, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

#[derive(Debug)]
pub enum Error {
    Wasm(wasmtime::Error),
    WasiArgs(wasi_common::StringArrayError),
    MissingExport,
    MissingMemory,
    FailedConnection,
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

pub struct Host<Network> {
    engine: Engine,
    network: Network,
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

#[async_trait]
pub trait Network: Clone + Send + Sync {
    async fn add(&mut self, origin: &str, guest: Guest) -> Option<Guest>;

    async fn connect(&mut self, origin: &str) -> Option<Box<dyn WasiFile>>;
}

impl<N> Host<N>
where
    N: Network,
{
    pub fn new(network: N, injection_count: u64, fuel_to_inject: u64) -> Result<Self, Error> {
        let mut configuration = Config::new();

        configuration.async_support(true);
        configuration.consume_fuel(true);
        configuration.wasm_memory64(true);

        let engine = Engine::new(&configuration)?;

        Ok(Host {
            engine,
            network,
            injection_count,
            fuel_to_inject,
        })
    }
}

impl<N> Host<N>
where
    N: Network + 'static,
{
    fn welcome<Code>(&self, code: Code) -> Result<Guest, Error>
    where
        Code: AsRef<[u8]>,
    {
        let network = self.network.clone();
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, Data::context_mut)?;

        linker.func_wrap2_async(
            "guest",
            "connect",
            move |mut caller: Caller<'_, Data>, pointer: u32, length: u64| {
                let mut network = network.clone();

                Box::new(async move {
                    let memory = caller
                        .get_export("memory")
                        .ok_or(Error::MissingExport)?
                        .into_memory()
                        .ok_or(Error::MissingMemory)?;
                    let (data, store): (&mut [u8], &mut Data) =
                        memory.data_and_store_mut(&mut caller);
                    let view = &data[..(pointer as usize + length as usize)][pointer as usize..];
                    let origin = String::from_utf8_lossy(view);

                    let connection = network
                        .connect(origin.as_ref())
                        .await
                        .ok_or(Error::FailedConnection)?;
                    let capabilities = FileCaps::FDSTAT_SET_FLAGS
                        | FileCaps::FILESTAT_GET
                        | FileCaps::READ
                        | FileCaps::WRITE
                        | FileCaps::POLL_READWRITE;
                    let file_descriptor =
                        store.context_mut().push_file(connection, capabilities)?;

                    Ok(file_descriptor)
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

#[async_trait]
impl Network for () {
    async fn add(&mut self, origin: &str, guest: Guest) -> Option<Guest> {
        None
    }

    async fn connect(&mut self, _: &str) -> Option<Box<dyn WasiFile>> {
        None
    }
}

#[async_trait]
impl<R, W> Network for BidirectionalPipe<R, W>
where
    R: Read + Send + Sync + 'static,
    W: Write + Send + Sync + 'static,
{
    async fn add(&mut self, origin: &str, guest: Guest) -> Option<Guest> {
        None
    }

    async fn connect(&mut self, origin: &str) -> Option<Box<dyn WasiFile>> {
        Some(Box::new(self.clone()))
    }
}

pub struct BidirectionalPipe<R, W> {
    read: Arc<RwLock<R>>,
    write: Arc<RwLock<W>>,
    state: Arc<RwLock<PipeState>>,
}

pub struct PipeState {
    blocking: bool,
    readable: bool,
    writable: bool,
}

impl Default for PipeState {
    fn default() -> Self {
        PipeState {
            blocking: true,
            readable: true,
            writable: true,
        }
    }
}

impl<R, W> Clone for BidirectionalPipe<R, W> {
    fn clone(&self) -> Self {
        BidirectionalPipe {
            read: Arc::clone(&self.read),
            write: Arc::clone(&self.write),
            state: Arc::clone(&self.state),
        }
    }
}

impl Default for BidirectionalPipe<Cursor<Vec<u8>>, Cursor<Vec<u8>>> {
    fn default() -> Self {
        BidirectionalPipe::new(Cursor::new(Vec::new()), Cursor::new(Vec::new()))
    }
}

impl<R, W> BidirectionalPipe<R, W>
where
    R: Read + Send + Sync + 'static,
    W: Write + Send + Sync + 'static,
{
    fn new(read: R, write: W) -> Self {
        BidirectionalPipe {
            read: Arc::new(RwLock::new(read)),
            write: Arc::new(RwLock::new(write)),
            state: Arc::new(RwLock::new(PipeState::default())),
        }
    }
}

impl<R, W> BidirectionalPipe<R, W> {
    fn into_write(self) -> Option<W> {
        let lock = Arc::try_unwrap(self.write).ok()?;
        match lock.into_inner() {
            Ok(w) => Some(w),
            Err(e) => Some(e.into_inner()),
        }
    }
}

#[async_trait]
impl<R, W> WasiFile for BidirectionalPipe<R, W>
where
    R: Read + Send + Sync + 'static,
    W: Write + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn get_filetype(&self) -> Result<FileType, wasi_common::Error> {
        Ok(FileType::SocketStream)
    }

    async fn sock_recv<'a>(
        &self,
        ri_data: &mut [IoSliceMut<'a>],
        ri_flags: RiFlags,
    ) -> Result<(u64, RoFlags), wasi_common::Error> {
        if (ri_flags & !(RiFlags::RECV_PEEK | RiFlags::RECV_WAITALL)) != RiFlags::empty() {
            Err(wasi_common::Error::not_supported())
        } else if ri_flags.contains(RiFlags::RECV_PEEK) {
            Err(wasi_common::Error::not_supported())
        } else if ri_flags.contains(RiFlags::RECV_WAITALL) {
            let n: usize = ri_data.iter().map(|buf| buf.len()).sum();
            let mut buffers = &mut ri_data[..];

            while !buffers.is_empty() {
                if buffers[0].is_empty() {
                    buffers = &mut buffers[1..];
                    continue;
                }

                let mut guard = RwLock::write(&self.read).unwrap();
                match guard.read_vectored(buffers) {
                    Ok(0) => return Err(wasi_common::Error::io()),
                    Ok(read) => IoSliceMut::advance_slices(&mut buffers, read as usize), // https://github.com/rust-lang/rust/issues/62726
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => (),
                    Err(e) => return Err(e.into()),
                }
            }

            Ok((n as u64, RoFlags::empty()))
        } else {
            Ok((self.read_vectored(ri_data).await?, RoFlags::empty()))
        }
    }

    async fn sock_send<'a>(
        &self,
        si_data: &[IoSlice<'a>],
        si_flags: SiFlags,
    ) -> Result<u64, wasi_common::Error> {
        if si_flags != SiFlags::empty() {
            return Err(wasi_common::Error::not_supported());
        }

        self.write_vectored(si_data).await
    }

    async fn sock_shutdown(&self, how: SdFlags) -> Result<(), wasi_common::Error> {
        let mut guard = RwLock::write(&self.state).unwrap();

        if how == SdFlags::RD | SdFlags::WR {
            guard.readable = false;
            guard.writable = false;

            Ok(())
        } else if how == SdFlags::RD {
            guard.readable = false;
            Ok(())
        } else if how == SdFlags::WR {
            guard.writable = false;
            Ok(())
        } else {
            Err(wasi_common::Error::invalid_argument())
        }
    }

    async fn get_fdflags(&self) -> Result<FdFlags, wasi_common::Error> {
        let guard = RwLock::read(&self.state).unwrap();

        if guard.blocking {
            Ok(FdFlags::empty())
        } else {
            Ok(FdFlags::NONBLOCK)
        }
    }

    async fn set_fdflags(&mut self, flags: FdFlags) -> Result<(), wasi_common::Error> {
        let mut guard = RwLock::write(&self.state).unwrap();

        if flags == FdFlags::NONBLOCK {
            guard.blocking = false;
            Ok(())
        } else if flags == FdFlags::empty() {
            guard.blocking = true;
            Ok(())
        } else {
            Err(wasi_common::Error::invalid_argument().context("only NONBLOCK is supported"))
        }
    }

    async fn read_vectored<'a>(
        &self,
        bufs: &mut [IoSliceMut<'a>],
    ) -> Result<u64, wasi_common::Error> {
        let mut guard = RwLock::write(&self.read).unwrap();
        Ok(guard.read_vectored(bufs)? as u64)
    }

    async fn write_vectored<'a>(&self, bufs: &[IoSlice<'a>]) -> Result<u64, wasi_common::Error> {
        let mut guard = RwLock::write(&self.write).unwrap();
        Ok(guard.write_vectored(bufs)? as u64)
    }

    async fn peek(&self, _buf: &mut [u8]) -> Result<u64, wasi_common::Error> {
        Err(wasi_common::Error::not_supported())
    }

    fn num_ready_bytes(&self) -> Result<u64, wasi_common::Error> {
        Err(wasi_common::Error::not_supported())
    }

    async fn readable(&self) -> Result<(), wasi_common::Error> {
        let guard = RwLock::read(&self.state).unwrap();

        if guard.readable {
            Ok(())
        } else {
            Err(wasi_common::Error::io())
        }
    }

    async fn writable(&self) -> Result<(), wasi_common::Error> {
        let guard = RwLock::read(&self.state).unwrap();

        if guard.writable {
            Ok(())
        } else {
            Err(wasi_common::Error::io())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn basic() {
        let code = include_bytes!(env!("CARGO_BIN_FILE_WASI"));
        let stream = BidirectionalPipe::new(Cursor::new(b"foobar"), Cursor::new(Vec::new()));
        let host = Host::new(stream.clone(), 100, 1000).unwrap();
        let guest = host.welcome(code).unwrap();

        let input: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let output: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let configuration = Configuration::new(input, output);

        let response = guest.invoke(configuration).await.unwrap();

        drop(host);
        drop(guest);

        assert_eq!(response.get_ref().as_slice(), b"Hello, world!\n");
        assert_eq!(stream.into_write().unwrap().get_ref().as_slice(), b"!");
    }
}
