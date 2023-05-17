use std::io;

use crate::size;
use crate::synchronous::Wire;

pub trait Body {
    fn size_hint(&self) -> size::Hint;

    fn write_to<Destination>(self, wire: &mut Destination) -> io::Result<usize>
    where
        Destination: Wire;
}

impl Body for &[u8] {
    fn size_hint(&self) -> size::Hint {
        size::Hint::exact(self.len())
    }

    fn write_to<Destination>(self, wire: &mut Destination) -> io::Result<usize>
    where
        Destination: Wire,
    {
        wire.write_all(&self)?;

        Ok(self.len())
    }
}

impl Body for &str {
    fn size_hint(&self) -> size::Hint {
        self.as_bytes().size_hint()
    }

    fn write_to<Destination>(self, wire: &mut Destination) -> io::Result<usize>
    where
        Destination: Wire,
    {
        self.as_bytes().write_to(wire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let message = "Hello, world!";

        assert_eq!(size::Hint::exact(message.len()), message.size_hint())
    }
}
