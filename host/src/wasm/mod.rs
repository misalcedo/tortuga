use async_trait::async_trait;
pub use connection::Connection;
pub use data::Data;
pub use identifier::Identifier;
use std::fmt::{Debug, Display};
use tortuga_guest::wire::{Destination, Source};

mod connection;
mod data;
mod header;
mod identifier;
pub mod wasmtime;

pub trait Factory: Clone + Send + Sync {
    type Stream: Stream;

    fn create(&mut self) -> Self::Stream;
}

pub trait Ticker: Send + Sync {
    fn tick(&mut self);
}

pub trait Host: Send {
    type Guest: Guest;
    type Error: Debug + Display;
    type Ticker: Ticker;

    fn welcome<Code>(&mut self, code: Code) -> Result<Identifier, Self::Error>
    where
        Code: AsRef<[u8]>;

    fn guest(&self, identifier: &Identifier) -> Option<Self::Guest>;

    fn ticker(&self) -> Self::Ticker;
}

#[async_trait]
pub trait Guest: Send + Sync {
    type Stream: Stream;
    type Error: Debug + Display;

    async fn invoke(&self, stream: Self::Stream) -> Result<i32, Self::Error>;
}

#[async_trait]
pub trait Stream: Source + Destination + Send {
    type Error: Debug + Display + Send;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
    async fn write(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}
