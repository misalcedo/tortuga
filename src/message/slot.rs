use super::envelope::Envelope;
use std::mem;

#[derive(Copy, Debug)]
pub enum Slot<const BYTES: usize> {
    Empty,
    Available(Envelope<BYTES>),
    Occupied(Envelope<BYTES>),
}

impl<const BYTES: usize> Default for Slot<BYTES> {
    fn default() -> Self {
        Self::Available(Envelope::default())
    }
}

impl<const BYTES: usize> Clone for Slot<BYTES> {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Available(envelope) => Self::Available(*envelope),
            Self::Occupied(envelope) => Self::Occupied(*envelope),
        }
    }
}

impl<const BYTES: usize> Slot<BYTES> {
    pub fn empty(&mut self) -> Option<Envelope<BYTES>> {
        match mem::replace(self, Self::Empty) {
            Self::Empty => None,
            Self::Available(envelope) => Some(envelope),
            Self::Occupied(envelope) => Some(envelope),
        }
    }

    pub fn take(&mut self) -> Option<Envelope<BYTES>> {
        match self {
            Self::Empty => Some(Envelope::default()),
            Self::Available(_) => self.empty(),
            Self::Occupied(_) => None,
        }
    }

    pub fn vacate(&mut self) -> Option<Envelope<BYTES>> {
        match self {
            Self::Empty | Self::Available(_) => None,
            Self::Occupied(_) => self.empty(),
        }
    }

    pub fn insert(&mut self, envelope: Envelope<BYTES>) -> bool {
        match self {
            Self::Occupied(_) => false,
            Self::Empty | Self::Available(_) => {
                *self = Slot::Occupied(envelope);
                true
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Empty => *self = Self::Available(Envelope::default()),
            Self::Available(_) => (),
            Self::Occupied(_) => {
                let mut envelope = self.vacate().unwrap_or_default();
                envelope.clear();
                *self = Self::Available(envelope);
            }
        }
    }
}
