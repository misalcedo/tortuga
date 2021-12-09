//! The lexical analyzer of a potentially infinite stream characters
//! that is provided by a source code scanner.
//! The lexer produces lexical tokens.

use crate::errors::LexicalError;
use crate::grammar::Operator;
use crate::number::{Fraction, Number, Sign, DECIMAL_RADIX, MAX_RADIX};
use crate::scanner::Scanner;
use crate::token::{Attachment, Kind, Token};

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

    let lexeme = scanner.lexeme().source();

    if lexeme.is_empty() {
        None
    } else {
        Some(lexeme)
    }
}

/// Scans a `Sign` (positive or negative).
fn scan_sign(scanner: &mut Scanner) -> Option<Sign> {
    match scanner.next_if(|c| c == '+' || c == '-') {
        Some('+') => Some(Sign::Positive),
        Some('-') => Some(Sign::Negative),
        _ => None
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
/// - 16#-FF
fn scan_number<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    let start = *scanner.start();
    let mut errors = Vec::new();
    let mut sign = None;
    let mut radix_lexeme = None;
    let mut integer_lexeme = scan_digits(scanner, DECIMAL_RADIX);
    let mut fraction_lexeme = None;
    
    if scanner.next_if_eq('#').is_some() {
        sign = scan_sign(scanner);
        radix_lexeme = integer_lexeme;
        integer_lexeme = scan_digits(scanner, MAX_RADIX);
    }

    let radix = match radix_lexeme.map(|r| r.parse::<u32>()) {
        Some(Ok(value)) if value > MAX_RADIX => {
            errors.push(LexicalError::RadixTooLarge(value));
            MAX_RADIX
        }
        Some(Err(error)) => {
            errors.push(LexicalError::InvalidRadix(error));
            DECIMAL_RADIX
        }
        Some(Ok(value)) => value,
        None => DECIMAL_RADIX,
    };

    if scanner.next_if_eq('.').is_some() {
        fraction_lexeme = scan_digits(scanner, MAX_RADIX);
    }

    if integer_lexeme.is_none() && fraction_lexeme.is_none() {
        errors.push(LexicalError::ExpectedDigits);
    }

    let integer = match integer_lexeme.map(|i| u128::from_str_radix(i, radix)) {
        Some(Err(error)) => {
            errors.push(LexicalError::InvalidInteger(error));
            0
        }
        Some(Ok(value)) => value,
        None => 0,
    };
    let numerator = match fraction_lexeme.map(|f| u128::from_str_radix(f, radix)) {
        Some(Err(error)) => {
            errors.push(LexicalError::InvalidFraction(error));
            0
        }
        Some(Ok(value)) => value,
        None => 0,
    };

    if scanner.next_if_eq('.').is_some() {
        errors.push(LexicalError::DuplicateDecimal);
    }

    let fraction = match fraction_lexeme{
        Some(value) if value.len() > (u32::MAX as usize) => {
            errors.push(LexicalError::FractionTooLong(value.len()));
            Fraction::default()
        },
        Some(value) => Fraction::new(numerator, radix.pow(value.len() as u32).into()),
        None => Fraction::default()
    };

    let number = Number::new(sign, integer, fraction);

    Token::new(Attachment::Number(number), scanner.lexeme_from(&start), errors)
}

/// Scans either an identifier or a number with a radix.
fn scan_identifier<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    let mut errors = Vec::new();

    while scanner
        .next_if(|c| c.is_alphanumeric() || c == '_')
        .is_some()
    {}

    let lexeme = scanner.lexeme();

    if lexeme.source().ends_with('_') {
        errors.push(LexicalError::TerminalUnderscore);
    }

    Token::new(Attachment::Empty(Kind::Identifier), lexeme, errors)
}

impl<'scanner, 'source> Lexer<'scanner, 'source> {
    /// Creates a new `Lexer` for the given source code.
    pub fn new(scanner: &'scanner mut Scanner<'source>) -> Lexer<'scanner, 'source> {
        Lexer { scanner }
    }

    /// Creates a new token for single character lexemes.
    fn new_short_token<T: Into<Attachment>>(&mut self, attachment: T) -> Option<Token<'source>> {
        self.scanner.next();

