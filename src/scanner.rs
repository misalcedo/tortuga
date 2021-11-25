//! Scans a source code file for lexical tokens.

use crate::errors::ValidationError;
use crate::number::{DECIMAL_RADIX, MAX_RADIX};
use crate::token::{Location, Token, TokenKind};
use std::str::Chars;

/// Scanner for the tortuga language.
/// The scanner can step back in the source code until the character after the last token was emitted.
/// Assumes the source code is written left to write.
pub struct Scanner<'source> {
    code: &'source str,
    location: Location,
    remaining: Chars<'source>,
}

/// Scans for digits.
fn scan_digits(source: &str, radix: u32) -> (Option<&str>, &str) {
    let mut offset = 0;
    let mut iterator = source.chars().peekable();

    while iterator.next_if(|c| c.is_digit(radix)).is_some() {
        offset += 1;
    }

    (
        Some(&source[..offset]).filter(|d| !d.is_empty()),
        &source[offset..],
    )
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Scanner<'source> {
        Scanner {
            code,
            location: Location::default(),
            remaining: code.chars(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.remaining.as_str().chars().next()
    }

    /// Returns the next charater only if it is not a new line.
    fn next_unless_newline(&mut self) -> Option<char> {
        let checkpoint = self.remaining.as_str();

        match self.remaining.next()? {
            '\r' | '\n' => {
                self.remaining = checkpoint.chars();
                None
            }
            c => Some(c),
        }
    }

    /// Returns the next character only if the equals the expected value.
    fn next_if_eq(&mut self, expected: char) -> Option<char> {
        let checkpoint = self.remaining.as_str();
        let character = self.remaining.next()?;

        if character == expected {
            Some(character)
        } else {
            self.remaining = checkpoint.chars();
            None
        }
    }

    /// Skips comments until the end of the current line.
    fn skip_comment(&mut self) {
        let mut current = self.location;

        while let Some(c) = self.next_unless_newline() {
            current.add_column(c);
        }

        self.location = current;
    }

    /// Gets the lexeme starting at this scanner's location (inclusive) until the given end location (exclusive).
    fn get_lexeme(&self) -> &'source str {
        let start = self.location.offset();
        let end = self.code.len() - self.remaining.as_str().len();

        &self.code[start..end]
    }

    fn new_token(&mut self, kind: TokenKind, validations: Vec<ValidationError>) -> Token<'source> {
        let start = self.location;
        let lexeme = self.get_lexeme();

        self.location.add_columns(lexeme);

        Token::new(kind, lexeme, start, validations)
    }

    /// Creates a new token for single character lexemes.
    fn new_short_token(&mut self, kind: TokenKind) -> Option<Token<'source>> {
        self.remaining.next();
        Some(self.new_token(kind, Vec::new()))
    }

    /// Scans a number literal.
    /// Numbers are decimal digits with an optional radix part.
    ///
    /// Examples:
    /// - 0.25
    /// - .25
    /// - 1.25
    /// - 0
    /// - 0.#2
    fn scan_number(&mut self) -> Token<'source> {
        let mut validations = Vec::new();

        let integer = scan_digits(self.remaining.as_str(), MAX_RADIX);

        self.remaining = integer.1.chars();

        // Check if we have a fractional part.
        if self.next_if_eq('.').is_some() {
            let fraction = scan_digits(self.remaining.as_str(), MAX_RADIX);

            self.remaining = fraction.1.chars();

            if integer.0.is_none() && fraction.0.is_none() {
                validations.push(ValidationError::ExpectedDigits);
            }
        }

        if self.next_if_eq('.').is_some() {
            validations.push(ValidationError::DuplicateDecimal);
        }

        if self.next_if_eq('#').is_some() {
            let radix = scan_digits(self.remaining.as_str(), DECIMAL_RADIX);

            self.remaining = radix.1.chars();

            if radix.0.is_none() {
                validations.push(ValidationError::MissingRadix);
            }
        }

        self.new_token(TokenKind::Number, validations)
    }

    /// The next lexical token in the source code.
    fn next_token(&mut self) -> Option<Token<'source>> {
        loop {
            match self.peek()? {
                '~' => return self.new_short_token(TokenKind::Tilde),
                '+' => return self.new_short_token(TokenKind::Plus),
                '-' => return self.new_short_token(TokenKind::Minus),
                '*' => return self.new_short_token(TokenKind::Star),
                '/' => return self.new_short_token(TokenKind::ForwardSlash),
                '=' => return self.new_short_token(TokenKind::Equals),
                '<' => return self.new_short_token(TokenKind::LessThan),
                '>' => return self.new_short_token(TokenKind::GreaterThan),
                '|' => return self.new_short_token(TokenKind::Pipe),
                '^' => return self.new_short_token(TokenKind::Caret),
                '%' => return self.new_short_token(TokenKind::Percent),
                '_' => return self.new_short_token(TokenKind::Underscore),
                ':' => return self.new_short_token(TokenKind::Locale),
                '(' => return self.new_short_token(TokenKind::LeftParenthesis),
                ')' => return self.new_short_token(TokenKind::RightParenthesis),
                '[' => return self.new_short_token(TokenKind::LeftBracket),
                ']' => return self.new_short_token(TokenKind::RightBracket),
                '{' => return self.new_short_token(TokenKind::LeftBrace),
                '}' => return self.new_short_token(TokenKind::RightBrace),
                '\r' => {
                    self.remaining.next();
                }
                '\n' => {
                    self.remaining.next();
                    self.location.next_line()
                }
                ';' => self.skip_comment(),
                c @ ('\t' | ' ') => {
                    self.remaining.next();
                    self.location.add_column(c)
                }
                c if c.is_digit(MAX_RADIX) || c == '.' => return Some(self.scan_number()),
                _ => {
                    self.remaining.next();
                    return Some(self.new_token(
                        TokenKind::Identifier,
                        vec![ValidationError::UnexpectedCharacter],
                    ));
                }
            }
        }
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source> Iterator for Scanner<'source> {
    // We can refer to this type using Self::Item
    type Item = Token<'source>;

    // Consumes the next token from the `Scanner`.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
