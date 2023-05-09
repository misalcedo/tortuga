use crate::Encoding;

#[async_trait]
pub trait Wire<Encoding: Encoding> {
    type Error;

    fn blocking_read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;

    fn blocking_write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>;

    async fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>;
}
