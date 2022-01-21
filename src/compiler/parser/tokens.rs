//! Extension to a token sequence useful for generating a syntax tre.

use crate::compiler::{Kind, Token};
use crate::{LexicalError, SyntacticalError};

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

/// An iterator sequence of tokens obtained through Lexical Analysis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tokens<'a> {
    offset: usize,
    marker: Option<usize>,
    tokens: Vec<Token<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn try_from<I>(input: I) -> Result<Self, SyntacticalError>
    where
        I: Iterator<Item = Result<Token<'a>, LexicalError>>,
    {
        let mut errors = Vec::new();
        let mut tokens = Vec::new();

        for result in input {
            match result {
                Ok(token) => tokens.push(token),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(Tokens::from(tokens))
        } else {
            Err(SyntacticalError::Lexical(errors))
        }
    }

    /// Marks the current offset as a backtracking point.
    /// Backtracking is only possible when an offset has been marked.
    pub fn mark(&mut self) {
        self.marker = Some(self.offset);
    }

    /// Moves the offset back to the marked point.
    /// If no offset was marked, backtracking does nothing.
    pub fn backtrack(&mut self) {
        if let Some(offset) = self.marker.take() {
            self.offset = offset;
        }
    }

    fn next(&mut self) -> Option<Token<'a>> {
        let token = self.tokens.get(self.offset);

        if token.is_some() {
            self.offset += 1;
        }

        token.cloned()
    }

    /// Peeks at the next [`Token`] in the sequence without advancing.
    pub fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.offset)
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, SyntacticalError> {
        self.next().ok_or(SyntacticalError::Incomplete)
    }

    /// Tests whether the next `Token`'s `Kind` is the expected one.
    /// Returns [`None`] on an empty sequence.
    /// Does not advance the sequence.
    pub fn next_matches<Matcher: TokenMatcher>(&self, matcher: Matcher) -> Option<bool> {
        Some(matcher.matches(self.peek()?))
    }

    /// Gets the next `Token` if it the given `Matcher` returns [`true`]. Otherwise, returns [`None`].
    /// The underlying `Token` sequence is only advanced on a [`Some`] return value.
    pub fn next_if_match<Matcher: TokenMatcher>(&mut self, matcher: Matcher) -> Option<Token<'a>> {
        if matcher.matches(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    /// Peeks the next [`Token`]'s [`Kind`], if one is present.
    pub fn peek_kind(&self) -> Option<Kind> {
        Some(*self.peek()?.kind())
    }

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    pub fn has_next(&self) -> bool {
        self.offset < self.tokens.len()
    }
}

impl<'a> From<Vec<Token<'a>>> for Tokens<'a> {
    fn from(tokens: Vec<Token<'a>>) -> Self {
        Tokens { tokens, offset: 0, marker: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::errors::lexical::ErrorKind;
    use crate::compiler::Lexeme;

    #[test]
    fn backtrack_unmarked() {
        let mut tokens = new_tokens();

        tokens.next().unwrap();
        
        let expected = tokens.clone();
        
        tokens.backtrack();
        
        assert_eq!(tokens, expected);
    }

    #[test]
    fn backtrack_marked() {
        let mut tokens = new_tokens();

        tokens.next().unwrap();

        let expected = tokens.clone();
        
        tokens.mark();
        tokens.backtrack();
        
        assert_eq!(tokens, expected);
    }

    #[test]
    fn has_next_when_empty() {
        let tokens = Tokens::default();

        assert!(!tokens.has_next());
    }

    #[test]
    fn has_next_with_tokens() {
        let tokens = new_tokens();

        assert!(tokens.has_next());
    }

    #[test]
    fn has_next_with_tokens_peeked() {
        let tokens = new_tokens();

        tokens.peek().unwrap();

        assert!(tokens.has_next());
    }

    #[test]
    fn next_matches_when_empty() {
        let tokens = Tokens::default();

        assert_eq!(tokens.next_matches(Kind::Number), None);
    }

    #[test]
    fn next_matches_with_tokens() {
        let tokens = new_tokens();

        assert_eq!(tokens.next_matches(Kind::Number), Some(true));
        assert_eq!(tokens.next_matches(Kind::Identifier), Some(false));
    }

    #[test]
    fn next_matches_with_tokens_peeked() {
        let tokens = new_tokens();

        tokens.peek().unwrap();

        assert_eq!(tokens.next_matches(Kind::Number), Some(true));
        assert_eq!(tokens.next_matches(Kind::Identifier), Some(false));
    }

    #[test]
    fn next_if_match_when_expected_empty() {
        let mut tokens = new_tokens();

        assert_eq!(tokens.next_if_match(&[][..]), None);
    }

    #[test]
    fn next_if_match_when_empty() {
        let mut tokens = Tokens::default();

        assert_eq!(tokens.next_if_match(&[Kind::Number]), None);
    }

    #[test]
    fn next_if_match_with_tokens() {
        let mut tokens = new_tokens();

        assert_eq!(
            tokens.next_if_match(&[Kind::Number]),
            Some(Token::new("1", Kind::Number))
        );
    }

    #[test]
    fn next_if_match_with_tokens_peeked() {
        let mut tokens = new_tokens();

        tokens.peek().unwrap();

        assert_eq!(
            tokens.next_if_match(&[Kind::Number]),
            Some(Token::new("1", Kind::Number))
        );
    }

    #[test]
    fn peek_kind_empty() {
        let tokens = Tokens::from(vec![]);

        assert_eq!(tokens.peek_kind(), None);
    }

    #[test]
    fn peek_kind_with_tokens() {
        let tokens = new_tokens();

        assert_eq!(tokens.peek_kind(), Some(Kind::Number));
    }

    #[test]
    fn peek_kind_then_next() {
        let mut tokens = new_tokens();

        tokens.next().unwrap();
        tokens.peek_kind().unwrap();

        assert_eq!(
            tokens.next(),
            Some(Token::new(Lexeme::new("1", "1+"), Kind::Plus))
        );
    }

    #[test]
    fn peek_kind_multiple() {
        let mut tokens = new_tokens();

        assert_eq!(tokens.peek_kind(), tokens.peek_kind());
        assert_eq!(tokens.next(), Some(Token::new("1", Kind::Number)));
    }

    #[test]
    fn invalid_tokens() {
        let tokens =
            Tokens::try_from(vec![Err(LexicalError::new("|", ErrorKind::Invalid))].into_iter());

        assert_eq!(
            tokens,
            Err(SyntacticalError::Lexical(vec![LexicalError::new(
                "|",
                ErrorKind::Invalid
            )]))
        )
    }

    fn new_tokens() -> Tokens<'static> {
        vec![
            Token::new("1", Kind::Number),
            Token::new(Lexeme::new("1", "1+"), Kind::Plus),
            Token::new(Lexeme::new("1+", "1+1"), Kind::Number),
        ]
        .into()
    }
}
