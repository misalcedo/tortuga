//! A cyclical queue for structures that re-uses allocated structures.
//! Useful when the items are expensive to allocate.
//! Heavily inspired by https://doc.rust-lang.org/nomicon/vec/vec.html

use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;
use std::alloc::{self, Layout};

pub struct Queue<T> {
    pointer: NonNull<T>,
    capacity: usize,
    length: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Sync> Sync for Queue<T> {}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            pointer: NonNull::dangling(),
            length: 0,
            capacity: 0,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        let (new_capacity, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.capacity <= isize::MAX.
            let new_capacity = 2 * self.capacity;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            (new_capacity, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_pointer = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_pointer = self.pointer.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_pointer, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_pointer` will be null, in which case we abort.
        self.pointer = match NonNull::new(new_pointer as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_capacity;
    }
}