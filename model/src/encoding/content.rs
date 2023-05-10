use std::fs::File;
use std::io::Cursor;

pub trait ContentLength {
    fn length(&mut self) -> Option<usize>;
}

impl ContentLength for str {
    fn length(&mut self) -> Option<usize> {
        Some(self.len())
    }
}

impl ContentLength for String {
    fn length(&mut self) -> Option<usize> {
        Some(self.len())
    }
}

impl ContentLength for Vec<u8> {
    fn length(&mut self) -> Option<usize> {
        Some(self.len())
    }
}

impl ContentLength for File {
    fn length(&mut self) -> Option<usize> {
        Some(self.metadata().ok()?.len() as usize)
    }
}

impl ContentLength for Cursor<Vec<u8>> {
    fn length(&mut self) -> Option<usize> {
        Some(self.get_ref().len() - self.position() as usize)
    }
}
