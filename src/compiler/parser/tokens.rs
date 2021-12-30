//! Extension to a token sequence useful for generating a syntax tre.

use crate::compiler::{Kind, Token};
use crate::LexicalError;
use std::iter::Peekable;

/// A sequence of tokens from Lexical Analysis.
pub trait Tokens {
    /// Gets the next `Token` if it matches the expected kind. Otherwise, returns `None`.
    /// The underlying `Token` sequence is only advanced on a `Some` return value.
    fn next_kind(&mut self, expected: &[Kind]) -> Option<Token>;

    /// Peeks the next `Token`'s `Kind`, if one is present.
    fn peek_kind(&mut self) -> Option<&Kind>;

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    fn has_next(&mut self) -> bool;
}

impl<I: Iterator<Item = Result<Token, LexicalError>>> Tokens for Peekable<I> {
    fn next_kind(&mut self, expected: &[Kind]) -> Option<Token> {
        if matches!(self.peek()?, Ok(token) if expected.contains(token.kind())) {
            self.next()?.ok()
        } else {
            None
        }
    }

    fn peek_kind(&mut self) -> Option<&Kind> {
        match self.peek()? {
            Ok(token) => Some(token.kind()),
            Err(_) => None,
        }
    }

    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::errors::lexical::ErrorKind;
    use crate::compiler::Lexeme;

    #[test]
    fn has_next_when_empty() {
        let tokens: Vec<Result<Token, LexicalError>> = vec![];
        let mut peekable = tokens.into_iter().peekable();

        assert!(!peekable.has_next());
    }

    #[test]
    fn has_next_with_tokens() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert!(peekable.has_next());
    }

    #[test]
    fn has_next_with_tokens_peeked() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        peekable.peek().unwrap();

        assert!(peekable.has_next());
    }

    #[test]
    fn next_kind_when_expected_empty() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_kind(&[]), None);
    }

    #[test]
    fn next_kind_when_empty() {
        let tokens: Vec<Result<Token, LexicalError>> = vec![];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_kind(&[Kind::Number]), None);
    }

    #[test]
    fn next_kind_with_tokens() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(
            peekable.next_kind(&[Kind::Number]),
            Some(Token::new("1", Kind::Number))
        );
    }

    #[test]
    fn next_kind_with_tokens_peeked() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        peekable.peek().unwrap();

        assert_eq!(
            peekable.next_kind(&[Kind::Number]),
            Some(Token::new("1", Kind::Number))
        );
    }

    #[test]
    fn peek_kind_empty() {
        let tokens: Vec<Result<Token, LexicalError>> = vec![];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.peek_kind(), None);
    }

    #[test]
    fn peek_kind_with_tokens() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.peek_kind(), Some(&Kind::Number));
    }

    #[test]
    fn peek_kind_then_next() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        peekable.next().unwrap();
        peekable.peek_kind().unwrap();

        assert_eq!(
            peekable.next(),
            Some(Ok(Token::new(Lexeme::new("1", "1+"), Kind::Plus)))
        );
    }

    #[test]
    fn peek_kind_multiple() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.peek_kind().copied(), peekable.peek_kind().copied());

        assert_eq!(peekable.next(), Some(Ok(Token::new("1", Kind::Number))));
    }

    #[test]
    fn next_kind_invalid() {
        let tokens = vec![Err(LexicalError::new("|", ErrorKind::Invalid))];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_kind(&[Kind::Number]), None)
    }

    #[test]
    fn peek_kind_invalid() {
        let tokens = vec![Err(LexicalError::new("|", ErrorKind::Invalid))];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.peek_kind(), None)
    }

    #[test]
    fn has_next_invalid() {
        let tokens = vec![Err(LexicalError::new(".", ErrorKind::Number))];
        let mut peekable = tokens.into_iter().peekable();

        assert!(peekable.has_next())
    }

    fn new_tokens() -> Vec<Result<Token, LexicalError>> {
        vec![
            Ok(Token::new("1", Kind::Number)),
            Ok(Token::new(Lexeme::new("1", "1+"), Kind::Plus)),
            Ok(Token::new(Lexeme::new("1+", "1+1"), Kind::Number)),
        ]
    }
}
