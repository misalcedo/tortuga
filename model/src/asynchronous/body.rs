use std::io;
use std::io::Cursor;

use crate::asynchronous;
use async_trait::async_trait;

#[async_trait]
pub trait Body: Send + Sync {
    async fn length(&self) -> Option<usize>;

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire;
}

#[async_trait]
impl Body for str {
    async fn length(&self) -> Option<usize> {
        Some(self.len())
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        self.as_bytes().copy(wire).await
    }
}

#[async_trait]
impl Body for String {
    async fn length(&self) -> Option<usize> {
        Some(self.len())
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        self.as_str().copy(wire).await
    }
}

#[async_trait]
impl Body for [u8] {
    async fn length(&self) -> Option<usize> {
        Some(self.len())
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        wire.write_all(self).await?;

        Ok(self.len())
    }
}

#[async_trait]
impl Body for Vec<u8> {
    async fn length(&self) -> Option<usize> {
        Some(self.len())
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        self.as_slice().copy(wire).await
    }
}

#[async_trait]
impl Body for Cursor<Vec<u8>> {
    async fn length(&self) -> Option<usize> {
        Some(self.get_ref().len() - self.position() as usize)
    }

    async fn copy<Wire>(&self, wire: &mut Wire) -> io::Result<usize>
    where
        Wire: asynchronous::Wire,
    {
        self.get_ref().copy(wire).await
    }
}
