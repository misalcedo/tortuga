use std::str::Chars;

pub struct Input<'a> {
    source: &'a str,
    characters: Chars<'a>,
    offset: usize,
    line: usize,
    column: usize,
    peeked: Option<char>,
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(source: &'a str) -> Self {
        Input {
            source,
            characters: source.chars(),
            offset: 0,
            line: 1,
            column: 1,
            peeked: None
        }
    }
}

impl<'a> Input<'a> {
    pub fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.characters.next();
        }

        self.peeked
    }
}

impl Iterator for Input<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.peeked.take().or_else(|| self.characters.next())?;

        self.offset += c.len_utf8();

        match c {
            '\n' => {
                self.line += 1;
                self.column = 1;
            },
            _ => self.column += 1
        }

        Some(c)
    }
}
