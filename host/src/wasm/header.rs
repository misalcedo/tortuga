use crate::wasm;
use async_trait::async_trait;
use std::io::{Read, Write};
use tortuga_guest::Header;

#[async_trait]
impl<Stream> wasm::Stream for Header<Stream>
where
    Stream: wasm::Stream<Error = std::io::Error> + Read + Write,
{
    type Error = std::io::Error;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        let header = self.buffer();

        if header.position() == header.get_ref().len() as u64 {
            wasm::Stream::read(self.inner(), buffer).await
        } else {
            Read::read(header, buffer)
        }
    }

    async fn write(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        wasm::Stream::write(self.inner(), buffer).await
    }
}
