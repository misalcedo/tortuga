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

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            pointer: NonNull::dangling(),
            length: 0,
            capacity,
            _marker: PhantomData,
        }
    }

    fn allocate(&mut self) {
        // `Layout::array` checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= isize::MAX,
        // so the `unwrap` should never fail.
        let layout = Layout::array::<T>(1).unwrap();
        
        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let pointer = unsafe { alloc::alloc(layout) };

        // If allocation fails, `pointer` will be null, in which case we abort.
        self.pointer = match NonNull::new(pointer as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
    }
}