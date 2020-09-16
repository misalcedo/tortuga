use crate::wasm::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Address {
    offset: usize,
    length: usize,
}

impl Address {
    /// Creates a new address for a contiguous block with the given memory offset and length in bytes.
    pub fn new(offset: usize, length: usize) -> Address {
        Address { offset, length }
    }

    /// The memory offset of the block represented by this address.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// The length in bytes of the memory block represented by this address.
    pub fn length(&self) -> usize {
        self.length
    }
}

pub(crate) trait Allocator {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Result<Address, Error>;
}
