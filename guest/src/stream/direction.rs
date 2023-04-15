#[derive(Debug, PartialEq, Clone)]
pub enum Bidirectional {}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadOnly {}

#[derive(Debug, PartialEq, Clone)]
pub enum WriteOnly {}

pub trait Readable: private::Sealed {}

pub trait Writable: private::Sealed {}

mod private {
    use super::{Bidirectional, ReadOnly, WriteOnly};

    pub trait Sealed {}

    impl Sealed for Bidirectional {}

    impl Sealed for ReadOnly {}

    impl Sealed for WriteOnly {}
}

impl Readable for Bidirectional {}

impl Readable for ReadOnly {}

impl Writable for Bidirectional {}

impl Writable for WriteOnly {}
