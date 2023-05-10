use crate::request::Request;

use crate::{Message, Response, Wire};
pub use basic::Basic;
pub use content::ContentLength;

mod basic;
mod content;

pub trait Encoding<Error> {
    fn encode<Body, Destination>(
        &mut self,
        message: Message<Request, Body>,
        destination: Destination,
    ) where
        Self: Serialize<Body, Error>,
        Body: ContentLength,
        Destination: Wire;

    fn decode<Body, Source>(&mut self, source: Source) -> Message<Response, Body>
    where
        Self: Deserialize<Body, Error>,
        Body: ContentLength,
        Source: Wire;
}

pub trait Serializable<Error, In, Out = In>:
    Serialize<In, Error> + Deserialize<Out, Error>
where
    In: ?Sized,
{
}

pub trait Serialize<In, Error>
where
    In: ?Sized,
{
    fn serialize<Destination>(
        &mut self,
        input: &In,
        destination: Destination,
    ) -> Result<usize, Error>
    where
        Destination: Wire;
}

pub trait Deserialize<Out, Error> {
    fn deserialize<Source>(&mut self, source: Source) -> Result<Out, Error>
    where
        Source: Wire;
}