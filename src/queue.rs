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
pub struct Envelope {
    post_mark: Option<PostMark>, 
    message: [u8; 8192]
}

impl Envelope {
    fn new() -> Envelope {
        Envelope {
            post_mark: None, 
            message: [0u8; 8192]
        }
    }

    fn seal(post_mark: PostMark, message: &[u8]) -> Result<(), String> {
        Ok(())
    }
}

/// A message distributor built on a circular buffer of pre-allocated buffers.
/// The maximum message size is determined by the buffer size of the envelope.
/// Messages that are too large return an error result.
pub struct RingBufferQueue {
    read_index: Option<usize>,
    write_index: usize,
    buffer: Vec<Envelope>
}

impl RingBufferQueue {
    pub fn new(capacity: usize) -> RingBufferQueue {
        let mut buffer = Vec::with_capacity(capacity);

        for i in 0..capacity {
            buffer.push(Envelope::new());
        }
        
        RingBufferQueue {
            buffer,
            read_index: None,
            write_index: 0
        }
    }

    pub fn push(&mut self, post_mark: PostMark, message: &[u8]) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::{PostMark, RingBufferQueue};

    #[test]
    fn distribute_single_message() {
        let mut distributor = RingBufferQueue::new(3);

        let result = distributor.push(PostMark::new(42, 7), b"Hi!");

        assert!(result.is_ok());
    }
}