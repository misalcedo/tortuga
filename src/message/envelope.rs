use std::convert::{AsMut, AsRef};

#[derive(Copy, Debug)]
pub struct Envelope<const BYTES: usize> {
    message: [u8; BYTES],
}

impl<const BYTES: usize> Default for Envelope<BYTES> {
    fn default() -> Self {
        Self {
            message: [0; BYTES],
        }
    }
}

impl<const BYTES: usize> Clone for Envelope<BYTES> {
    fn clone(&self) -> Self {
        Self {
            message: self.message,
        }
    }
}

impl<const BYTES: usize> AsRef<[u8]> for Envelope<BYTES> {
    fn as_ref(&self) -> &[u8] {
        &self.message[..]
    }
}

impl<const BYTES: usize> AsMut<[u8]> for Envelope<BYTES> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.message[..]
    }
}

impl<const BYTES: usize> Envelope<BYTES> {
    pub fn clear(&mut self) {
        for x in self.message.iter_mut() {
            *x = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_message() {
        const SIZE: usize = 512;
        const FIRST: usize = 0;
        const LAST: usize = SIZE - 1;

        let mut envelope = Envelope::<SIZE>::default();

        {
            let message = envelope.as_mut();
            message[FIRST] = 42;
            message[LAST] = 1;
        }
        {
            let message = envelope.as_ref();
            assert_eq!(message[FIRST], 42);
            assert_eq!(message[LAST], 1);
        }

        envelope.clear();

        {
            let message = envelope.as_ref();
            assert!(message.iter().all(|x| *x == 0));
        }
    }
}
