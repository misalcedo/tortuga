use super::envelope::Envelope;
use super::slot::Slot;

use std::fmt::{self, Debug};

struct PooledCircularQueue<const SLOTS: usize, const BYTES: usize> {
    next_push: usize,
    next_pop: usize,
    slots: [Slot<BYTES>; SLOTS],
}

pub struct IntoIter<const SLOTS: usize, const BYTES: usize> {
    queue: PooledCircularQueue<SLOTS, BYTES>,
}

pub struct Iter<'a, const SLOTS: usize, const BYTES: usize> {
    offset: usize,
    queue: &'a PooledCircularQueue<SLOTS, BYTES>,
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

impl<const SLOTS: usize, const BYTES: usize> Clone for PooledCircularQueue<SLOTS, BYTES> {
    fn clone(&self) -> Self {
        Self {
            slots: self.slots.clone(),
            next_push: self.next_push,
            next_pop: self.next_pop,
        }
    }
}

impl<const SLOTS: usize, const BYTES: usize> PooledCircularQueue<SLOTS, BYTES> {
    pub fn len(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| matches!(slot, Slot::Occupied(_)))
            .count()
    }

    pub const fn capacity(&self) -> usize {
        SLOTS
    }

    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
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
        if self.capacity() == 0 {
            return false;
        }

        let slot = &mut self.slots[self.next_push];
        let inserted = slot.insert(envelope);

        if inserted {
            self.next_push = self.next_push.wrapping_add(1) % SLOTS;
        }

        inserted
    }

    #[must_use = "There may not be enough space to push to the queue."]
    pub fn push_pooled<F>(&mut self, mutator: F) -> bool
    where
        F: FnOnce(&mut Envelope<BYTES>),
    {
        let mut envelope = self.create();

        mutator(&mut envelope);

        self.push(envelope)
    }

    pub fn pop(&mut self) -> Option<Envelope<BYTES>> {
        let slot = self.slots.get_mut(self.next_pop)?;
        let envelope = slot.vacate();

        if envelope.is_some() {
            self.next_pop = self.next_pop.wrapping_add(1) % SLOTS;
        }

        envelope
    }

    pub fn peek(&self) -> Option<&Envelope<BYTES>> {
        self.slots.get(self.next_pop)?.peek()
    }

    pub fn iter(&self) -> Iter<'_, SLOTS, BYTES> {
        Iter {
            offset: 0,
            queue: self,
        }
    }
}

impl<const SLOTS: usize, const BYTES: usize> IntoIterator for PooledCircularQueue<SLOTS, BYTES> {
    type Item = Envelope<BYTES>;
    type IntoIter = IntoIter<SLOTS, BYTES>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { queue: self }
    }
}

impl<'a, const SLOTS: usize, const BYTES: usize> IntoIterator
    for &'a PooledCircularQueue<SLOTS, BYTES>
{
    type Item = &'a Envelope<BYTES>;
    type IntoIter = Iter<'a, SLOTS, BYTES>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const SLOTS: usize, const BYTES: usize> Iterator for IntoIter<SLOTS, BYTES> {
    type Item = Envelope<BYTES>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.queue.len(), Some(self.queue.len()))
    }
}

impl<'a, const SLOTS: usize, const BYTES: usize> ExactSizeIterator for IntoIter<SLOTS, BYTES> {
    fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<'a, const SLOTS: usize, const BYTES: usize> Iterator for Iter<'a, SLOTS, BYTES> {
    type Item = &'a Envelope<BYTES>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.len() {
            return None;
        }

        let index = self.queue.next_pop.wrapping_add(self.offset) % SLOTS;
        let slot = self.queue.slots.get(index)?;
        let envelope = slot.peek();

        if envelope.is_some() {
            self.offset += 1;
        }

        envelope
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.queue.len(), Some(self.queue.len()))
    }
}

