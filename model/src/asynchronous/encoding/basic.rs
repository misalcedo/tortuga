use crate::asynchronous::encoding::{
    error::EncodingError, Deserialize, EncodingResult, Serializable, Serialize,
};
use crate::asynchronous::{self, wire, Encoding, Wire};
use crate::{frame, Frame, Message, Method, Request, Response, Status, Uri};
use async_trait::async_trait;
use std::string::FromUtf8Error;

pub enum Error {
    InvalidFrameKind(u8),
    InvalidMethod(u8),
    InvalidStatus(u16),
    UnexpectedFrameKind {
        expected: frame::Kind,
        actual: frame::Kind,
    },
    UTF8(FromUtf8Error),
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::UTF8(value)
    }
}

pub struct Basic {}

#[async_trait]
impl Encoding<Error> for Basic {
    async fn encode<Body, Head, Destination>(
        &mut self,
        mut message: Message<Head, Body>,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Body: asynchronous::Body,
        Destination: Wire,
        Head: Serializable<Self, Error> + Send + Sync,
    {
        let content_length = message.body().length().await;
        let mut counter = wire::Counter::default();

        message.head().serialize(self, &mut counter).await?;
        self.serialize(&content_length, &mut counter).await?;

        let header = frame::Header::from(counter.bytes_read());

        let mut bytes = self.serialize(&header, destination).await?;

        bytes += self.serialize(message.head(), destination).await?;
        bytes += self.serialize(&content_length, destination).await?;

        todo!("Serialize the body");

        Ok(bytes)
    }

    async fn decode<Body, Head, Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<Message<Head, Body>, Error>
    where
        Body: asynchronous::Body,
        Source: Wire,
        Head: Serializable<Self, Error> + Send + Sync,
    {
        let header: frame::Header = self.deserialize(source).await?;
        let head: Head = self.deserialize(source).await?;
        let length: usize = self.deserialize(source).await?;
        let body: Body = self.deserialize(source).await?;

        todo!()
    }
}

#[async_trait]
impl Serialize<bool> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &bool,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let value = if *input { 1u8 } else { 0u8 };

        self.serialize(&value, destination).await
    }
}

#[async_trait]
impl Deserialize<bool> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<bool, Error>
    where
        Source: Wire,
    {
        let output: u8 = self.deserialize(source).await?;

        Ok(output == 1u8)
    }
}

impl Serializable<bool> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<u8> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &u8,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let buffer = input.to_le_bytes();

        destination.write_all(&buffer).await?;

        Ok(buffer.len())
    }
}

#[async_trait]
impl Deserialize<u8> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<u8, Error>
    where
        Source: Wire,
    {
        let mut buffer = 0u8.to_le_bytes();

        source.read_exact(&mut buffer).await?;

        Ok(u8::from_le_bytes(buffer))
    }
}

impl Serializable<u8> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<u16> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &u16,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let mut buffer = input.to_le_bytes();

        destination.write_all(&mut buffer).await?;

        Ok(buffer.len())
    }
}

#[async_trait]
impl Deserialize<u16> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<u16, Error>
    where
        Source: Wire,
    {
        let mut buffer = 0u16.to_le_bytes();

        source.read_exact(&mut buffer).await?;

        Ok(u16::from_le_bytes(buffer))
    }
}

impl Serializable<u16> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<u64> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &u64,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let mut buffer = input.to_le_bytes();

        destination.write_all(&mut buffer).await?;

        Ok(buffer.len())
    }
}

#[async_trait]
impl Deserialize<u64> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<u64, Error>
    where
        Source: Wire,
    {
        let mut buffer = 0u64.to_le_bytes();

        source.read_exact(&mut buffer).await?;

        Ok(u64::from_le_bytes(buffer))
    }
}

impl Serializable<u64> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<usize> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &usize,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(&(*input as u64), destination).await
    }
}

#[async_trait]
impl Deserialize<usize> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<usize, Error>
    where
        Source: Wire,
    {
        let output: u64 = self.deserialize(source).await?;

        Ok(output as usize)
    }
}

impl Serializable<usize> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<Option<usize>> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &Option<usize>,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let mut bytes = self.serialize(&input.is_some(), destination).await?;

        bytes += self
            .serialize(&input.unwrap_or_default(), destination)
            .await?;

        Ok(bytes)
    }
}

#[async_trait]
impl Deserialize<Option<usize>> for Basic {
    type Error = Error;

    async fn deserialize<Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<Option<usize>, Error>
    where
        Source: Wire,
    {
        let some: bool = self.deserialize(source).await?;
        let output: usize = self.deserialize(source).await?;

        if some {
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}

impl Serializable<Option<usize>> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<[u8]> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &[u8],
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let bytes = self.serialize(&input.len(), destination).await?;

        destination.write_all(input).await?;

        Ok(bytes + input.len())
    }
}

#[async_trait]
impl Deserialize<Vec<u8>> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<Vec<u8>, Error>
    where
        Source: Wire,
    {
        let length: usize = self.deserialize(source).await?;
        let mut buffer = vec![0u8; length];

        source.read_exact(buffer.as_mut_slice()).await?;

        Ok(buffer)
    }
}

