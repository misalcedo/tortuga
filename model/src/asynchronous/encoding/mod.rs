use async_trait::async_trait;

pub use basic::Basic;
pub use error::EncodingResult;

use crate::asynchronous::{self, Wire};
use crate::Message;

mod basic;
mod error;

// TODO: Figure out how I want to ensure that the body supports streaming and can be multiple values.
#[async_trait]
pub trait Encoding<Error> {
    async fn encode<Body, Head, Destination>(
        &mut self,
        message: Message<Head, Body>,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Body: asynchronous::Body,
        Destination: Wire,
        Head: Serializable<Self, Error> + Send + Sync;

    async fn decode<Body, Head, Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<Message<Head, Body>, Error>
    where
        Body: asynchronous::Body,
        Source: Wire,
        Head: Serializable<Self, Error> + Send + Sync;
}

pub trait Serializable<Encoding, Out = Self>
where
    Encoding: Serialize<Self, Error = Self::Error> + Deserialize<Out, Error = Self::Error> + ?Sized,
{
    type Error;
}

#[async_trait]
pub trait Serialize<In>
where
    In: ?Sized,
{
    type Error;

    async fn serialize<Destination>(
        &mut self,
        input: &In,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Self::Error>
    where
        Destination: Wire;
}

#[async_trait]
pub trait Deserialize<Out> {
    type Error;

    async fn deserialize<Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<Out, Self::Error>
    where
        Source: Wire;
}