impl<'a, const SLOTS: usize, const BYTES: usize> ExactSizeIterator for Iter<'a, SLOTS, BYTES> {
    fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<const SLOTS: usize, const BYTES: usize> Debug for PooledCircularQueue<SLOTS, BYTES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<const SLOTS: usize, const BYTES: usize> PartialEq for PooledCircularQueue<SLOTS, BYTES> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<const SLOTS: usize, const BYTES: usize> Eq for PooledCircularQueue<SLOTS, BYTES> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_case() {
        let mut queue = PooledCircularQueue::<0, 512>::default();
        let envelope = queue.create();

        assert!(!queue.push(envelope));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn empty_case() {
        let mut queue = PooledCircularQueue::<5, 512>::default();
        let _envelope = queue.create();

        assert_eq!(queue.pop(), None);
        assert_eq!(queue.peek(), None);
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

        assert_eq!(queue.pop(), None);
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
    fn into_iter() {
        const FIRST: usize = 0;

        let mut queue = PooledCircularQueue::<3, 1>::default();

        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 1));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 2));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 3));

        let mut iter = queue.into_iter();

        assert_eq!(iter.next(), Some(Envelope::new([1])));
        assert_eq!(iter.next(), Some(Envelope::new([2])));
        assert_eq!(iter.next(), Some(Envelope::new([3])));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        const FIRST: usize = 0;

        let mut queue = PooledCircularQueue::<3, 1>::default();

        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 1));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 2));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 3));

        let mut iter = queue.iter();

        assert_eq!(iter.next(), Some(&Envelope::new([1])));
        assert_eq!(iter.next(), Some(&Envelope::new([2])));
        assert_eq!(iter.next(), Some(&Envelope::new([3])));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pooled_case() {
        const SLOTS: usize = 4;
        const BYTES: usize = 1;
        const FIRST: usize = 0;

        let mut queue = PooledCircularQueue::<SLOTS, BYTES>::default();

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), SLOTS);
        assert!(queue.is_empty());

        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 1));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 2));

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.capacity(), SLOTS);

        assert_eq!(queue.pop(), Some(Envelope::new([1])));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 3));
        assert_eq!(queue.pop(), Some(Envelope::new([2])));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 4));

        assert_eq!(queue.peek(), Some(&Envelope::new([3])));

        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 5));
        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 6));

        assert_eq!(queue.len(), 4);
        assert_eq!(queue.capacity(), SLOTS);

        assert_eq!(queue.peek(), Some(&Envelope::new([3])));
        assert_eq!(queue.pop(), Some(Envelope::new([3])));

        let mut iter = queue.iter();

        assert_eq!(iter.next(), Some(&Envelope::new([4])));
        assert_eq!(iter.next(), Some(&Envelope::new([5])));
        assert_eq!(iter.next(), Some(&Envelope::new([6])));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert_eq!(queue.pop(), Some(Envelope::new([4])));
        assert_eq!(queue.peek(), Some(&Envelope::new([5])));

        assert!(queue.push_pooled(|e| e.as_mut()[FIRST] = 7));
        assert_eq!(queue.pop(), Some(Envelope::new([5])));
        assert_eq!(queue.pop(), Some(Envelope::new([6])));

        for item in queue {
            assert_eq!(item, Envelope::new([7]));
        }
    }

    #[test]
    fn unpooled_case() {
        const SLOTS: usize = 4;
        const BYTES: usize = 1;

        let mut queue = PooledCircularQueue::<SLOTS, BYTES>::default();

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), SLOTS);
        assert!(queue.is_empty());

        assert!(queue.push(Envelope::new([1])));
        assert!(queue.push(Envelope::new([2])));

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.capacity(), SLOTS);

        assert_eq!(queue.pop(), Some(Envelope::new([1])));
        assert!(queue.push(Envelope::new([3])));
        assert_eq!(queue.pop(), Some(Envelope::new([2])));
        assert!(queue.push(Envelope::new([4])));

        assert_eq!(queue.peek(), Some(&Envelope::new([3])));

        assert!(queue.push(Envelope::new([5])));
        assert!(queue.push(Envelope::new([6])));

        assert_eq!(queue.len(), 4);
        assert_eq!(queue.capacity(), SLOTS);

        assert_eq!(queue.peek(), Some(&Envelope::new([3])));
        assert_eq!(queue.pop(), Some(Envelope::new([3])));

        let mut iter = queue.iter();

        assert_eq!(iter.next(), Some(&Envelope::new([4])));
        assert_eq!(iter.next(), Some(&Envelope::new([5])));
        assert_eq!(iter.next(), Some(&Envelope::new([6])));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert_eq!(queue.pop(), Some(Envelope::new([4])));
        assert_eq!(queue.peek(), Some(&Envelope::new([5])));

        assert!(queue.push(Envelope::new([7])));
        assert_eq!(queue.pop(), Some(Envelope::new([5])));
        assert_eq!(queue.pop(), Some(Envelope::new([6])));

        for item in queue {
            assert_eq!(item, Envelope::new([7]));
        }
    }
}
