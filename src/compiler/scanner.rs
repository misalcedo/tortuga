//! Performs lexical analysis on Tortuga input and produces a sequence of `Token`s.

use crate::compiler::{Input, Token};
use std::str::Chars;

/// A lexical analyzer with 1 character of lookahead.
#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    input: Input<Chars<'a>>,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            input: source.into(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'a> Lexer<'a> {}
