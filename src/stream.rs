//! Transforms an `Iterator<Item=Token<'_>>` into a stream of tokens.
//! The stream can be peeked and conditionally polled.
//! Also, transforms the infallible iterator into a fallible one.
//! That allows the parser to treat `InvalidToken`s as errors.

use crate::errors::SyntaxError;
use crate::token::{Kind, Token, ValidToken};

/// A stream of `Token`s to be consumed by the `Parser`.
#[derive(Debug)]
pub struct TokenStream<'source, I: Iterator<Item = Token<'source>>> {
    tokens: I,
    peeked: Option<ValidToken<'source>>,
}

impl<'source, I, II> From<II> for TokenStream<'source, I>
where
    I: Iterator<Item = Token<'source>>,
    II: IntoIterator<IntoIter = I, Item = Token<'source>>,
{
    fn from(tokens: II) -> Self {
        TokenStream {
            tokens: tokens.into_iter(),
            peeked: None,
        }
    }
}

impl<'source, I: Iterator<Item = Token<'source>>> TokenStream<'source, I> {
    pub fn peek(&mut self) -> Result<Option<&ValidToken<'source>>, SyntaxError<'source>> {
        self.peeked = match self.peeked.take() {
            Some(token) => Some(token),
            None => self.next()?,
        };

        Ok(self.peeked.as_ref())
    }

    /// Consumes the next token in the `Iterator` and returns the `Token`.
    /// If the stream was peeked since the last call to `next`, the peeked value is returned instead.
    pub fn next(&mut self) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        match self.peeked.take() {
            Some(token) => Ok(Some(token)),
            None => match self.tokens.next() {
                Some(Token::Valid(token)) => Ok(Some(token)),
                Some(Token::Invalid(token)) => Err(SyntaxError::InvalidToken(token)),
                None => Ok(None),
            },
        }
    }

    /// Gets the next `Token` if it matches the expected kind. Otherwise, returns an error.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn next_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<ValidToken<'source>, SyntaxError<'source>> {
        let token = self
            .next()?
            .ok_or_else(|| SyntaxError::IncompleteRule(expected.to_vec()))?;

        if expected.contains(&token.kind()) {
            Ok(token)
        } else {
            Err(SyntaxError::NoMatchingRule(token, expected.to_vec()))
        }
    }

    /// Gets the next `Token` only if it has the given kind. Otherwise, returns an empty `Option`.
    /// Returns an error if an `InvaliToken` is next in the stream.
    /// Returns an empty `Option` if the stream is empty.
    pub fn next_if_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        match self.next()? {
            Some(token) => {
                if expected.contains(&token.kind()) {
                    Ok(Some(token))
                } else {
                    self.peeked.insert(token);
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Peeks the next `Token`'s `Kind`.
    /// Returns an error if there are no more `Token`s in the stream.
    pub fn peek_kind(&mut self) -> Result<Kind, SyntaxError<'source>> {
        match self.peek()? {
            Some(token) => Ok(token.kind()),
            None => Err(SyntaxError::IncompleteRule(Vec::new())),
        }
    }

    /// Tests if the next token's kind matches one of the expected ones.
    pub fn next_matches_kind(&mut self, expected: &[Kind]) -> bool {
        match self.peek() {
            Ok(Some(token)) => expected.contains(&token.kind()),
            _ => false,
        }
    }

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    pub fn is_empty(&mut self) -> bool {
        match self.peek() {
            Ok(None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::LexicalError;
    use crate::location::Location;
    use crate::number::Number;
    use crate::token::{Attachment, InvalidToken, Lexeme};

    #[test]
    fn next_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert_eq!(stream.next(), Ok(None));
    }

    #[test]
    fn next_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(
            stream.next(),
            Ok(Some(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            )))
        );
    }

    #[test]
    fn is_empty_when_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert!(stream.is_empty());
    }

    #[test]
    fn is_empty_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert!(!stream.is_empty());
    }

    #[test]
    fn next_matches_kind_when_expected_empty() {
        let mut stream = TokenStream::from(new_tokens());

        assert!(!stream.next_matches_kind(&[]));
    }

    #[test]
    fn next_matches_kind_when_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert!(!stream.next_matches_kind(&[Kind::Number]));
    }

    #[test]
    fn next_matches_kind_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert!(stream.next_matches_kind(&[Kind::Number]));
    }

    #[test]
    fn next_matches_kind_with_tokens_peeked() {
        let mut stream = TokenStream::from(new_tokens());

        stream.peek().unwrap();

        assert!(stream.next_matches_kind(&[Kind::Number]));
    }

    #[test]
    fn next_if_kind_when_expected_empty() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(stream.next_if_kind(&[]), Ok(None));
    }

    #[test]
    fn next_if_kind_when_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert_eq!(stream.next_if_kind(&[Kind::Number]), Ok(None));
    }

    #[test]
    fn next_if_kind_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(
            stream.next_if_kind(&[Kind::Number]),
            Ok(Some(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            )))
        );
    }

    #[test]
    fn next_if_kind_with_tokens_peeked() {
        let mut stream = TokenStream::from(new_tokens());

        stream.peek().unwrap();

        assert_eq!(
            stream.next_if_kind(&[Kind::Number]),
            Ok(Some(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            )))
        );
    }

    #[test]
    fn next_kind_when_expected_empty() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(
            stream.next_kind(&[]),
            Err(SyntaxError::NoMatchingRule(
                ValidToken::new(
                    Attachment::Number(Number::new_integer(1)),
                    Lexeme::new("1", Location::default())
                ),
                vec![]
            ))
        );
    }

    #[test]
    fn next_kind_when_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert_eq!(
            stream.next_kind(&[]),
            Err(SyntaxError::IncompleteRule(vec![]))
        );
    }

    #[test]
    fn next_kind_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(
            stream.next_kind(&[Kind::Number]),
            Ok(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            ))
        );
    }

    #[test]
    fn next_kind_with_tokens_peeked() {
        let mut stream = TokenStream::from(new_tokens());

        stream.peek().unwrap();

        assert_eq!(
            stream.next_kind(&[Kind::Number]),
            Ok(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            ))
        );
    }

    #[test]
    fn peek_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert_eq!(stream.peek(), Ok(None));
    }

    #[test]
    fn peek_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(
            stream.peek(),
            Ok(Some(&ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            )))
        );
    }

    #[test]
    fn peek_then_next() {
        let mut stream = TokenStream::from(new_tokens());

        stream.next().unwrap();
        stream.peek().unwrap();

        assert_eq!(
            stream.next(),
            Ok(Some(ValidToken::new(
                Kind::Plus.into(),
                Lexeme::new("+", Location::new(1, 2, 1))
            )))
        );
    }

    #[test]
    fn peek_multiple() {
        let mut stream = TokenStream::from(new_tokens());

        stream.peek().unwrap();
        stream.peek().unwrap();

        assert_eq!(
            stream.next(),
            Ok(Some(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            )))
        );
    }

    #[test]
    fn peek_kind_empty() {
        let mut stream = TokenStream::from(vec![]);

        assert_eq!(
            stream.peek_kind(),
            Err(SyntaxError::IncompleteRule(Vec::new()))
        );
    }

    #[test]
    fn peek_kind_with_tokens() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(stream.peek_kind(), Ok(Kind::Number));
    }

    #[test]
    fn peek_kind_then_next() {
        let mut stream = TokenStream::from(new_tokens());

        stream.next().unwrap();
        stream.peek_kind().unwrap();

        assert_eq!(
            stream.next(),
            Ok(Some(ValidToken::new(
                Kind::Plus.into(),
                Lexeme::new("+", Location::new(1, 2, 1))
            )))
        );
    }

    #[test]
    fn peek_kind_multiple() {
        let mut stream = TokenStream::from(new_tokens());

        assert_eq!(stream.peek_kind(), stream.peek_kind());

        assert_eq!(
            stream.next(),
            Ok(Some(ValidToken::new(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default())
            )))
        );
    }

    #[test]
    fn next_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(
            stream.next(),
            Err(SyntaxError::InvalidToken(InvalidToken::new(
                Some(Kind::Number),
                Lexeme::new("1", Location::default()),
                vec![LexicalError::DuplicateDecimal]
            )))
        )
    }

    #[test]
    fn peek_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(
            stream.peek(),
            Err(SyntaxError::InvalidToken(InvalidToken::new(
                Some(Kind::Number),
                Lexeme::new("1", Location::default()),
                vec![LexicalError::DuplicateDecimal]
            )))
        )
    }

    #[test]
    fn next_if_kind_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(
            stream.next_if_kind(&[]),
            Err(SyntaxError::InvalidToken(InvalidToken::new(
                Some(Kind::Number),
                Lexeme::new("1", Location::default()),
                vec![LexicalError::DuplicateDecimal]
            )))
        )
    }

    #[test]
    fn next_kind_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(
            stream.next_kind(&[]),
            Err(SyntaxError::InvalidToken(InvalidToken::new(
                Some(Kind::Number),
                Lexeme::new("1", Location::default()),
                vec![LexicalError::DuplicateDecimal]
            )))
        )
    }

    #[test]
    fn peek_kind_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(
            stream.peek_kind(),
            Err(SyntaxError::InvalidToken(InvalidToken::new(
                Some(Kind::Number),
                Lexeme::new("1", Location::default()),
                vec![LexicalError::DuplicateDecimal]
            )))
        )
    }

    #[test]
    fn next_matches_kind_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(stream.next_matches_kind(&[]), false)
    }

    #[test]
    fn is_empty_invalid() {
        let mut stream = TokenStream::from(vec![Token::new_invalid(
            Some(Kind::Number),
            Lexeme::new("1", Location::default()),
            vec![LexicalError::DuplicateDecimal],
        )]);

        assert_eq!(stream.is_empty(), false)
    }

    fn new_tokens() -> Vec<Token<'static>> {
        vec![
            Token::new_valid(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default()),
            ),
            Token::new_valid(Kind::Plus.into(), Lexeme::new("+", Location::new(1, 2, 1))),
            Token::new_valid(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::new(1, 3, 2)),
            ),
        ]
    }
}
