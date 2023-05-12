use crate::asynchronous::{Body, Wire};
use crate::{encoding, size, Message};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::io;

pub type EncoderResult<Value, Error> = Result<Value, EncoderError<Error>>;

pub enum EncoderError<Error> {
    Encoder(Error),
    Wire(io::Error),
}

impl<Error> EncoderError<Error> {
    pub fn new(error: Error) -> Self {
        EncoderError::Encoder(error)
    }
}

impl<Error> From<io::Error> for EncoderError<Error> {
    fn from(value: io::Error) -> Self {
        EncoderError::Wire(value)
    }
}

#[async_trait]
pub trait Encoder {
    type Error;

    async fn encode_message<Head, Content, Destination>(
        &self,
        message: Message<Head, Content>,
        destination: &mut Destination,
    ) -> EncoderResult<usize, Self::Error>
    where
        Head: Serialize + Send + Sync,
        Content: Body,
        Destination: Wire;

    async fn decode_message<'a, Head, Content, Source>(
        &'a self,
        source: &mut Source,
    ) -> EncoderResult<Message<Head, Content>, Self::Error>
    where
        Head: Deserialize<'a> + Send + Sync,
        Content: Body,
        Source: Wire;
}

#[async_trait]
impl Encoder for encoding::Binary {
    type Error = encoding::Error;

    async fn encode_message<Head, Content, Destination>(
        &self,
        message: Message<Head, Content>,
        destination: &mut Destination,
    ) -> EncoderResult<usize, Self::Error>
    where
        Head: Serialize + Send + Sync,
        Content: Body,
        Destination: Wire,
    {
        let mut bytes = 0;

        let buffer = self.serialize(message.head()).map_err(EncoderError::new)?;

        destination.write_all(buffer.as_slice()).await?;
        bytes += buffer.len();

        let content = message.into_content();
        let buffer = self
            .serialize(&content.size_hint().await)
            .map_err(EncoderError::new)?;

        destination.write_all(buffer.as_slice()).await?;
        bytes += buffer.len();
        bytes += content.write_to(destination).await?;

        Ok(bytes)
    }

    async fn decode_message<'a, Head, Content, Source>(
        &'a self,
        source: &mut Source,
    ) -> EncoderResult<Message<Head, Content>, Self::Error>
    where
        Head: Deserialize<'a> + Send + Sync,
        Content: Body,
        Source: Wire,
    {
        let size = self
            .serialized_size(&size::Hint::default())
            .map_err(EncoderError::new)?;
        let mut buffer = vec![0u8; size];

        source.read_exact(buffer.as_mut_slice()).await?;

        let hint: size::Hint = self
            .deserialize(buffer.as_slice())
            .map_err(EncoderError::new)?;

        todo!()
    }
}
