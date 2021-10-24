//! Scans a source code file for lexical tokens (lexemes).

use std::iter::Iterator;

/// Scanner  
pub struct Scanner {
}

/// A lexical token.
#[derive(Debug)]
pub struct Token {}

impl Scanner {
    pub fn new() -> Self {
        Scanner {}
    }

    pub fn scan(&self, code: &str) -> impl Iterator<Item=Token> {
        std::iter::empty::<Token>()
    }
}