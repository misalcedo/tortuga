use crate::asynchronous::encoding::{ContentLength, Deserialize, EncodingResult, Serialize};
use crate::{Encoding, Message, Request, Response, Wire};
use async_trait::async_trait;

pub enum Error {}
pub struct Basic {}

#[async_trait]
impl Encoding<Error> for Basic {
    async fn encode<Body, Destination>(
        &mut self,
        message: Message<Request, Body>,
        destination: Destination,
    ) -> EncodingResult<usize, Error>
    where
        Self: Serialize<Body, Error>,
        Body: ContentLength,
        Destination: Wire,
    {
        todo!()
    }

    async fn decode<Body, Source>(&mut self, source: Source) -> Message<Response, Body>
    where
        Self: Deserialize<Body, Error>,
        Body: ContentLength,
        Source: Wire,
    {
        todo!()
    }
}
