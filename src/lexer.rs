//! The lexical anaylzer of a potentially infinite stream characters
//! that is provided by a source code scanner.
//! The lexer produces lexical tokens.

use crate::errors::ValidationError;
use crate::number::{Sign, DECIMAL_RADIX, MAX_RADIX};
use crate::scanner::Scanner;
use crate::token::{Kind, Token};

/// Performs Lexical Analysis for the tortuga language.
pub struct Lexer<'scanner, 'source>
where
    'source: 'scanner,
{
    scanner: &'scanner mut Scanner<'source>,
}

/// Scans for digits in the given radix.
/// Skips any prefixing tokens scanned prior to the digits.
fn scan_digits<'source>(scanner: &mut Scanner<'source>, radix: u32) -> Option<&'source str> {
    scanner.step_forward();

    while scanner.next_if(|c| c.is_digit(radix)).is_some() {}

    let lexeme = scanner.lexeme();

    if lexeme.is_empty() {
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
/// - 2#0.
/// - 16#-FFFFFF
fn scan_number<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    let start = *scanner.start();
    let mut validations = Vec::new();
    let mut radix = None;
    let mut _sign = Sign::Positive;
    let mut integer = scan_digits(scanner, DECIMAL_RADIX);
    let mut fraction = None;

    if scanner.next_if_eq('#').is_some() {
        radix = integer;
        _sign = match scanner.next_if(|c| c == '+' || c == '-') {
            Some('-') => Sign::Negative,
            _ => Sign::Positive,
        };

        integer = scan_digits(scanner, MAX_RADIX);
    }

    let radix = match radix.map(|r| r.parse::<u32>()) {
        Some(Ok(value)) if value > MAX_RADIX => {
            validations.push(ValidationError::RadixTooLarge(value));
            MAX_RADIX
        }
        Some(Err(error)) => {
            validations.push(ValidationError::InvalidRadix(error));
            DECIMAL_RADIX
        }
        Some(Ok(value)) => value,
        None => DECIMAL_RADIX,
    };

    if scanner.next_if_eq('.').is_some() {
        fraction = scan_digits(scanner, MAX_RADIX);
    }

    if integer.is_none() && fraction.is_none() {
        validations.push(ValidationError::ExpectedDigits);
    }

    let _integer = match integer.map(|i| u128::from_str_radix(i, radix)) {
        Some(Err(error)) => {
            validations.push(ValidationError::InvalidInteger(error));
            0
        }
        Some(Ok(value)) => value,
        None => 0,
    };
    let _fraction = match fraction.map(|f| u128::from_str_radix(f, radix)) {
        Some(Err(error)) => {
            validations.push(ValidationError::InvalidFraction(error));
            0
        }
        Some(Ok(value)) => value,
        None => 0,
    };

    if scanner.next_if_eq('.').is_some() {
        validations.push(ValidationError::DuplicateDecimal);
    }

    Token::new(
        Kind::Number,
        start,
        scanner.lexeme_from(&start),
        validations,
    )
}

/// Scans either an identifier or a number with a radix.
fn scan_identifier<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    let mut validations = Vec::new();

    while scanner
        .next_if(|c| c.is_alphanumeric() || c == '_')
        .is_some()
    {}

    let start = *scanner.start();
    let lexeme = scanner.lexeme();

    if lexeme.ends_with('_') {
        validations.push(ValidationError::TerminalUnderscore);
    }

    Token::new(Kind::Identifier, start, lexeme, validations)
}

impl<'scanner, 'source> Lexer<'scanner, 'source> {
    /// Creates a new `Lexer` for the given source code.
    pub fn new(scanner: &'scanner mut Scanner<'source>) -> Lexer<'scanner, 'source> {
        Lexer { scanner }
    }

