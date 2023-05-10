use crate::asynchronous::encoding::{Deserialize, EncodingResult, Serializable, Serialize};
use crate::asynchronous::{ContentLength, Encoding, Wire};
use crate::Message;
use async_trait::async_trait;
use std::io;

pub enum Error {}
pub struct Basic {}

#[async_trait]
impl Encoding<Error> for Basic {
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
        Head: Send + Sync,
    {
        todo!()
    }

    async fn decode<Body, Head, Source>(&mut self, source: Source) -> Message<Head, Body>
    where
        Self: Deserialize<Body, Error>,
        Self: Deserialize<Head, Error>,
        Body: ContentLength,
        Source: Wire,
        Head: Send + Sync,
    {
        todo!()
    }
}

#[async_trait]
impl Serialize<bool, io::Error> for Basic {
    async fn serialize<Destination>(
        &mut self,
        input: &bool,
        destination: Destination,
    ) -> Result<usize, io::Error>
    where
        Destination: Wire,
    {
        let value = if *input { 1u8 } else { 0u8 };

        self.serialize(&value, destination).await
    }
}

#[async_trait]
impl Deserialize<bool, io::Error> for Basic {
    async fn deserialize<Source>(&mut self, source: Source) -> Result<bool, io::Error>
    where
        Source: Wire,
    {
        let output: u8 = self.deserialize(source).await?;

        Ok(output == 1u8)
    }
}

impl Serializable<io::Error, bool> for Basic {}

#[async_trait]
impl Serialize<u8, io::Error> for Basic {
    async fn serialize<Destination>(
        &mut self,
        input: &u8,
        mut destination: Destination,
    ) -> Result<usize, io::Error>
    where
        Destination: Wire,
    {
        let buffer = input.to_le_bytes();

        destination.write_all(&buffer).await?;

        Ok(buffer.len())
    }
}

#[async_trait]
impl Deserialize<u8, io::Error> for Basic {
    async fn deserialize<Source>(&mut self, mut source: Source) -> Result<u8, io::Error>
    where
        Source: Wire,
    {
        let mut buffer = 0u8.to_le_bytes();

        source.read_exact(&mut buffer).await?;

        Ok(u8::from_le_bytes(buffer))
    }
}

impl Serializable<io::Error, u8> for Basic {}

#[async_trait]
impl Serialize<u16, io::Error> for Basic {
    async fn serialize<Destination>(
        &mut self,
        input: &u16,
        mut destination: Destination,
    ) -> Result<usize, io::Error>
    where
        Destination: Wire,
    {
        let mut buffer = input.to_le_bytes();

        destination.write_all(&mut buffer).await?;

        Ok(buffer.len())
    }
}

#[async_trait]
impl Deserialize<u16, io::Error> for Basic {
    async fn deserialize<Source>(&mut self, mut source: Source) -> Result<u16, io::Error>
    where
        Source: Wire,
    {
        let mut buffer = 0u16.to_le_bytes();

        source.read_exact(&mut buffer).await?;

        Ok(u16::from_le_bytes(buffer))
    }
}

impl Serializable<io::Error, u16> for Basic {}

#[async_trait]
impl Serialize<u64, io::Error> for Basic {
    async fn serialize<Destination>(
        &mut self,
        input: &u64,
        mut destination: Destination,
    ) -> Result<usize, io::Error>
    where
        Destination: Wire,
    {
        let mut buffer = input.to_le_bytes();

        destination.write_all(&mut buffer).await?;

        Ok(buffer.len())
    }
}

#[async_trait]
impl Deserialize<u64, io::Error> for Basic {
    async fn deserialize<Source>(&mut self, mut source: Source) -> Result<u64, io::Error>
    where
        Source: Wire,
    {
        let mut buffer = 0u64.to_le_bytes();

        source.read_exact(&mut buffer).await?;

        Ok(u64::from_le_bytes(buffer))
    }
}

impl Serializable<io::Error, u64> for Basic {}

#[async_trait]
impl Serialize<usize, io::Error> for Basic {
    async fn serialize<Destination>(
        &mut self,
        input: &usize,
        destination: Destination,
    ) -> Result<usize, io::Error>
    where
        Destination: Wire,
    {
        self.serialize(&(*input as u64), destination).await
    }
}

#[async_trait]
impl Deserialize<usize, io::Error> for Basic {
    async fn deserialize<Source>(&mut self, source: Source) -> Result<usize, io::Error>
    where
        Source: Wire,
    {
        let output: u64 = self.deserialize(source).await?;

        Ok(output as usize)
    }
}

impl Serializable<io::Error, usize> for Basic {}

// impl<W> Encode<io::Error, u64> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &u64) -> io::Result<usize> {
//         let buffer = value.to_le_bytes();
//
//         self.write_all(&buffer)?;
//
//         Ok(buffer.len())
//     }
//
//     fn decode(&mut self) -> Result<u64, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, usize> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &usize) -> io::Result<usize> {
//         self.encode(&(*value as u64))
//     }
//
//     fn decode(&mut self) -> Result<usize, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, str, String> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &str) -> io::Result<usize> {
//         let mut bytes = self.encode(&value.len())?;
//         let buffer = value.as_bytes();
//
//         self.write_all(buffer)?;
//
//         bytes += buffer.len();
//
//         Ok(bytes)
//     }
//
//     fn decode(&mut self) -> Result<String, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, Uri> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &Uri) -> io::Result<usize> {
//         self.encode(value.as_ref())
//     }
//
//     fn decode(&mut self) -> Result<Uri, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, frame::Kind> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &frame::Kind) -> io::Result<usize> {
//         self.encode(&u8::from(*value))
//     }
//
//     fn decode(&mut self) -> Result<frame::Kind, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, frame::Header> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &frame::Header) -> io::Result<usize> {
//         let mut bytes = 0;
//
//         bytes += self.encode(&value.kind())?;
//         bytes += self.encode(&value.len())?;
//
//         Ok(bytes)
//     }
//
//     fn decode(&mut self) -> Result<frame::Header, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, frame::Data> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &frame::Data) -> io::Result<usize> {
//         let mut bytes = 0;
//
//         bytes += self.encode(&value.kind())?;
//         bytes += self.encode(&value.len())?;
//
//         Ok(bytes)
//     }
//
//     fn decode(&mut self) -> Result<frame::Data, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, Method> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &Method) -> io::Result<usize> {
//         self.encode(&u8::from(*value))
//     }
//
//     fn decode(&mut self) -> Result<Method, io::Error> {
//         todo!()
//     }
// }
