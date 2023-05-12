use std::io;

use crate::{asynchronous, size};
use async_trait::async_trait;

#[async_trait]
pub trait Body: Send + Sync {
    async fn size_hint(&self) -> size::Hint;

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire;
}

#[async_trait]
impl Body for [u8] {
    async fn size_hint(&self) -> size::Hint {
        size::Hint::exact(self.as_ref().len())
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        let bytes = self.as_ref();

        wire.write_all(bytes).await?;

        Ok(bytes.len())
    }
}

#[async_trait]
impl Body for str {
    async fn size_hint(&self) -> size::Hint {
        self.as_bytes().size_hint().await
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        self.as_bytes().copy(wire).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cursor() {
        let message = "Hello, world!";

        assert_eq!(size::Hint::exact(message.len()), message.size_hint().await)
    }
}
