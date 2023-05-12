use std::io;

use crate::asynchronous::Wire;
use crate::size;
use async_trait::async_trait;

#[async_trait]
pub trait Body: Send + Sync {
    async fn size_hint(&self) -> size::Hint;

    async fn write_to<Destination>(self, wire: &mut Destination) -> io::Result<usize>
    where
        Destination: Wire;
}

#[async_trait]
impl Body for &[u8] {
    async fn size_hint(&self) -> size::Hint {
        size::Hint::exact(self.len())
    }

    async fn write_to<Destination>(self, wire: &mut Destination) -> io::Result<usize>
    where
        Destination: Wire,
    {
        wire.write_all(&self).await?;

        Ok(self.len())
    }
}

#[async_trait]
impl Body for &str {
    async fn size_hint(&self) -> size::Hint {
        self.as_bytes().size_hint().await
    }

    async fn write_to<Destination>(self, wire: &mut Destination) -> io::Result<usize>
    where
        Destination: Wire,
    {
        self.as_bytes().write_to(wire).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn string() {
        let message = "Hello, world!";

        assert_eq!(size::Hint::exact(message.len()), message.size_hint().await)
    }
}
