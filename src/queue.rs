
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PostMark {
    sender: u128,
    receipient: u128
}

impl PostMark {
    fn new(sender: u128, receipient: u128) -> PostMark {
        PostMark {
            sender,
            receipient
        }
    }
}

// 2 ^ 13 == 8 kibibytes.
/// An envelope that is reused for all messages added to the queue.
/// The length of the message is the capacity, whereas the length property of the envelope
/// is the actual number of bytes used by the message.
// If the post mark field is empty, the message in the envelope is invalid.
pub struct Envelope {
    post_mark: Option<PostMark>, 
    message: [u8; 8192],
    length: usize
}

impl Envelope {
    fn new() -> Envelope {
        Envelope {
            post_mark: None, 
            message: [0u8; 8192],
            length: 8192
        }
    }

    fn seal(&mut self, post_mark: PostMark, message: &[u8]) -> Result<(), String> {
        if message.len() > self.message.len() {
            return Err(String::from("Cannot seal envelope as the message to send is too large."));
        }

        self.post_mark = Some(post_mark);
        self.message[..message.len()].copy_from_slice(message);
        self.length = message.len();

        Ok(())
    }

    fn message(&self) -> Option<&[u8]> {
        if (self.post_mark.is_some()) {
            Some(&self.message[..self.length])
        }
        else {
            None
        }
    }

    fn empty(&mut self) {
        self.post_mark = None;
        self.length = self.message.len();
    }
}

/// A message distributor built on a circular buffer of pre-allocated buffers.
/// The maximum message size is determined by the buffer size of the envelope.
/// Messages that are too large return an error result.
/// Currently limited to a single thread so that reads and writes do not happen concurrently.
pub struct RingBufferQueue {
    read_index: usize,
    write_index: usize,
    buffer: Vec<Envelope>,
    size: usize
}

impl RingBufferQueue {
    pub fn new(capacity: usize) -> RingBufferQueue {
        let mut buffer = Vec::with_capacity(capacity);

        for i in 0..capacity {
            buffer.push(Envelope::new());
        }
        
        RingBufferQueue {
            buffer,
            read_index: 0,
            write_index: 0,
            size: 0
        }
    }

    pub fn push(&mut self, post_mark: PostMark, message: &[u8]) -> Result<(), String> {
        if self.is_full() {
            return Err(String::from("Writer has caught up to reader. Need t read more to free space for the writer."));
        }
        
        let capacity = self.buffer.len();
        let envelope = &mut self.buffer[self.write_index];

        // Not an atomic update of the queue.
        self.size = self.size + 1;
        self.write_index = (self.write_index + 1) % capacity;

        envelope.seal(post_mark, message)
    }

    pub fn pop(&mut self) -> Option<(PostMark, Cow<[u8]>)> {
        if self.size == 0 {
            None
        } else {
            let envelope = &self.buffer[self.read_index];
            let post_mark = envelope.post_mark.unwrap();
            let message = envelope.message().unwrap();

            // Not an atomic update of the queue.
            self.size = self.size - 1;
            self.read_index = (self.read_index + 1) % self.buffer.len();

            Some((post_mark, Cow::from(message)))
        }
    }

    /// Determines whether the queue is full or if the queue is empty.
    /// When the read and write index are equal, 
    /// the queue is either full or empty and we can determine which by either keeping count or checking the contents.
    fn is_full(&self) -> bool {
        self.write_index == self.read_index && self.size != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::{PostMark, RingBufferQueue};

    #[test]
    fn push_single_message() {
        let message = b"Hi!";
        let post_mark = PostMark::new(42, 7);
        let mut queue = RingBufferQueue::new(3);
        
        let result = queue.push(post_mark, message);

        assert!(result.is_ok());
        
        let (actual_post_mark, actual_message) = queue.pop().unwrap();

        assert_eq!(post_mark, actual_post_mark);
        assert_eq!(message[..], actual_message[..]);
    }

    #[test]
    fn push_when_full() {
        let message = b"Hi!";
        let post_mark = PostMark::new(42, 7);
        let mut queue = RingBufferQueue::new(1);
        
        let result = queue.push(post_mark, message);
        let result = queue.push(post_mark, message);

        assert!(result.is_err());
    }
}