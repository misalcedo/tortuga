//! Scans a source code file for lexical tokens.

use crate::errors::ValidationError;
use crate::number::{DECIMAL_RADIX, MAX_RADIX};
use crate::token::{Location, Token, TokenKind};
use std::iter::{Iterator, Peekable};
use std::str::Chars;

/// Scanner for the tortuga language.
/// The scanner can step back in the source code until the character after the last token was emitted.
/// Assumes the source code is written left to write.
pub struct Scanner<'source> {
    code: &'source str,
    location: Location,
    characters: Peekable<Chars<'source>>,
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Scanner<'source> {
        Scanner {
            code,
            location: Location::default(),
            characters: code.chars().peekable(),
        }
    }

    /// Returns the next charater only if it is not a new line.
    fn next_unless_newline(&mut self) -> Option<char> {
        match self.characters.next_if(|c| c != &'\n')? {
            '\r' => None,
            c => Some(c),
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

    /// Reverts the iterator this scanner's location's offset in the source code.
    fn backtrack(&mut self) {
        self.characters = self.code[self.location.offset()..].chars().peekable();
    }

    /// Gets the lexeme starting at this scanner's location (inclusive) until the given end location (exclusive).
    fn get_lexeme(&self, end: Location) -> &'source str {
        &self.code[self.location.offset()..end.offset()]
    }

    fn new_token(
        &mut self,
        kind: TokenKind,
        end: Location,
        validations: Vec<ValidationError>,
    ) -> Token<'source> {
        let start = self.location;
        let lexeme = self.get_lexeme(end);

        self.location = end;

        Token::new(kind, lexeme, start, validations)
    }

    /// Creates a new token for single character lexemes.
    fn new_short_token(&mut self, kind: TokenKind, character: char) -> Option<Token<'source>> {
        Some(self.new_token(kind, self.location.successor(character), Vec::new()))
    }

    /// A text reference is used for internationalization.
    /// The text within quotes is used to lookup a localized string literal during compilation.
    /// Text references may contain any character except double quote and new line.
    /// Also, text references must not be blank (only space or empty).
    fn scan_text_reference(&mut self) -> Token<'source> {
        let mut current = self.location;
        let mut validations = Vec::new();

        current.add_column('"');

        loop {
            match self.next_unless_newline() {
                Some(c @ '"') => {
                    current.add_column(c);
                    break;
                }
                Some(c) => current.add_column(c),
                None => {
                    validations.push(ValidationError::MissingClosingQuote);
                    break;
                }
            }
        }

        let reference = self.get_lexeme(current)[1..].trim();
        if reference.is_empty() || reference == "\"" {
            validations.push(ValidationError::BlankTextReference);
        }

        self.new_token(TokenKind::TextReference, current, validations)
    }

    /// Scans a continous string of digits (e.g. 0-9, a-z, A-Z).
    fn scan_digits(&mut self, radix: u32, current: &mut Location) -> Option<&'source str> {
        let start = *current;

        while let Some(c) = self.characters.next_if(|c| c.is_digit(radix)) {
            current.add_column(c)
        }

        let lexeme = &self.code[start.offset()..current.offset()];

        if lexeme.trim().is_empty() {
            None
        } else {
            Some(lexeme)
        }
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
        let mut current = self.location;
        let mut validations = Vec::new();

        self.backtrack();

        let integer = self.scan_digits(MAX_RADIX, &mut current);

        // Check if we have a fractional part.
        if let Some(c) = self.characters.next_if_eq(&'.') {
            current.add_column(c);

            let fraction = self.scan_digits(MAX_RADIX, &mut current);

            if integer.is_none() && fraction.is_none() {
                validations.push(ValidationError::ExpectedDigits);
            }
        }

        if let Some(c) = self.characters.next_if_eq(&'.') {
            current.add_column(c);
            validations.push(ValidationError::DuplicateDecimal);
        }

        if let Some(c) = self.characters.next_if_eq(&'#') {
            current.add_column(c);

            let radix = self.scan_digits(DECIMAL_RADIX, &mut current);
            if radix.is_none() {
                validations.push(ValidationError::MissingRadix);
            }
        } else if self.get_lexeme(current).starts_with(|c: char| c.is_ascii_alphabetic()) {
            // Treat the token as an identifier if the literal does not have a radix portion, but starts with an ASCII letter.
            self.backtrack();
            return self.scan_identifier();
        }

        self.new_token(TokenKind::Number, current, validations)
    }

    /// Scans an identifier from the source code.
    /// Identifiers must start with an alphabetic character.
    /// The remaining characters must be alphanumeric or an underscore.
    /// Identifiers must not end in an underscore.
    fn scan_identifier(&mut self) -> Token<'source> {
        let mut current = self.location;
        let mut validations = Vec::new();

        self.backtrack();

        while let Some(c) = self
            .characters
            .next_if(|c| c.is_alphanumeric() || c == &'_')
        {
            current.add_column(c);
        }

        let lexeme = self.get_lexeme(current);

        if lexeme.ends_with('_') {
            validations.push(ValidationError::TerminalUnderscore);
        }

        self.new_token(TokenKind::Identifier, current, validations)
    }

    /// The next lexical token in the source code.
    fn next_token(&mut self) -> Option<Token<'source>> {
        loop {
            match self.characters.next()? {
                c @ '~' => return self.new_short_token(TokenKind::Tilde, c),
                c @ '+' => return self.new_short_token(TokenKind::Plus, c),
                c @ '-' => return self.new_short_token(TokenKind::Minus, c),
                c @ '*' => return self.new_short_token(TokenKind::Star, c),
                c @ '/' => return self.new_short_token(TokenKind::ForwardSlash, c),
                c @ '=' => return self.new_short_token(TokenKind::Equals, c),
                c @ '<' => return self.new_short_token(TokenKind::LessThan, c),
                c @ '>' => return self.new_short_token(TokenKind::GreaterThan, c),
                c @ '|' => return self.new_short_token(TokenKind::Pipe, c),
                c @ '^' => return self.new_short_token(TokenKind::Caret, c),
                c @ '%' => return self.new_short_token(TokenKind::Percent, c),
                c @ '_' => return self.new_short_token(TokenKind::Underscore, c),
                c @ ':' => return self.new_short_token(TokenKind::Locale, c),
                c @ '(' => return self.new_short_token(TokenKind::LeftParenthesis, c),
                c @ ')' => return self.new_short_token(TokenKind::RightParenthesis, c),
                c @ '[' => return self.new_short_token(TokenKind::LeftBracket, c),
                c @ ']' => return self.new_short_token(TokenKind::RightBracket, c),
                c @ '{' => return self.new_short_token(TokenKind::LeftBrace, c),
                c @ '}' => return self.new_short_token(TokenKind::RightBrace, c),
                '"' => return Some(self.scan_text_reference()),
                '\r' => (),
                '\n' => self.location.next_line(),
                ';' => self.skip_comment(),
                c @ ('\t' | ' ') => self.location.add_column(c),
                c if c.is_digit(MAX_RADIX) || c == '.' => return Some(self.scan_number()),
                c if c.is_alphabetic() => return Some(self.scan_identifier()),
                c => {
                    return Some(self.new_token(
                        TokenKind::Identifier,
                        self.location.successor(c),
                        vec![ValidationError::UnexpectedCharacter],
                    ))
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
