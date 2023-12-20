use std::collections::LinkedList;
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SlotIndex(NonZeroUsize);

impl Default for SlotIndex {
    fn default() -> Self {
        Self(NonZeroUsize::MIN)
    }
}

impl SlotIndex {
    pub fn new(index: usize) -> Self {
        Self(NonZeroUsize::new(index).expect("Slot index must be non-zero."))
    }

    pub fn get(&self) -> usize {
        self.0.get()
    }
}

// A naturally indexed (starting at 1) slab of client sockets.
pub struct SwitchBoard<T> {
    slots: Vec<Option<T>>,
    available: LinkedList<SlotIndex>,
}

impl<T> Default for SwitchBoard<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SwitchBoard<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            available: LinkedList::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            available: LinkedList::new(),
        }
    }

    pub fn remove(&mut self, slot: SlotIndex) {
        let length = self.slots.len();

        if slot.get() == length {
            self.slots.pop();
        } else {
            self.slots[slot.get() - 1].take();
            self.available.push_back(slot);
        }
    }

    pub fn add(&mut self, value: T) -> SlotIndex {
        let slot = self.reserve();
        self[slot] = Some(value);
        slot
    }

    fn reserve(&mut self) -> SlotIndex {
        self.available.pop_front().unwrap_or_else(|| {
            self.slots.reserve(1);
            self.slots.push(None);

            let slot = self.slots.len();

            SlotIndex(NonZeroUsize::new(slot).unwrap_or(NonZeroUsize::MIN))
        })
    }
}

impl<T> Index<SlotIndex> for SwitchBoard<T> {
    type Output = Option<T>;

    fn index(&self, index: SlotIndex) -> &Self::Output {
        self.slots.index(index.get() - 1)
    }
}

impl<T> IndexMut<SlotIndex> for SwitchBoard<T> {
    fn index_mut(&mut self, index: SlotIndex) -> &mut Self::Output {
        self.slots.index_mut(index.get() - 1)
    }
}

impl<T> Index<usize> for SwitchBoard<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.slots.index(index).as_ref().expect("Found empty slot")
    }
}

impl<T> IndexMut<usize> for SwitchBoard<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.slots
            .index_mut(index)
            .as_mut()
            .expect("Found empty slot")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut board = SwitchBoard::new();

        let slot1 = board.reserve();
        let slot2 = board.reserve();
        let slot3 = board.reserve();

        board[slot1] = Some(42);
        board[slot2] = Some(0);

        board.remove(slot2);

        assert_eq!(board[slot1], Some(42));
        assert_eq!(board[slot2], None);
        assert_eq!(board[slot3], None);
    }

    #[test]
    #[should_panic]
    fn remove_last() {
        let mut board: SwitchBoard<()> = SwitchBoard::new();
        let slot1 = board.reserve();

        board.remove(slot1);

        board[slot1];
    }

    #[test]
    #[should_panic]
    fn index_usize_empty() {
        let mut board: SwitchBoard<()> = SwitchBoard::new();

        board.reserve();
        board[0];
    }

    #[test]
    #[should_panic]
    fn index_usize_too_large() {
        let board: SwitchBoard<()> = SwitchBoard::new();

        board[0];
    }
}
