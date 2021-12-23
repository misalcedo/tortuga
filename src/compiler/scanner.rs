use crate::compiler::input::Input;

pub struct Scanner<'a> {
    input: Input<'a>
}

impl<'a, I: Into<Input<'a>>> From<I> for Scanner<'a> {
    fn from(input: I) -> Self {
        Scanner {
            input: input.into()
        }
    }
}

impl<'a> Scanner<'a> {
    fn scan_identifier(&mut self) {
    }

    fn scan_number(&mut self) {
    }
}