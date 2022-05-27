mod envelope;
mod slot;

use envelope::Envelope;
use slot::Slot;

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
    fn null_case() {
        let mut queue = PooledCircularQueue::<0, 512>::default();
        let envelope = queue.create();

        assert!(!queue.push(envelope));
        assert!(queue.pop().is_none());
        assert!(queue.peek().is_none());
    }

    #[test]
    fn empty_case() {
        let mut queue = PooledCircularQueue::<5, 512>::default();
        let _envelope = queue.create();

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
}
