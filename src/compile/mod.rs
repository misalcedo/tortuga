//! Compiler modules.

mod errors;
mod lexer;
mod parser;
mod scanner;
mod stream;
mod token;

pub use errors::*;
pub use lexer::Lexer;
pub use parser::Parser;
pub(crate) use scanner::Scanner;
pub(crate) use stream::TokenStream;
pub use token::*;

use crate::grammar::Program;

/// Parses a given string into an abstract syntax tree.
///
/// # Examples
/// ```rust
/// use tortuga::parse;
/// use tortuga::grammar::*;
///
/// assert_eq!(
///     parse("x=1").unwrap(),
///     Program::from(vec![
///         ComparisonOperation::new(
///             Variable::new("x").into(),
///             ComparisonOperator::EqualTo,
///             Number::new(None, 1, Fraction::default()).into()
///         ).into()
///     ])
/// );
/// ```
pub fn parse(source: &str) -> Result<Program, ParseError> {
    let lexer = Lexer::from(source);
    let parser = Parser::from(lexer);

    parser.parse()
}