        Some(Token::new_valid(
            attachment.into(),
            self.scanner.lexeme()
        ))
    }

    /// The next lexical token in the source code.
    fn next_token(&mut self) -> Option<Token<'source>> {
        match self.scanner.peek()? {
            '+' => self.new_short_token(Operator::Add),
            '-' => self.new_short_token(Operator::Subtract),
            '*' => self.new_short_token(Operator::Multiply),
            '/' => self.new_short_token(Operator::Divide),
            '^' => self.new_short_token(Operator::Exponent),
            '=' => self.new_short_token(Kind::Equals),
            '<' => self.new_short_token(Kind::LessThan),
            '>' => self.new_short_token(Kind::GreaterThan),
            '~' => self.new_short_token(Kind::Tilde),
            '|' => self.new_short_token(Kind::Pipe),
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
                while self.scanner.next_if(|c| !c.is_ascii_punctuation() && !c.is_alphanumeric()).is_some() {}

                Some(Token::new_invalid(
                    None,
                    self.scanner.lexeme(),
                    vec![LexicalError::UnexpectedCharacter],
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
    use crate::token::Lexeme;

    #[test]
    fn lex_number() {
        let mut scanner = "+1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Operator(Operator::Add),
                Lexeme::new("+", Location::default()),
            ))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Number(Number::new(None, 1, Fraction::default())),
                Lexeme::new("1", Location::new(1, 2, 1)),
            ))
        );
    }

    #[test]
    fn lex_binary_number() {
        let mut scanner = "2#-011.01".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Number(Number::new(Some(Sign::Negative), 3, Fraction::new(1, 4))),
                Lexeme::new("2#-011.01", Location::default()),
            ))
        );
    }

    #[test]
    fn lex_hex_number() {
        let mut scanner = "16#FFFFFF".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Number(Number::new_integer(16777215)),
                Lexeme::new("16#FFFFFF", Location::default()),
            ))
        );
    }

    #[test]
    fn lex_empty_number() {
        let mut scanner = ".".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_invalid(
                Some(Kind::Number),
                Lexeme::new(".", Location::default()),
                vec![LexicalError::ExpectedDigits]
            ))
        );
    }

    #[test]
    fn lex_number_trailing_dot() {
        let mut scanner = "1.2.".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_invalid(
                Some(Kind::Number),
                Lexeme::new("1.2.", Location::default()),
                vec![LexicalError::DuplicateDecimal]
            ))
        );
    }

    #[test]
    fn lex_number_radix_too_large() {
        let mut scanner = "256#1.".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_invalid(
                Some(Kind::Number),
                Lexeme::new("256#1.", Location::default()),
                vec![LexicalError::RadixTooLarge(256)]
            ))
        );
    }

    #[test]
    fn lex_number_invalid_radix() {
        let mut scanner = "222222222222222222222222222222222222222222#1.".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_invalid(
                Some(Kind::Number),
                Lexeme::new("222222222222222222222222222222222222222222#1.", Location::default()),
                vec![LexicalError::InvalidRadix(
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
            Some(Token::new_invalid(
                Some(Kind::Number),
                Lexeme::new("10#FF", Location::default()),
                vec![LexicalError::InvalidInteger(
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
            Some(Token::new_invalid(
                Some(Kind::Number),
                Lexeme::new(".FF", Location::default()),
                vec![LexicalError::InvalidFraction(
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
            Some(Token::new_invalid(
                Some(Kind::Identifier),
                Lexeme::new("x_", Location::default()),
                vec![LexicalError::TerminalUnderscore]
            ))
        );
    }

    #[test]
    fn lex_identifier() {
        let mut scanner = "x_1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Kind::Identifier.into(),
                Lexeme::new("x_1", Location::default())
            ))
        );
    }

    #[test]
    fn lex_expression_with_no_space() {
        let mut scanner = "1x".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(Attachment::Number(Number::new_integer(1)), Lexeme::new("1", Location::default())))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Kind::Identifier.into(),
                Lexeme::new("x", Location::new(1, 2, 1)),
            ))
        );
    }

    #[test]
    fn lex_expression_simple() {
        let mut scanner = "x = 1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Kind::Identifier.into(),
                Lexeme::new("x", Location::default()),
            ))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Kind::Equals.into(),
                Lexeme::new("=", Location::new(1, 3, 2)),
            ))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::new(1, 5, 4)),
            ))
        );

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn lex_expression_math() {
        let mut scanner = "1+1".into();
        let mut lexer = Lexer::new(&mut scanner);

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::default()),
            ))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Operator(Operator::Add),
                Lexeme::new("+", Location::new(1, 2, 1)),
            ))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new_valid(
                Attachment::Number(Number::new_integer(1)),
                Lexeme::new("1", Location::new(1, 3, 2)),
            ))
        );

        assert_eq!(lexer.next(), None);
    }
}
