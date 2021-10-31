//! Scans a source code file for lexical tokens.

use crate::errors::LexicalError;
use crate::token::{Location, Token, TokenKind};
use std::iter::Iterator;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

/// Scanner for the tortuga language.
/// Uses a grapheme cursor to allow for arbitrary lookahead and lookback.
pub struct Scanner<'source> {
    code: &'source str,
    location: Location,
    cursor: GraphemeCursor,
}

/// Creates a new token and updates the location past the given lexeme.
fn new_token<'source>(kind: TokenKind, lexeme: &'source str, location: &mut Location) -> Token<'source> {
    let start = location.clone();
    
    location.add_columns(lexeme.graphemes(true).count());

    Token::new(kind, lexeme, start)
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Scanner<'source> {
        Scanner {
            code,
            location: Location::default(),
            cursor: GraphemeCursor::new(0, code.len(), true),
        }
    }

    /// Skips comments until the end of the current line.
    /// The location is not updated as all comments end in either a new line or EOF (end of file).
    fn skip_comment(&mut self) -> Result<(), LexicalError>
    {
        while let Some(grapheme) = self.next_grapheme(1)? {
            if grapheme == "\n" {
                self.step_back()?;
                break;
            }
        }

        Ok(())
    }

    /// Revert the cursor location to the previous grapheme boundary.
    fn step_back(&mut self) -> Result<(), LexicalError> {
        self.cursor.prev_boundary(self.code, 0)?;
        Ok(())
    }

    /// The next grapheme in the source code.
    fn next_grapheme(&mut self, lookahead: usize) -> Result<Option<&'source str>, LexicalError> {
        let start = self.cursor.cur_cursor();

        for _ in 0..lookahead {
            self.cursor.next_boundary(self.code, 0)?;
        }

        let end = self.cursor.cur_cursor();

        if start == end {
            return Ok(None);
        }

        let remaining = &self.code[start..end];

        Ok(Some(remaining))
    }

    /// Scans a number literal.
    /// Numbers are decimal digits with an optional decimal part.
    /// 
    /// Examples:
    /// - 0.25
    /// - .25
    /// - 1.25
    /// - 0
    fn scan_number(&mut self) -> Result<Token<'source>, LexicalError>
    {
        self.step_back()?;

        let start = self.location.clone();
        let start_index = self.cursor.cur_cursor();
        let mut has_fractional = false;

        loop {
            match self.next_grapheme(1)? {
                Some("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") => self.location.add_columns(1),
                Some(".") if has_fractional => { 
                    self.location.add_columns(1);
                    let lexeme = self.code[start_index..self.cursor.cur_cursor()].to_string();
                    Err(LexicalError::DuplicateDecimal(self.location.clone(), lexeme))?;
                },
                Some(".") => {
                    has_fractional = true;
                    self.location.add_columns(1);
                },
                Some(_) => {
                    self.step_back()?;
                    break
                }
                None => break,
            } 
        }

        let end_index = self.cursor.cur_cursor();
        let lexeme = &self.code[start_index..end_index];

        if lexeme.ends_with(".") {
            Err(LexicalError::TerminalDecimal(self.location.clone(), lexeme.to_string()))
        } else {
            Ok(Token::new(TokenKind::Number, lexeme, start))
        }
    }

    /// The next lexical token in the source code.
    fn next_token(&mut self) -> Result<Option<Token<'source>>, LexicalError> {
        let mut token = None;

        loop {
            match self.next_grapheme(1)? {
                Some(grapheme @ "~") => { token.insert(new_token(TokenKind::Tilde, grapheme, &mut self.location)); },
                Some(grapheme @ "%") => { token.insert(new_token(TokenKind::Percent, grapheme, &mut self.location)); },
                Some(grapheme @ "^") => { token.insert(new_token(TokenKind::Caret, grapheme, &mut self.location)); },
                Some(grapheme @ "*") => { token.insert(new_token(TokenKind::Star, grapheme, &mut self.location)); },
                Some(grapheme @ "-") => { token.insert(new_token(TokenKind::Minus, grapheme, &mut self.location)); },
                Some(grapheme @ "=") => { token.insert(new_token(TokenKind::Equals, grapheme, &mut self.location)); },
                Some(grapheme @ "+") => { token.insert(new_token(TokenKind::Plus, grapheme, &mut self.location)); },
                Some(grapheme @ "(") => { token.insert(new_token(TokenKind::LeftParenthesis, grapheme, &mut self.location)); },
                Some(grapheme @ ")") => { token.insert(new_token(TokenKind::RightParenthesis, grapheme, &mut self.location)); },
                Some(grapheme @ "[") => { token.insert(new_token(TokenKind::LeftBracket, grapheme, &mut self.location)); },
                Some(grapheme @ "]") => { token.insert(new_token(TokenKind::RightBracket, grapheme, &mut self.location)); },
                Some(grapheme @ "{") => { token.insert(new_token(TokenKind::LeftBrace, grapheme, &mut self.location)); },
                Some(grapheme @ "}") => { token.insert(new_token(TokenKind::RightBrace, grapheme, &mut self.location)); },
                Some(grapheme @ "|") => { token.insert(new_token(TokenKind::Pipe, grapheme, &mut self.location)); },
                Some(grapheme @ "/") => { token.insert(new_token(TokenKind::ForwardSlash, grapheme, &mut self.location)); },
                Some(grapheme @ "<") => { token.insert(new_token(TokenKind::LessThan, grapheme, &mut self.location)); },
                Some(grapheme @ ">") => { token.insert(new_token(TokenKind::GreaterThan, grapheme, &mut self.location)); },
                Some("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | ".") => { token.insert(self.scan_number()?); },
                Some("\t") => self.location.add_columns(4),
                Some(" ") => self.location.add_columns(1),
                Some("\r") => (),
                Some("\n") => self.location.next_line(),
                Some(";") => self.skip_comment()?,
                Some(grapheme) => {
                    let start = self.location.clone();
                    self.location.add_columns(1);
                    Err(LexicalError::UnexpectedGrapheme(start, grapheme.to_string()))?
                },
                None => break
            }

            if token.is_some() {
                break
            }
        }

        Ok(token)
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source> Iterator for Scanner<'source>
{
    // We can refer to this type using Self::Item
    type Item = Result<Token<'source>, LexicalError>;

    // Consumes the next token from the `Scanner`.
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(error) => Some(Err(error)),
        }
    }
}
