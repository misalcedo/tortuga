//! The lexical anaylzer of a potentially infinite stream characters
//! that is provided by a source code scanner.
//! The lexer produces lexical tokens.

use crate::errors::ValidationError;
use crate::number::{DECIMAL_RADIX, MAX_RADIX};
use crate::token::{Token, TokenKind};
use crate::scanner::Scanner;

/// Performs Lexical Analysis for the tortuga language.
pub struct Lexer<'scanner, 'source> where 'source: 'scanner {
    scanner: &'scanner mut Scanner<'source>,
}

/// Scans for digits in the given radix.
fn scan_digits<'source>(scanner: &mut Scanner<'source>, radix: u32) -> Option<&'source str> {
    while scanner.next_if(|c| c.is_digit(radix)).is_some() {
    }

    let lexeme = scanner.lexeme();

    if lexeme.is_empty() {
        None
    } else {
        Some(lexeme)
    }
}

impl<'scanner, 'source> Lexer<'scanner, 'source> {
    /// Creates a new `Lexer` for the given source code.
    pub fn new(scanner: &'scanner mut Scanner<'source>) -> Lexer<'scanner, 'source> {
        Lexer {
            scanner
        }
    }

    fn new_token(&mut self, kind: TokenKind, validations: Vec<ValidationError>) -> Token<'source> {
        Token::new(kind, *self.scanner.start(), self.scanner.lexeme(), validations)
    }

    /// Creates a new token for single character lexemes.
    fn new_short_token(&mut self, kind: TokenKind) -> Option<Token<'source>> {
        self.scanner.next();
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
        let mut radix = None;
        let mut integer = scan_digits(self.scanner, DECIMAL_RADIX);
        let mut fraction = None;

        if self.scanner.next_if_eq('#').is_some() {
            radix = integer;
            integer = scan_digits(self.scanner, MAX_RADIX);
        }

        let radix = match radix.map(|r| r.parse::<u32>()) {
            Some(Ok(value)) if value > MAX_RADIX => {
                validations.push(ValidationError::RadixTooLarge(value));
                MAX_RADIX
            },
            Some(Err(error)) => {
                validations.push(ValidationError::InvalidRadix(error));
                DECIMAL_RADIX
            },
            Some(Ok(value)) => value,
            None => DECIMAL_RADIX
        };


        if self.scanner.next_if_eq('.').is_some() {
            fraction = scan_digits(self.scanner, MAX_RADIX);
        }

        if integer.is_none() && fraction.is_none() {
            validations.push(ValidationError::ExpectedDigits);
        }

        let _integer = match integer.map(|i| u128::from_str_radix(i, radix)) {
            Some(Err(error)) => {
                validations.push(ValidationError::InvalidInteger(error));
                0
            },
            Some(Ok(value)) => value,
            None => 0
        };
        let _fraction = match fraction.map(|f| u128::from_str_radix(f, radix)) {
            Some(Err(error)) => {
                validations.push(ValidationError::InvalidFraction(error));
                0
            },
            Some(Ok(value)) => value,
            None => 0
        };

        if self.scanner.next_if_eq('.').is_some() {
            validations.push(ValidationError::DuplicateDecimal);
        }

        self.new_token(TokenKind::Number, validations)
    }

    /// Scans either an identifier or a number with a radix.
    fn scan_identifier(&mut self) -> Token<'source> {
        let mut validations = Vec::new();

        while self.scanner
            .next_if(|c| c.is_ascii_alphanumeric() || c == '_')
            .is_some()
        {}

        if self.scanner.lexeme().ends_with('_') {
            validations.push(ValidationError::TerminalUnderscore);
        }

        self.new_token(TokenKind::Identifier, validations)
    }

    /// The next lexical token in the source code.
    fn next_token(&mut self) -> Option<Token<'source>> {
        loop {
            match self.scanner.peek()? {
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
                '(' => return self.new_short_token(TokenKind::LeftParenthesis),
                ')' => return self.new_short_token(TokenKind::RightParenthesis),
                '[' => return self.new_short_token(TokenKind::LeftBracket),
                ']' => return self.new_short_token(TokenKind::RightBracket),
                '{' => return self.new_short_token(TokenKind::LeftBrace),
                '}' => return self.new_short_token(TokenKind::RightBrace),
                c if c.is_alphabetic() => return Some(self.scan_identifier()),
                c if c.is_ascii_digit() || c == '.' => return Some(self.scan_number()),
                _ => {
                    self.scanner.next();

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
impl<'scanner, 'source> Iterator for Lexer<'scanner, 'source> {
    // We can refer to this type using Self::Item
    type Item = Token<'source>;

    // Consumes the next token from the `Scanner`.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
