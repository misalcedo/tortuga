use crate::context::RequestContext;
use bytes::Bytes;
use std::io;

mod process;
mod wasm;

pub use process::Process;
pub use wasm::Wasm;

pub trait Script {
    async fn invoke(&self, context: RequestContext, body: Bytes) -> io::Result<Bytes>;
}
