//! Compiler modules.

mod errors;
mod lexer;
mod location;
mod parser;
mod scanner;
mod stream;
mod token;

pub use errors::{LexicalError, SyntaxError, ParseError};
pub use lexer::Lexer;
pub use location::Location;
pub use parser::Parser;
pub use scanner::Scanner;
pub use token::{InvalidToken, Kind, ValidToken};

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
pub fn parse(source: &str) -> Result<crate::grammar::Program, ParseError> {
    let mut scanner = Scanner::from(source);
    let lexer = Lexer::new(&mut scanner);
    let parser = Parser::new(lexer);

    parser.parse()
}