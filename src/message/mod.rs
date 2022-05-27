use std::convert::{AsMut, AsRef};
use std::mem;

#[derive(Copy, Debug)]
struct Envelope<const BYTES: usize> {
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

#[derive(Copy, Debug)]
enum Slot<const BYTES: usize> {
    Empty,
    Available(Envelope<BYTES>),
    Occupied(Envelope<BYTES>),
}

impl<const BYTES: usize> Default for Slot<BYTES> {
    fn default() -> Self {
        Self::Available(Envelope::default())
    }
}

impl<const BYTES: usize> Clone for Slot<BYTES> {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Available(envelope) => Self::Available(*envelope),
            Self::Occupied(envelope) => Self::Occupied(*envelope),
        }
    }
}

impl<const BYTES: usize> Slot<BYTES> {
    pub fn empty(&mut self) -> Option<Envelope<BYTES>> {
        match mem::replace(self, Self::Empty) {
            Self::Empty => None,
            Self::Available(envelope) => Some(envelope),
            Self::Occupied(envelope) => Some(envelope),
        }
    }

    pub fn take(&mut self) -> Option<Envelope<BYTES>> {
        match self {
            Self::Empty => Some(Envelope::default()),
            Self::Available(_) => self.empty(),
            Self::Occupied(_) => None,
        }
    }

    pub fn vacate(&mut self) -> Option<Envelope<BYTES>> {
        match self {
            Self::Empty | Self::Available(_) => None,
            Self::Occupied(_) => self.empty(),
        }
    }

    pub fn insert(&mut self, envelope: Envelope<BYTES>) -> bool {
        match self {
            Self::Occupied(_) => false,
            Self::Empty | Self::Available(_) => {
                *self = Slot::Occupied(envelope);
                true
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Empty => *self = Self::Available(Envelope::default()),
            Self::Available(_) => (),
            Self::Occupied(_) => {
                let mut envelope = self.vacate().unwrap_or_default();
                envelope.clear();
                *self = Self::Available(envelope);
            }
        }
    }
}

#[derive(Debug)]
struct PooledCircularQueue<const SLOTS: usize, const BYTES: usize> {
    slots: [Slot<BYTES>; SLOTS],
    current: usize,
}

impl<const SLOTS: usize, const BYTES: usize> Default for PooledCircularQueue<SLOTS, BYTES> {
    fn default() -> Self {
        Self {
            slots: [Slot::default(); SLOTS],
            current: 0,
        }
    }
}

impl<const SLOTS: usize, const BYTES: usize> PooledCircularQueue<SLOTS, BYTES> {
    pub fn len(&self) -> usize {
        SLOTS
    }

    pub fn is_empty(&self) -> bool {
        self.slots
            .iter()
            .all(|slot| !matches!(slot, Slot::Occupied(_)))
    }

    const fn next(&self) -> usize {
        (self.current + 1) % SLOTS
    }

    pub fn clear(&mut self) {
        for slot in self.slots.iter_mut() {
            slot.clear();
        }
    }

    pub fn create(&mut self) -> Envelope<BYTES> {
        self.slots
            .get_mut(self.current)
            .and_then(|slot| slot.take())
            .unwrap_or_default()
    }

    #[must_use = "There may not be enough space to push to the queue."]
    pub fn push(&mut self, envelope: Envelope<BYTES>) -> bool {
        match self.slots.get_mut(self.current) {
            None | Some(Slot::Occupied(_)) => false,
            Some(slot @ Slot::Empty) => {
                self.current = (self.current + 1) % SLOTS;
                slot.insert(envelope)
            }
            Some(slot @ Slot::Available(_)) => {
                self.current = (self.current + 1) % SLOTS;
                slot.insert(envelope)
            }
        }
    }

    pub fn pop(&mut self) -> Option<Envelope<BYTES>> {
        match self.slots.get_mut(self.current)? {
            Slot::Empty | Slot::Available(_) => None,
            slot @ Slot::Occupied(_) => {
                let envelope = slot.vacate();
                self.current = (self.current + 1) % SLOTS;

                envelope
            }
        }
    }

    pub fn peek(&mut self) -> Option<&Envelope<BYTES>> {
        match self.slots.get(self.current)? {
            Slot::Empty | Slot::Available(_) => None,
            Slot::Occupied(envelope) => Some(envelope),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_case() {
        let mut queue = PooledCircularQueue::<0, 512>::default();
        let envelope = queue.create();

        assert!(!queue.push(envelope));
        assert!(queue.pop().is_none());
        assert!(queue.peek().is_none());
    }

    #[test]
    fn base_case() {
        const BYTES: usize = 512;
        const FIRST: usize = 0;
        const LAST: usize = BYTES - 1;

        let mut queue = PooledCircularQueue::<1, BYTES>::default();
        let mut envelope = queue.create();

        {
            let message = envelope.as_mut();
            message[FIRST] = 42;
            message[LAST] = 1;
        }

        assert!(queue.push(envelope));
        assert!(!queue.push(envelope));

        let message = queue.peek().unwrap();
        assert_eq!(message.as_ref()[FIRST], 42);
        assert_eq!(message.as_ref()[LAST], 1);

        let message = queue.pop().unwrap();
        assert_eq!(message.as_ref()[FIRST], 42);
        assert_eq!(message.as_ref()[LAST], 1);

        assert!(queue.peek().is_none());
        assert!(queue.push(envelope));
    }

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
