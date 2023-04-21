use std::mem::size_of;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum FrameType {
    Data = 0x00,
    Header = 0x01,
}

impl TryFrom<u8> for FrameType {
    type Error = u8;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            0x00 => Ok(FrameType::Data),
            0x01 => Ok(FrameType::Header),
            _ => Err(kind)
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    kind: FrameType,
    length: usize,
}

impl Frame {
    pub fn new(kind: FrameType, length: usize) -> Self {
        Frame {
            kind,
            length
        }
    }

    pub fn bytes() -> usize {
        size_of::<u8>() + size_of::<u64>()
    }

    pub fn kind(&self) -> FrameType {
        self.kind
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
