#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Buffer {
    buffer: Vec<u8>,
    read_cursor: usize,
    write_cursor: usize,
    length: usize,
}

impl Buffer {
    pub fn new(capacity: usize) -> Self {
        Buffer {
            buffer: vec![0u8; capacity],
            read_cursor: 0,
            write_cursor: 0,
            length: 0,
        }
    }

    #[must_use]
    pub fn reserve(&mut self, additional_capacity: usize) -> bool {
        let resizable = self.is_empty();

        if resizable {
            self.buffer.reserve_exact(additional_capacity);

            for _ in 0..additional_capacity {
                self.buffer.push(0);
            }
        }

        resizable
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let end = self.buffer.len();
        let bytes = self.length.min(buffer.len());

        for byte in 0..bytes {
            buffer[byte] = self.buffer[self.read_cursor];
            self.read_cursor = (self.read_cursor + 1) % end;
        }

        self.length -= bytes;

        bytes
    }

    pub fn write(&mut self, buffer: &[u8]) -> usize {
        let end = self.buffer.len();
        let remaining = end - self.length;
        let bytes = remaining.min(buffer.len());

        for byte in 0..bytes {
            self.buffer[self.write_cursor] = buffer[byte];
            self.write_cursor = (self.write_cursor + 1) % end;
        }

        self.length += bytes;

        bytes
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write() {
        let input = [42];
        let mut output = [0];
        let mut buffer = Buffer::new(1);

        assert_eq!(buffer.write(&input), 1);
        assert_eq!(buffer.write(&input), 0);

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.remaining(), 0);

        assert_eq!(buffer.read(&mut output), 1);

        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.remaining(), 1);

        assert_eq!(buffer.write(&input), 1);

        assert_eq!(input, output);
    }

    #[test]
    fn looped() {
        let input = [42; 2];
        let mut output = [0; 1];
        let mut buffer = Buffer::new(3);

        assert_eq!(buffer.write(&input), 2);
        assert_eq!(buffer.read(&mut output), 1);
        assert_eq!(output, [42]);
        assert_eq!(buffer.write(&input), 2);
        assert_eq!(buffer.read(&mut output), 1);
        assert_eq!(output, [42]);
        assert_eq!(buffer.read(&mut output), 1);
        assert_eq!(output, [42]);
        assert_eq!(buffer.read(&mut output), 1);
        assert_eq!(output, [42]);
        assert_eq!(buffer.read(&mut output), 0);
        assert_eq!(output, [42]);
    }

    #[test]
    fn unallocated() {
        let mut buffer = Buffer::default();

        assert_eq!(buffer.write(&[42]), 0);
        assert_eq!(buffer.read(&mut [0]), 0);

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn empty() {
        let buffer = Buffer::new(1);

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn empty_used() {
        let mut buffer = Buffer::new(1);

        buffer.write(&[42]);
        buffer.read(&mut [0]);

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn reserve_non_empty() {
        let mut buffer = Buffer::new(1);

        buffer.write(&[42]);

        assert!(!buffer.reserve(1));
    }

    #[test]
    fn reserve_used() {
        let mut buffer = Buffer::new(1);

        buffer.write(&[42]);
        buffer.read(&mut [0]);

        assert!(buffer.reserve(1));
    }

    #[test]
    fn reserve_unallocated() {
        let mut buffer = Buffer::default();

        assert!(buffer.reserve(1));
    }

    #[test]
    fn reserve_empty() {
        let mut buffer = Buffer::new(1);

        assert!(buffer.reserve(1));
    }

    #[test]
    fn used_empty() {}
}
