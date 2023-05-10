use crate::encoding::{ContentLength, Deserialize, EncodingResult, Serialize};
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

// impl<W> Encode<io::Error, bool> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &bool) -> io::Result<usize> {
//         let buffer = if value {
//             1u8.to_le_bytes()
//         } else {
//             0u8.to_le_bytes()
//         };
//
//         self.write_all(&buffer)?;
//
//         Ok(buffer.len())
//     }
//
//     fn decode(&mut self) -> Result<bool, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, u8> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &u8) -> io::Result<usize> {
//         let buffer = value.to_le_bytes();
//
//         self.write_all(&buffer)?;
//
//         Ok(buffer.len())
//     }
//
//     fn decode(&mut self) -> Result<u8, io::Error> {
//         todo!()
//     }
// }
//
// impl<W> Encode<io::Error, u16> for W
// where
//     W: Wire,
// {
//     fn encode(&mut self, value: &u16) -> io::Result<usize> {
//         let buffer = value.to_le_bytes();
//
//         self.write_all(&buffer)?;
//
//         Ok(buffer.len())
//     }
//
//     fn decode(&mut self) -> Result<u16, io::Error> {
//         todo!()
//     }
// }
//
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
