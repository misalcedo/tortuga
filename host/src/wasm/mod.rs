use std::io;

use async_trait::async_trait;
pub use connection::Connection;
pub use data::Data;
use tortuga_guest::wire::{Destination, ReadableMessage, Source, WritableMessage};

mod connection;
mod data;
pub mod wasmtime;

pub trait Factory<Stream>: Clone + Send + Sync {
    fn create(&mut self) -> Stream;
}

pub trait Host<Stream>: Send {
    type Guest: Guest<Stream>;
    type Error;

    fn welcome<Code>(&mut self, code: Code) -> Result<Self::Guest, Self::Error>
    where
        Code: AsRef<[u8]>;
}

#[async_trait]
pub trait Guest<Stream>: Send {
    type Error;

    async fn invoke(&self, stream: Stream) -> Result<i32, Self::Error>;
}

#[async_trait]
pub trait Stream: Source + Destination + Send {
    type Error;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
    async fn write(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}
