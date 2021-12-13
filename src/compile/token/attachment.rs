//! Attachments to Lexical Tokens are bits of information computing in the lexical analysis phase.
//! The attachment is passed to the parser to centralize the format of complex token types.

use crate::compile::Kind;
use crate::grammar::Number;

/// An attachment is used to pass parsed information from the lexical analysis to the parser.
/// This avoids having to duplicate the format of tokens in different places.
#[derive(Debug, PartialEq)]
pub enum Attachment {
    Number(Number),
    Empty(Kind),
}

impl From<&Attachment> for Kind {
    fn from(attachment: &Attachment) -> Self {
        match attachment {
            Attachment::Number(..) => Kind::Number,
            Attachment::Empty(kind) => *kind,
        }
    }
}

impl From<Attachment> for Kind {
    fn from(attachment: Attachment) -> Self {
        (&attachment).into()
    }
}

impl From<Kind> for Attachment {
    fn from(kind: Kind) -> Self {
        Attachment::Empty(kind)
    }
}
