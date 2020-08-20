#[derive(Debug, Eq, PartialEq)]
pub struct Address {
    offset: u32,
    length: u32,
}

impl Address {
    /// Creates a new address for a contiguous block with the given memory offset and length in bytes.
    pub fn new(offset: u32, length: u32) -> Address {
        Address { offset, length }
    }

    /// The memory offset of the block represented by this address.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// The length in bytes of the memory block represented by this address.
    pub fn length(&self) -> u32 {
        self.length
    }
}

/// A sender and receiver of messages.
/// Defines the contract between the intent and the system.
pub trait Actor {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Address;

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    fn receive(&self, message: Address);
}
