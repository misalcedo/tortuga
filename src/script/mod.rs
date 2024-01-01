use crate::context::RequestContext;
use bytes::Bytes;
use std::io;

mod process;
pub mod wasm;

pub use process::Process;

pub trait Script {
    async fn invoke(&self, context: RequestContext, body: Bytes) -> io::Result<Bytes>;
}
