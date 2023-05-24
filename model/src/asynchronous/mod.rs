use async_trait::async_trait;
pub use body::Body;
pub use wire::Wire;

mod body;
mod wire;

#[async_trait]
pub trait Read: Send + Sync {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;

    async fn read_exact(&mut self, mut buf: &mut [u8]) -> std::io::Result<()> {
        while !buf.is_empty() {
            match self.read(buf).await {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ))
        } else {
            Ok(())
        }
    }
}

#[async_trait]
pub trait Write: Send + Sync {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>;
    async fn flush(&mut self) -> std::io::Result<()>;
}