    /// Creates a new token for single character lexemes.
    fn new_short_token(&mut self, kind: Kind) -> Option<Token<'source>> {
        self.scanner.next();
        Some(Token::new(
            kind,
            *self.scanner.start(),
            self.scanner.lexeme(),
            Vec::new(),
        ))
    }

    /// The next lexical token in the source code.
    fn next_token(&mut self) -> Option<Token<'source>> {
        match self.scanner.peek()? {
            '~' => self.new_short_token(Kind::Tilde),
            '+' => self.new_short_token(Kind::Plus),
            '-' => self.new_short_token(Kind::Minus),
            '*' => self.new_short_token(Kind::Star),
            '/' => self.new_short_token(Kind::ForwardSlash),
            '=' => self.new_short_token(Kind::Equals),
            '<' => self.new_short_token(Kind::LessThan),
            '>' => self.new_short_token(Kind::GreaterThan),
            '|' => self.new_short_token(Kind::Pipe),
            '^' => self.new_short_token(Kind::Caret),
            '%' => self.new_short_token(Kind::Percent),
            '_' => self.new_short_token(Kind::Underscore),
            '(' => self.new_short_token(Kind::LeftParenthesis),
            ')' => self.new_short_token(Kind::RightParenthesis),
            '[' => self.new_short_token(Kind::LeftBracket),
            ']' => self.new_short_token(Kind::RightBracket),
            '{' => self.new_short_token(Kind::LeftBrace),
            '}' => self.new_short_token(Kind::RightBrace),
            c if c.is_alphabetic() => Some(scan_identifier(self.scanner)),
            c if c.is_ascii_digit() || c == '.' => Some(scan_number(&mut self.scanner)),
            _ => {
                self.scanner.next();

                Some(Token::new(
                    Kind::Identifier,
                    *self.scanner.start(),
                    self.scanner.lexeme(),
                    vec![ValidationError::UnexpectedCharacter],
                ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::location::Location;

    #[test]
    fn lex_number() {
        let mut scanner = "1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "1",
                Vec::new()
            ))
        );
    }

    #[test]
    fn lex_binary_number() {
        let mut scanner = "2#-011.01".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "2#-011.01",
                Vec::new()
            ))
        );
    }

    #[test]
    fn lex_hex_number() {
        let mut scanner = "16#FFFFFF".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "16#FFFFFF",
                Vec::new()
            ))
        );
    }

    #[test]
    fn lex_empty_number() {
        let mut scanner = ".".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                ".",
                vec![ValidationError::ExpectedDigits]
            ))
        );
    }

    #[test]
    fn lex_number_trailing_dot() {
        let mut scanner = "1.2.".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "1.2.",
                vec![ValidationError::DuplicateDecimal]
            ))
        );
    }

    #[test]
    fn lex_number_radix_too_large() {
        let mut scanner = "256#1.".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "256#1.",
                vec![ValidationError::RadixTooLarge(256)]
            ))
        );
    }

    #[test]
    fn lex_number_invalid_radix() {
        let mut scanner = "222222222222222222222222222222222222222222#1.".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "222222222222222222222222222222222222222222#1.",
                vec![ValidationError::InvalidRadix(
                    "222222222222222222222222222222222222222222"
                        .parse::<u32>()
                        .unwrap_err()
                )]
            ))
        );
    }

    #[test]
    fn lex_number_invalid_integer() {
        let mut scanner = "10#FF".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                "10#FF",
                vec![ValidationError::InvalidInteger(
                    u128::from_str_radix("FF", DECIMAL_RADIX).unwrap_err()
                )]
            ))
        );
    }

    #[test]
    fn lex_number_invalid_fraction() {
        let mut scanner = ".FF".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::default(),
                ".FF",
                vec![ValidationError::InvalidFraction(
                    u128::from_str_radix("FF", DECIMAL_RADIX).unwrap_err()
                )]
            ))
        );
    }

    #[test]
    fn lex_invalid_identifier() {
        let mut scanner = "x_".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Identifier,
                Location::default(),
                "x_",
                vec![ValidationError::TerminalUnderscore]
            ))
        );
    }

    #[test]
    fn lex_identifier() {
        let mut scanner = "x_1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Identifier,
                Location::default(),
                "x_1",
                vec![]
            ))
        );
    }

    #[test]
    fn lex_expression_with_no_space() {
        let mut scanner = "1x".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(Kind::Number, Location::default(), "1", vec![]))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Identifier,
                Location::new(1, 2, 1),
                "x",
                vec![]
            ))
        );
    }

    #[test]
    fn lex_expression_simple() {
        let mut scanner = "x = 1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Identifier,
                Location::default(),
                "x",
                vec![]
            ))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Equals,
                Location::new(1, 3, 2),
                "=",
                vec![]
            ))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                Kind::Number,
                Location::new(1, 5, 4),
                "1",
                vec![]
            ))
        );

        assert_eq!(lexer.next(), None);
    }
}
