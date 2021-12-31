//! Extension to a token sequence useful for generating a syntax tre.

use crate::compiler::{Kind, Token};
use crate::LexicalError;
use std::iter::Peekable;

/// Determines whether a token matches a given pattern.
pub trait TokenMatcher {
    /// Tests whether a token matches a given pattern.
    fn matches(&self, token: &Token) -> bool;
}

impl TokenMatcher for Kind {
    fn matches(&self, token: &Token) -> bool {
        token.kind() == self
    }
}

impl<S: AsRef<[Kind]>> TokenMatcher for S {
    fn matches(&self, token: &Token) -> bool {
        self.as_ref().contains(token.kind())
    }
}

/// A sequence of tokens from Lexical Analysis.
pub trait Tokens {
    /// Gets the next `Token` if it the given `Matcher` returns [`true`]. Otherwise, returns [`None`].
    /// The underlying `Token` sequence is only advanced on a [`Some`] return value.
    fn next_if_match<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<Token>;

    /// Peeks the next `Token`'s `Kind`, if one is present.
    fn peek_kind(&mut self) -> Option<&Kind>;

    /// Tests whether the next `Token`'s `Kind` is the expected one.
    /// Returns [`None`] on an empty sequence.
    /// Does not advance the sequence.
    fn next_matches<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<bool>;

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    fn has_next(&mut self) -> bool;
}

impl<I: Iterator<Item = Result<Token, LexicalError>>> Tokens for Peekable<I> {
    fn next_if_match<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<Token> {
        if matches!(self.peek()?, Ok(token) if matcher.matches(token)) {
            if let Some(Ok(ref token)) = self.peek() {
                println!("Token at: {}.", token.lexeme().start());
            }

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

    fn next_matches<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<bool> {
        Some(matches!(self.peek()?, Ok(token) if matcher.matches(token)))
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
    fn next_matches_when_empty() {
        let tokens: Vec<Result<Token, LexicalError>> = vec![];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_matches(Kind::Number), None);
    }

    #[test]
    fn next_matches_with_tokens() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_matches(Kind::Number), Some(true));
        assert_eq!(peekable.next_matches(Kind::Identifier), Some(false));
    }

    #[test]
    fn next_matches_with_tokens_peeked() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        peekable.peek().unwrap();

        assert_eq!(peekable.next_matches(Kind::Number), Some(true));
        assert_eq!(peekable.next_matches(Kind::Identifier), Some(false));
    }

    #[test]
    fn next_if_match_when_expected_empty() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_if_match(&[][..]), None);
    }

    #[test]
    fn next_if_match_when_empty() {
        let tokens: Vec<Result<Token, LexicalError>> = vec![];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_if_match(&[Kind::Number]), None);
    }

    #[test]
    fn next_if_match_with_tokens() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(
            peekable.next_if_match(&[Kind::Number]),
            Some(Token::new("1", Kind::Number))
        );
    }

    #[test]
    fn next_if_match_with_tokens_peeked() {
        let tokens = new_tokens();
        let mut peekable = tokens.into_iter().peekable();

        peekable.peek().unwrap();

        assert_eq!(
            peekable.next_if_match(&[Kind::Number]),
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

        peekable.next().unwrap().unwrap();
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
    fn next_if_match_invalid() {
        let tokens = vec![Err(LexicalError::new("|", ErrorKind::Invalid))];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_if_match(&[Kind::Number]), None)
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

    #[test]
    fn next_matches_invalid() {
        let tokens = vec![Err(LexicalError::new(".", ErrorKind::Number))];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_matches(Kind::At), Some(false))
    }

    fn new_tokens() -> Vec<Result<Token, LexicalError>> {
        vec![
            Ok(Token::new("1", Kind::Number)),
            Ok(Token::new(Lexeme::new("1", "1+"), Kind::Plus)),
            Ok(Token::new(Lexeme::new("1+", "1+1"), Kind::Number)),
        ]
    }
}
