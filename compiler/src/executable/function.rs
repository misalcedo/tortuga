use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Function {
    parameters: usize,
    locals: usize,
    code: Rc<Vec<u8>>,
    captures: Rc<Vec<bool>>,
}

impl Function {
    pub fn new<Co, Ca>(parameters: usize, locals: usize, code: Co, captures: Ca) -> Self
    where
        Co: Into<Vec<u8>>,
        Ca: Into<Vec<bool>>,
    {
        Function {
            parameters,
            locals,
            code: Rc::new(code.into()),
            captures: Rc::new(captures.into()),
        }
    }

    pub fn code(&self) -> Rc<Vec<u8>> {
        Rc::clone(&self.code)
    }

    pub fn arity(&self) -> usize {
        self.parameters
    }

    pub fn locals(&self) -> usize {
        self.locals
    }

    pub fn captures(&self) -> Rc<Vec<bool>> {
        Rc::clone(&self.captures)
    }

    pub fn values(&self) -> usize {
        1 + self.parameters + self.locals + self.captures.len()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<_/{}>", self.parameters)
    }
}

impl Default for Function {
    fn default() -> Self {
        Function {
            code: Rc::new(vec![]),
            parameters: 0,
            locals: 0,
            captures: Rc::new(vec![]),
        }
    }
}
