use crate::grammar::Uri;
use std::fmt::{Display, Formatter};
use tortuga_executable::Text;

impl<'a> From<Uri<'a>> for Text {
    fn from(uri: Uri<'a>) -> Self {
        Text::from(uri.as_str())
    }
}
