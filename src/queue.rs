//! A cyclical queue for structures that re-uses allocated structures.
//! Useful when the items are expensive to allocate.
//! Heavily inspired by https://doc.rust-lang.org/nomicon/vec/vec.html
//! May want to use `Vec::with_capacity` instead of allocating myself.

use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};

pub struct Queue<T> {
    pointer: Option<NonNull<T>>,
    capacity: usize,
    length: usize,
    head: usize,
    tail: usize,
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Sync> Sync for Queue<T> {}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        let (pointer, capacity) = if mem::size_of::<T>() == 0 {
            // `NonNull::dangling()` is our "zero-sized allocation"
            // No need to limit capacity for Zero-sized types since we can hold infinite of nothing.
            (Some(NonNull::dangling()), usize::MAX)
        } else if capacity == 0 {
            // `NonNull::dangling()` is our "zero-sized allocation"
            (Some(NonNull::dangling()), capacity)
        } else {
            (None, capacity)
        };

        Self {
            pointer,
            capacity,
            length: 0,
            head: 0,
            tail: 0,
        }
    }

    // Fallible allocation.
    fn allocate(&mut self) -> Option<NonNull<T>> {
        // since we set the capacity to usize::MAX when T has size 0,
        // getting to here necessarily means the queue needs to be allocated.
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let layout = Layout::array::<T>(self.capacity).ok()?;

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let pointer = unsafe { alloc::alloc(layout) };

        // If allocation fails, `pointer` will be null.
        NonNull::new(pointer as *mut T)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn clear(&mut self) {
        self.length = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn push(&mut self, element: T) -> bool {
        self.enqueue(element).is_some()
    }

    fn enqueue(&mut self, element: T) -> Option<()> {
        if self.length == self.capacity {
            return None;
        }

        if self.pointer.is_none() {
            self.pointer = self.allocate();
        }

        unsafe {
            ptr::write(self.pointer?.as_ptr().add(self.tail), element);
        }

        // Can't fail, we'll OOM first.
        self.length += 1;
        self.tail = (self.tail + 1) % self.capacity;

        Some(())
    }

    // dequeue
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            let offset = self.head;

            self.length -= 1;
            self.head = (self.head + 1) % self.capacity;

            unsafe { Some(ptr::read(self.pointer?.as_ptr().add(offset))) }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.length == 0 {
            None
        } else {
            unsafe { self.pointer?.as_ptr().add(self.head).as_ref() }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // empty the queue in case T is Drop
        while let Some(_) = self.pop() {}

        if self.capacity != 0 && mem::size_of::<T>() != 0 {
            if let (Some(pointer), Ok(layout)) = (self.pointer, Layout::array::<T>(self.capacity)) {
                unsafe {
                    alloc::dealloc(pointer.as_ptr() as *mut u8, layout);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_case() {
        let mut queue = Queue::new(0);

        assert!(!queue.push([1]));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn empty_case() {
        let mut queue = Queue::<usize>::new(5);

        assert_eq!(queue.pop(), None);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn base_case() {
        let mut queue = Queue::<[u8; 1]>::new(1);

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.pop(), None);
        assert!(queue.push([1]));
        assert!(!queue.push([2]));

        assert_eq!(queue.peek(), Some(&[1]));
        assert_eq!(queue.peek(), Some(&[1]));

        assert_eq!(queue.pop().unwrap(), [1]);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.len(), 0);
        assert!(queue.push([2]));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn basic_queue() {
        let mut queue = Queue::<[u8; 1]>::new(2);

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.pop(), None);
        assert!(queue.push([1]));
        assert!(queue.push([2]));

        assert_eq!(queue.peek().unwrap(), &[1]);
        assert_eq!(queue.peek().unwrap(), &[1]);

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.pop(), Some([1]));
        assert_eq!(queue.peek(), Some(&[2]));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.pop().unwrap(), [2]);
        assert_eq!(queue.len(), 0);
        assert!(queue.push([3]));
        assert_eq!(queue.len(), 1);
    }
}
