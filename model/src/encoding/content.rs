use std::fs::File;
use std::io::Cursor;

use async_trait::async_trait;

#[async_trait]
pub trait ContentLength: Send + Sync {
    async fn length(&mut self) -> Option<usize>;
}

#[async_trait]
impl ContentLength for str {
    async fn length(&mut self) -> Option<usize> {
        Some(self.len())
    }
}

#[async_trait]
impl ContentLength for String {
    async fn length(&mut self) -> Option<usize> {
        Some(self.len())
    }
}

#[async_trait]
impl ContentLength for Vec<u8> {
    async fn length(&mut self) -> Option<usize> {
        Some(self.len())
    }
}

#[async_trait]
impl ContentLength for File {
    async fn length(&mut self) -> Option<usize> {
        Some(self.metadata().ok()?.len() as usize)
    }
}

#[async_trait]
impl ContentLength for Cursor<Vec<u8>> {
    async fn length(&mut self) -> Option<usize> {
        Some(self.get_ref().len() - self.position() as usize)
    }
}
