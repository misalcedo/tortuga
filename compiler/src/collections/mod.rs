//! Data structures used throughout the compiler
pub mod forest;
mod indices;
mod stack;
mod tree;

pub use forest::Forest;
pub use indices::IndexedSet;
pub use stack::NonEmptyStack;
pub use tree::Tree;
