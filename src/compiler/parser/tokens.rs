//! Extension to a token sequence useful for generating a syntax tre.

use crate::compiler::{Kind, Token};
use crate::{LexicalError, SyntacticalError};
use std::iter::Peekable;

/// Determines whether a token matches a given pattern.
pub trait TokenMatcher {
    /// Tests whether a token matches a given pattern.
    fn matches(&self, token: &Token) -> bool;
}

impl TokenMatcher for bool {
    fn matches(&self, _: &Token) -> bool {
        *self
    }
}

impl TokenMatcher for Kind {
    fn matches(&self, token: &Token) -> bool {
        token.kind() == self
    }
}

impl TokenMatcher for [Kind] {
    fn matches(&self, token: &Token) -> bool {
        self.contains(token.kind())
    }
}

impl TokenMatcher for &[Kind] {
    fn matches(&self, token: &Token) -> bool {
        self.contains(token.kind())
    }
}

impl<const N: usize> TokenMatcher for [Kind; N] {
    fn matches(&self, token: &Token) -> bool {
        self.contains(token.kind())
    }
}

impl<const N: usize> TokenMatcher for &[Kind; N] {
    fn matches(&self, token: &Token) -> bool {
        self.contains(token.kind())
    }
}

/// A sequence of tokens from Lexical Analysis.
pub trait Tokens<'a> {
    /// Advances the sequence and returns the next [`Token`].
    fn next_token(&mut self) -> Result<Token<'a>, SyntacticalError>;

    /// Peeks at the next [`Token`] in the sequence without advancing.
    fn peek_token(&mut self) -> Option<&Token<'a>>;

    /// Gets the next `Token` if it the given `Matcher` returns [`true`]. Otherwise, returns [`None`].
    /// The underlying `Token` sequence is only advanced on a [`Some`] return value.
    fn next_if_match<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<Token<'a>> {
        if matcher.matches(self.peek_token()?) {
            self.next_token().ok()
        } else {
            None
        }
    }

    /// Peeks the next [`Token`]'s [`Kind`], if one is present.
    fn peek_kind(&mut self) -> Option<Kind> {
        Some(*self.peek_token()?.kind())
    }

    /// Tests whether the next `Token`'s `Kind` is the expected one.
    /// Returns [`None`] on an empty sequence.
    /// Does not advance the sequence.
    fn next_matches<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<bool>;

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    fn has_next(&mut self) -> bool {
        self.next_matches(true).is_some()
    }
}

impl<'a, I: Iterator<Item = Result<Token<'a>, LexicalError>>> Tokens<'a> for Peekable<I> {
    fn next_token(&mut self) -> Result<Token<'a>, SyntacticalError> {
        self.next()
            .ok_or(SyntacticalError::Incomplete)?
            .map_err(SyntacticalError::Lexical)
    }

    fn peek_token(&mut self) -> Option<&Token<'a>> {
        match self.peek().map(Result::as_ref).transpose() {
            Ok(token) => token,
            Err(_) => None,
        }
    }

    fn next_matches<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<bool> {
        match self.peek()? {
            Ok(token) => Some(matcher.matches(token)),
            Err(_) => Some(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::errors::lexical::ErrorKind;
    use crate::compiler::{Lexeme, Location};

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

        assert_eq!(peekable.peek_kind(), Some(Kind::Number));
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

        assert_eq!(peekable.peek_kind(), peekable.peek_kind());

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
        let tokens = vec![Err(LexicalError::new(
            Lexeme::new(Location::default(), "."),
            ErrorKind::Number,
        ))];
        let mut peekable = tokens.into_iter().peekable();

        assert_eq!(peekable.next_matches(Kind::At), Some(false))
    }

    fn new_tokens() -> Vec<Result<Token<'static>, LexicalError>> {
        vec![
            Ok(Token::new("1", Kind::Number)),
            Ok(Token::new(Lexeme::new("1", "1+"), Kind::Plus)),
            Ok(Token::new(Lexeme::new("1+", "1+1"), Kind::Number)),
        ]
    }
}