impl Serializable<[u8], Vec<u8>> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<str> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &str,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(input.as_bytes(), destination).await
    }
}

#[async_trait]
impl Deserialize<String> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<String, Error>
    where
        Source: Wire,
    {
        let bytes: Vec<u8> = self.deserialize(source).await?;

        String::from_utf8(bytes)
            .map_err(Error::UTF8)
            .map_err(EncodingError::Encoding)
    }
}

impl Serializable<str, String> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<Uri> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &Uri,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(input.as_str(), destination).await
    }
}

#[async_trait]
impl Deserialize<Uri> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<Uri, Error>
    where
        Source: Wire,
    {
        let uri: String = self.deserialize(source).await?;
        Ok(Uri::from(uri))
    }
}

impl Serializable<Uri> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<frame::Kind> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &frame::Kind,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(&u8::from(*input), destination).await
    }
}

#[async_trait]
impl Deserialize<frame::Kind> for Basic {
    type Error = Error;

    async fn deserialize<Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<frame::Kind, Error>
    where
        Source: Wire,
    {
        let kind: u8 = self.deserialize(source).await?;
        frame::Kind::try_from(kind)
            .map_err(|_| Error::InvalidFrameKind(kind))
            .map_err(EncodingError::Encoding)
    }
}

impl Serializable<frame::Kind> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<frame::Header> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &frame::Header,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let mut bytes = self.serialize(&input.kind(), destination).await?;

        bytes += self.serialize(&input.len(), destination).await?;

        Ok(bytes)
    }
}

#[async_trait]
impl Deserialize<frame::Header> for Basic {
    type Error = Error;

    async fn deserialize<Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<frame::Header, Error>
    where
        Source: Wire,
    {
        let kind: frame::Kind = self.deserialize(source).await?;
        let length: usize = self.deserialize(source).await?;

        match kind {
            frame::Kind::Data => Err(EncodingError::Encoding(Error::UnexpectedFrameKind {
                expected: frame::Kind::Header,
                actual: frame::Kind::Data,
            })),
            frame::Kind::Header => Ok(frame::Header::from(length)),
        }
    }
}

impl Serializable<frame::Header> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<frame::Data> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &frame::Data,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let mut bytes = self.serialize(&input.kind(), destination).await?;

        bytes += self.serialize(&input.len(), destination).await?;

        Ok(bytes)
    }
}

#[async_trait]
impl Deserialize<frame::Data> for Basic {
    type Error = Error;

    async fn deserialize<Source>(
        &mut self,
        source: &mut Source,
    ) -> EncodingResult<frame::Data, Error>
    where
        Source: Wire,
    {
        let kind: frame::Kind = self.deserialize(source).await?;
        let length: usize = self.deserialize(source).await?;

        match kind {
            frame::Kind::Data => Ok(frame::Data::from(length)),
            frame::Kind::Header => Err(EncodingError::Encoding(Error::UnexpectedFrameKind {
                expected: frame::Kind::Data,
                actual: frame::Kind::Header,
            })),
        }
    }
}

impl Serializable<frame::Data> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<Method> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &Method,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(&u8::from(*input), destination).await
    }
}

#[async_trait]
impl Deserialize<Method> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<Method, Error>
    where
        Source: Wire,
    {
        let method: u8 = self.deserialize(source).await?;
        Method::try_from(method)
            .map_err(|_| Error::InvalidMethod(method))
            .map_err(EncodingError::Encoding)
    }
}

impl Serializable<Method> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<Status> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &Status,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(&u16::from(*input), destination).await
    }
}

#[async_trait]
impl Deserialize<Status> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<Status, Error>
    where
        Source: Wire,
    {
        let status: u16 = self.deserialize(source).await?;
        Status::try_from(status)
            .map_err(|_| Error::InvalidStatus(status))
            .map_err(EncodingError::Encoding)
    }
}

impl Serializable<Status> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<Request> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &Request,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        let mut bytes = self.serialize(&input.method(), destination).await?;

        bytes += self.serialize(input.uri(), destination).await?;

        Ok(bytes)
    }
}

#[async_trait]
impl Deserialize<Request> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<Request, Error>
    where
        Source: Wire,
    {
        let method: Method = self.deserialize(source).await?;
        let uri: Uri = self.deserialize(source).await?;

        Ok(Request::new(method, uri))
    }
}

impl Serializable<Request> for Basic {
    type Error = Error;
}

#[async_trait]
impl Serialize<Response> for Basic {
    type Error = Error;

    async fn serialize<Destination>(
        &mut self,
        input: &Response,
        destination: &mut Destination,
    ) -> EncodingResult<usize, Error>
    where
        Destination: Wire,
    {
        self.serialize(&input.status(), destination).await
    }
}

#[async_trait]
impl Deserialize<Response> for Basic {
    type Error = Error;

    async fn deserialize<Source>(&mut self, source: &mut Source) -> EncodingResult<Response, Error>
    where
        Source: Wire,
    {
        let status: u16 = self.deserialize(source).await?;
        Ok(Response::new(status))
    }
}

impl Serializable<Response> for Basic {
    type Error = Error;
}
