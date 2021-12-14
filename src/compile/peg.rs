//! Uses a PEG grammar to validate a source file.

#[derive(pest_derive::Parser)]
#[grammar = "../docs/grammar.pest"]
pub struct Parser;
