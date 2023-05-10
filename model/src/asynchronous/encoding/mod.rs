use async_trait::async_trait;

pub use crate::asynchronous::content::ContentLength;
pub use basic::Basic;
pub use error::EncodingResult;

use crate::asynchronous::Wire;
use crate::Message;

mod basic;
mod error;

#[async_trait]
pub trait Encoding<Error> {
    async fn encode<Body, Head, Destination>(
        &mut self,
        message: Message<Head, Body>,
        destination: Destination,
    ) -> EncodingResult<usize, Error>
    where
        Self: Serialize<Body, Error>,
        Self: Serialize<Head, Error>,
        Body: ContentLength,
        Destination: Wire,
        Head: Send + Sync;

    async fn decode<Body, Head, Source>(&mut self, source: Source) -> Message<Head, Body>
    where
        Self: Deserialize<Body, Error>,
        Self: Deserialize<Head, Error>,
        Body: ContentLength,
        Source: Wire,
        Head: Send + Sync;
}

pub trait Serializable<Error, In, Out = In>:
    Serialize<In, Error> + Deserialize<Out, Error>
where
    In: ?Sized,
{
}

#[async_trait]
pub trait Serialize<In, Error>
where
    In: ?Sized,
{
    async fn serialize<Destination>(
        &mut self,
        input: &In,
        destination: Destination,
    ) -> Result<usize, Error>
    where
        Destination: Wire;
}

#[async_trait]
pub trait Deserialize<Out, Error> {
    async fn deserialize<Source>(&mut self, source: Source) -> Result<Out, Error>
    where
        Source: Wire;
}
