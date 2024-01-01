use crate::context::RequestContext;
use bytes::Bytes;
use std::future::Future;
use std::io;

mod process;
mod wasm;

pub use process::Process;
pub use wasm::Wasm;

pub trait Script {
    fn invoke(
        &self,
        context: RequestContext,
        body: Bytes,
    ) -> impl Future<Output = io::Result<Bytes>> + Send;
}
