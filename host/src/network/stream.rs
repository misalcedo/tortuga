use crate::network::Pipe;

pub struct DuplexStream {
    to: Pipe,
    from: Pipe,
}

impl DuplexStream {
    pub fn new(capacity: usize) -> (DuplexStream, DuplexStream) {
        let to = Pipe::new(capacity);
        let from = Pipe::new(capacity);

        let b = DuplexStream {
            to: from.clone(),
            from: to.clone(),
        };
        let a = DuplexStream { to, from };

        (a, b)
    }
}
