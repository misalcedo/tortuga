use crate::wasm;
use async_trait::async_trait;
pub use router::Router;
pub use wasm::Identifier;

pub mod acceptor;
mod router;
mod tokio;

#[async_trait]
pub trait Provider: Send {
    type Stream: wasm::Stream;

    fn try_next(&mut self) -> Option<Self::Stream>;

    async fn next(&mut self) -> Self::Stream;
}

#[async_trait]
pub trait Acceptor: Send {
    type Stream: wasm::Stream;

    fn try_accept(&mut self) -> Option<Message<Self::Stream>>;

    async fn accept(&mut self) -> Message<Self::Stream>;
}

#[derive(Clone, Debug)]
pub struct Message<Stream> {
    to: Identifier,
    stream: Stream,
}

impl<Stream> Message<Stream> {
    pub fn new(to: Identifier, stream: Stream) -> Self {
        Message { to, stream }
    }

    pub fn to(&self) -> Identifier {
        self.to
    }

    pub fn into_inner(self) -> Stream {
        self.stream
    }
}
