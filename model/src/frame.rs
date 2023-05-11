#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Kind {
    Data = 0x00,
    Header = 0x01,
}

impl TryFrom<u8> for Kind {
    type Error = u8;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            0x00 => Ok(Kind::Data),
            0x01 => Ok(Kind::Header),
            _ => Err(kind),
        }
    }
}

impl From<Kind> for u8 {
    fn from(value: Kind) -> Self {
        value as u8
    }
}

pub trait Frame {
    fn kind(&self) -> Kind;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Header(usize);

impl From<usize> for Header {
    fn from(value: usize) -> Self {
        Header(value)
    }
}

impl Frame for Header {
    fn kind(&self) -> Kind {
        Kind::Header
    }

    fn len(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Data(usize);

impl From<usize> for Data {
    fn from(value: usize) -> Self {
        Data(value)
    }
}

impl Frame for Data {
    fn kind(&self) -> Kind {
        Kind::Data
    }

    fn len(&self) -> usize {
        self.0
    }
}
