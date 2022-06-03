//! A cyclical queue for structures that re-uses allocated structures.
//! Useful when the items are expensive to allocate.
//! Heavily inspired by https://doc.rust-lang.org/nomicon/vec/vec.html
//! May want to use `Vec::with_capacity` instead of allocating myself.

use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

pub struct Queue<T> {
    pointer: NonNull<T>,
    capacity: usize,
    length: usize,
    head: usize,
    tail: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Sync> Sync for Queue<T> {}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        
        let layout = Layout::array::<T>(capacity).unwrap();
        
        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let pointer = unsafe { alloc::alloc(layout) };

        // If allocation fails, `pointer` will be null, in which case we abort.
        let pointer = NonNull::new(pointer as *mut T).unwrap_or_else(|| alloc::handle_alloc_error(layout));

        Self {
            pointer,
            capacity,
            length: 0,
            head: 0,
            tail: 0,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        // To calculate length:
        // if head is less than tail, tail - head
        // else capacity - (head - tail)

        // Essentially the below, but need to handle overflow and sign
        // ((self.tail - self.head) + self.capacity) % self.capacity
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

    // enqueue
    pub fn push(&mut self, element: T) -> bool {
        if self.length == self.capacity {
            return false;
        }

        unsafe {
            ptr::write(self.pointer.as_ptr().add(self.tail), element);
        }
    
        // Can't fail, we'll OOM first.
        self.length += 1;
        self.tail = (self.tail + 1) % self.capacity;

        true
    }

    // dequeue
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            let offset = self.head;

            self.length -= 1;
            self.head = (self.head + 1) % self.capacity;

            unsafe {
                Some(ptr::read(self.pointer.as_ptr().add(offset)))
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.length == 0 {
            None
        } else {
            unsafe {
                self.pointer.as_ptr().add(self.head).as_ref()
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
        let mut queue = Queue::<[u8;1]>::new(1);

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
        let mut queue = Queue::<[u8;1]>::new(2);

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
