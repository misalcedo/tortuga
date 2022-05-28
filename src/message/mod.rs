mod envelope;
mod slot;

use envelope::Envelope;
use slot::Slot;

#[derive(Debug)]
struct PooledCircularQueue<const SLOTS: usize, const BYTES: usize> {
    slots: [Slot<BYTES>; SLOTS],
    next_push: usize,
    next_pop: usize,
}

impl<const SLOTS: usize, const BYTES: usize> Default for PooledCircularQueue<SLOTS, BYTES> {
    fn default() -> Self {
        Self {
            slots: [Slot::default(); SLOTS],
            next_push: 0,
            next_pop: 0,
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

    pub fn clear(&mut self) {
        for slot in self.slots.iter_mut() {
            slot.clear();
        }
    }

    pub fn create(&mut self) -> Envelope<BYTES> {
        self.slots
            .get_mut(self.next_push)
            .and_then(|slot| slot.take())
            .unwrap_or_default()
    }

    #[must_use = "There may not be enough space to push to the queue."]
    pub fn push(&mut self, envelope: Envelope<BYTES>) -> bool {
        match self.slots.get_mut(self.next_push) {
            None | Some(Slot::Occupied(_)) => false,
            Some(slot @ Slot::Empty) => {
                self.next_push = (self.next_push + 1) % SLOTS;
                slot.insert(envelope)
            }
            Some(slot @ Slot::Available(_)) => {
                self.next_push = (self.next_push + 1) % SLOTS;
                slot.insert(envelope)
            }
        }
    }

    pub fn pop(&mut self) -> Option<Envelope<BYTES>> {
        match self.slots.get_mut(self.next_push)? {
            Slot::Empty | Slot::Available(_) => None,
            slot @ Slot::Occupied(_) => {
                let envelope = slot.vacate();
                self.next_push = (self.next_push + 1) % SLOTS;

                envelope
            }
        }
    }

    pub fn peek(&mut self) -> Option<&Envelope<BYTES>> {
        match self.slots.get(self.next_push)? {
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

    #[test]
    fn unpooled_case() {
        const BYTES: usize = 1;
        const FIRST: usize = 0;

        let mut queue = PooledCircularQueue::<1, BYTES>::default();
        
        assert!(queue.push(Envelope::new([1])));
        assert!(queue.push(Envelope::new([2])));

        assert_eq!(queue.pop().unwrap().as_ref()[FIRST], 1);
        assert!(queue.push(Envelope::new([3])));
        assert_eq!(queue.pop().unwrap().as_ref()[FIRST], 2);
        assert!(queue.push(Envelope::new([4])));

        assert_eq!(queue.peek().unwrap().as_ref()[FIRST], 3);

        assert!(queue.push(Envelope::new([5])));
        assert!(queue.push(Envelope::new([6])));

        assert_eq!(queue.peek().unwrap().as_ref()[FIRST], 3);
        assert_eq!(queue.pop().unwrap().as_ref()[FIRST], 3);
        assert_eq!(queue.pop().unwrap().as_ref()[FIRST], 4);
        assert_eq!(queue.pop().unwrap().as_ref()[FIRST], 5);
        assert_eq!(queue.pop().unwrap().as_ref()[FIRST], 6);
        assert!(queue.pop().is_none());
    }
}
