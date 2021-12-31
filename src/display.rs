use crate::compiler::{Lexeme, Token};
use crate::grammar::*;
use crate::{runtime, Kind, LexicalError, SyntacticalError};
use colored::*;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{self, Write};

/// A trait that defines how to get a [`Lexeme`] from types that can be displayed.
pub trait WithLexeme {
    /// The [`Lexeme`] of this object.
    fn lexeme(&self) -> &Lexeme;

    /// Create a [`LexemeString`] for this instance with the given source.
    fn as_display<'a>(&self, source: &'a str) -> LexemeString<'a> {
        LexemeString {
            source,
            lexeme: *self.lexeme(),
        }
    }
}

/// A [`String`]-like type to display [`Lexeme`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LexemeString<'a> {
    source: &'a str,
    lexeme: Lexeme,
}

impl<'a> fmt::Display for LexemeString<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.lexeme.extract_from(self.source))
    }
}

impl WithLexeme for Lexeme {
    fn lexeme(&self) -> &Lexeme {
        self
    }
}

/// A printer to standard out for Tortuga programs.
pub struct PrettyPrinter<'a, StdOut: Write, StdErr: Write> {
    source: &'a str,
    std_out: StdOut,
    std_err: StdErr,
}

impl<'a, StdOut: Write, StdErr: Write> PrettyPrinter<'a, StdOut, StdErr> {
    /// Create a new pretty printer.
    pub fn new(source: &'a str, std_out: StdOut, std_err: StdErr) -> Self {
        PrettyPrinter {
            source,
            std_out,
            std_err,
        }
    }

    /// Prints a [`Display`] instance to this [`PrettyPrinter`]'s `std_out` [`Write`]r.
    pub fn print<D: Display>(&mut self, value: D) -> io::Result<()> {
        write!(self.std_out, "{}", value)
    }

    /// Prints a [`Display`] instance to this [`PrettyPrinter`]'s `std_err` [`Write`]r.
    pub fn print_err<D: Display>(&mut self, value: D) -> io::Result<()> {
        write!(self.std_err, "{}", value)
    }

    /// Prints a [`Token`] to this [`PrettyPrinter`]'s `std_out` [`Write`]r.
    pub fn print_token(&mut self, token: Token) -> io::Result<()> {
        print_token_to(self.source, token, &mut self.std_out)
    }

    /// Prints a [`SyntacticalError`] to this [`PrettyPrinter`]'s `std_err` [`Write`]r.
    pub fn print_syntactical_error(&mut self, error: SyntacticalError) -> io::Result<()> {
        match error {
            SyntacticalError::Incomplete => {
                self.print_error_prefix("EOF")?;
                writeln!(self.std_err, "Reached the end of file prematurely; unable to complete parsing a grammar rule.")
            }
            SyntacticalError::NoMatch(token) => {
                self.print_error_prefix("NoMatch")?;
                self.print_err("No grammar rule matched the token: ")?;
                print_token_to(self.source, token, &mut self.std_err)
            }
            SyntacticalError::Lexical(error) => self.print_lexical_error(error),
        }
    }

    /// Prints a [`LexicalError`] to this [`PrettyPrinter`]'s `std_err` [`Write`]r.
    pub fn print_lexical_error(&mut self, error: LexicalError) -> io::Result<()> {
        let kind = error.kind();
        let lexeme = error.as_display(self.source).to_string();
        let start = error.lexeme().start().to_string();

        self.print_error_prefix(kind)?;

        writeln!(
            self.std_err,
            "\"{}\" {} {}",
            lexeme.blue(),
            "@".yellow().bold(),
            start.red()
        )
    }

    fn print_error_prefix<T: ToString>(&mut self, kind: T) -> io::Result<()> {
        write!(
            self.std_err,
            "[{}|{}] ",
            "ERROR".red().bold(),
            kind.to_string().green().bold()
        )
    }

    /// Prints a [`Program`] to this [`PrettyPrinter`]'s `std_out` [`Write`]r.
    pub fn print_program(&mut self, program: &Program) -> io::Result<()> {
        match program {
            Program::Expressions(expressions) => self.print_expressions(expressions),
            Program::Comparisons(comparisons) => self.print_comparisons(comparisons),
        }
    }

    fn print_expressions(&mut self, expressions: &Expressions) -> io::Result<()> {
        Ok(())
    }

    fn print_comparisons(&mut self, comparisons: &Comparisons) -> io::Result<()> {
        Ok(())
    }
}

fn print_token_to<W: Write>(source: &str, token: Token, mut write: W) -> io::Result<()> {
    let kind = token.kind().to_string();
    let lexeme = token.as_display(source).to_string();
    let start = token.lexeme().start().to_string();

    match token.kind() {
        Kind::Number => writeln!(
            write,
            "[{}] \"{}\" = {} {} {}",
            kind.green().bold(),
            lexeme.blue(),
            lexeme
                .parse::<runtime::Number>()
                .unwrap_or_default()
                .to_string()
                .blue()
                .bold(),
            "@".yellow().bold(),
            start.red()
        )?,
        Kind::Identifier => writeln!(
            write,
            "[{}] \"{}\" {} {}",
            kind.green().bold(),
            lexeme.blue(),
            "@".yellow().bold(),
            start.red()
        )?,
        _ => writeln!(
            write,
            "[{}] {} {}",
            kind.green().bold(),
            "@".yellow().bold(),
            start.red()
        )?,
    }

    Ok(())
}
