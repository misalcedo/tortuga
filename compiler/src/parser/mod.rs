//! Parse a sequence of tokens into a syntax tree.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the nesting of the parsing functions to denote each precedence level.

mod error;

use crate::grammar::{
    ExpressionReference, Identifier, Internal, InternalKind, Number, Program, Uri,
};
use crate::scanner::LexicalError;
use crate::{Location, Token, TokenKind};
pub use error::SyntacticalError;

pub trait ErrorReporter {
    fn report(&mut self, error: SyntacticalError);
}

impl ErrorReporter for Vec<SyntacticalError> {
    fn report(&mut self, error: SyntacticalError) {
        self.push(error)
    }
}

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
pub struct Parser<'a, Iterator, Reporter> {
    reporter: Reporter,
    tokens: Iterator,
    current: Option<Token<'a>>,
    program: Program<'a>,
    end_location: Location,
    had_error: bool,
}

type SyntacticalResult<Output> = Result<Output, SyntacticalError>;

impl<'a, I, R> Parser<'a, I, R>
where
    I: Iterator<Item = Result<Token<'a>, LexicalError>>,
    R: ErrorReporter,
{
    fn new<II>(tokens: II, reporter: R) -> Self
    where
        II: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        Parser {
            reporter,
            tokens: tokens.into_iter(),
            current: None,
            program: Program::default(),
            end_location: Location::default(),
            had_error: false,
        }
    }

    /// Generate a [`Program`] syntax tree for this [`Parser`]'s sequence of [`Token`]s.
    pub fn parse(mut self) -> Result<Program<'a>, R> {
        self.advance();

        while self.current.is_some() {
            match self.parse_expression() {
                Ok(expression) => self.program.mark_root(expression),
                Err(error) => self.synchronize(error),
            }
        }

        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(self.program)
        }
    }

    fn synchronize(&mut self, error: SyntacticalError) {
        self.report_error(error);

        loop {
            if self.current.is_some() {
                self.advance();
            }

            if self.current.is_none() || self.check(TokenKind::Semicolon) {
                return;
            }
        }
    }

    fn parse_expression(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_left_hand_side()
    }

    fn parse_left_hand_side(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_modulo()
    }

    fn parse_modulo(&mut self) -> SyntacticalResult<ExpressionReference> {
        let mut lhs = self.parse_modulo()?;

        while self.consume_conditionally(TokenKind::Caret) {
            let rhs = self.parse_modulo()?;
            let operation = Internal::new(InternalKind::Power, vec![lhs, rhs]);

            lhs = self.program.insert(operation);
        }

        Ok(lhs)
    }

    fn parse_sum(&mut self) -> SyntacticalResult<ExpressionReference> {
        let mut lhs = self.parse_product()?;

        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let operator = if self.consume_conditionally(TokenKind::Plus) {
                InternalKind::Add
            } else {
                self.consume(TokenKind::Minus, "Expected '+' or '-'.")?;
                InternalKind::Subtract
            };
            let rhs = self.parse_product()?;
            let operation = Internal::new(operator, vec![lhs, rhs]);

            lhs = self.program.insert(operation);
        }

        Ok(lhs)
    }

    fn parse_product(&mut self) -> SyntacticalResult<ExpressionReference> {
        let mut lhs = self.parse_power()?;

        while self.check(TokenKind::Star) || self.check(TokenKind::Slash) {
            let operator = if self.consume_conditionally(TokenKind::Star) {
                InternalKind::Multiply
            } else {
                self.consume(TokenKind::Slash, "Expected '*' or '/'.")?;
                InternalKind::Divide
            };
            let rhs = self.parse_power()?;
            let operation = Internal::new(operator, vec![lhs, rhs]);

            lhs = self.program.insert(operation);
        }

        Ok(lhs)
    }

    fn parse_power(&mut self) -> SyntacticalResult<ExpressionReference> {
        let mut lhs = self.parse_call()?;

        while self.consume_conditionally(TokenKind::Caret) {
            let rhs = self.parse_call()?;
            let operation = Internal::new(InternalKind::Power, vec![lhs, rhs]);

            lhs = self.program.insert(operation);
        }

        Ok(lhs)
    }

    fn parse_call(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> SyntacticalResult<ExpressionReference> {
        if self.check(TokenKind::LeftParenthesis) {
            self.parse_grouping()
        } else if self.check(TokenKind::Number) || self.check(TokenKind::Minus) {
            self.parse_number()
        } else if self.check(TokenKind::Identifier) {
            self.parse_identifier()
        } else {
            self.parse_uri()
        }
    }

    fn parse_grouping(&mut self) -> SyntacticalResult<ExpressionReference> {
        self.consume(TokenKind::LeftParenthesis, "")?;
        let expression = self.parse_expression()?;
        self.consume(TokenKind::RightParenthesis, "")?;

        Ok(expression)
    }

    fn parse_number(&mut self) -> SyntacticalResult<ExpressionReference> {
        let negative = self.consume_conditionally(TokenKind::Minus);
        let token = self.consume(TokenKind::Number, "Expected a number.")?;
        let number = Number::new(negative, token.lexeme());

        Ok(self.program.insert(number))
    }

    fn parse_identifier(&mut self) -> SyntacticalResult<ExpressionReference> {
        let token = self.consume(TokenKind::Identifier, "Expected an identifier.")?;
        let identifier = Identifier::from(token.lexeme());

        Ok(self.program.insert(identifier))
    }

    fn parse_uri(&mut self) -> SyntacticalResult<ExpressionReference> {
        let token = self.consume(TokenKind::Uri, "Expected an URI.")?;
        let uri = Uri::from(token.lexeme());

        Ok(self.program.insert(uri))
    }

    fn advance(&mut self) {
        self.current = self.next_token();
        if let Some(token) = &self.current {
            self.end_location = token.end();
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            match self.tokens.next()? {
                Ok(token) => return Some(token),
                Err(error) => self.report_error(error.into()),
            }
        }
    }

    fn report_error(&mut self, error: SyntacticalError) {
        self.had_error = true;
        self.reporter.report(error);
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> SyntacticalResult<Token<'a>> {
        match self.current {
            Some(token) if token.kind() == &kind => {
                self.advance();
                Ok(token)
            }
            Some(ref token) => Err(SyntacticalError::new(message, token.start())),
            None => Err(SyntacticalError::new(message, &self.end_location)),
        }
    }

    fn consume_conditionally(&mut self, kind: TokenKind) -> bool {
        let same = self.check(kind);

        if same {
            self.advance();
        }

        same
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        match self.current {
            Some(token) => token.kind() == &kind,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Internal, InternalKind, Number};
    use crate::Scanner;

    #[test]
    fn from_scanner() {
        // TODO deal with stack overflow.
        let input = "-3 + 2";
        let scanner: Scanner<'_> = input.into();
        let parser: Parser<_, _> = Parser::new(scanner, Vec::new());

        let program = parser.parse().unwrap();

        let mut expected = Program::default();

        let left = Number::negative("3");
        let left_index = expected.insert(left.clone());

        let right = Number::positive("2");
        let right_index = expected.insert(right.clone());

        let add = Internal::new(InternalKind::Add, vec![left_index, right_index]);
        let add_index = expected.insert(add.clone());

        expected.mark_root(add_index);

        assert_eq!(program, expected);
    }
}

//     fn parse_comparison(&mut self) -> Result<Comparison, SyntacticalError> {
//         let operator = self.parse_comparator()?;
//         let expression = self.parse_expression()?;
//
//         Ok(Comparison::new(operator, expression))
//     }
//
//     fn parse_comparator(&mut self) -> Result<Comparator, SyntacticalError> {
//         let operator = match self.next_kind(COMPARISON_KINDS)?.kind() {
//             Kind::LessThan => Comparator::LessThan,
//             Kind::GreaterThan => Comparator::GreaterThan,
//             Kind::LessThanOrEqualTo => Comparator::LessThanOrEqualTo,
//             Kind::GreaterThanOrEqualTo => Comparator::GreaterThanOrEqualTo,
//             Kind::NotEqual => Comparator::NotEqualTo,
//             _ => Comparator::EqualTo,
//         };
//
//         Ok(operator)
//     }
//
//     fn parse_epsilon(&mut self) -> Result<Expression, SyntacticalError> {
//         let lhs = self.parse_modulo()?;
//
//         if self.tokens.next_if_match(Kind::Tilde).is_some() {
//             let rhs = self.parse_modulo()?;
//             Ok(Operation::new(lhs, Operator::Tolerance, rhs).into())
//         } else {
//             Ok(lhs)
//         }
//     }
//
//     fn parse_modulo(&mut self) -> Result<Expression, SyntacticalError> {
//         let mut lhs = self.parse_sum()?;
//
//         while self.tokens.next_if_match(Kind::Percent).is_some() {
//             let rhs = self.parse_sum()?;
//
//             lhs = Operation::new(lhs, Operator::Modulo, rhs).into();
//         }
//
//         Ok(lhs)
//     }
//
//     fn parse_sum(&mut self) -> Result<Expression, SyntacticalError> {
//         let mut lhs = self.parse_product()?;
//
//         while let Some(token) = self.tokens.next_if_match(&[Kind::Plus, Kind::Minus]) {
//             let rhs = self.parse_product()?;
//             let operator = match token.kind() {
//                 Kind::Minus => Operator::Subtract,
//                 _ => Operator::Add,
//             };
//
//             lhs = Operation::new(lhs, operator, rhs).into();
//         }
//
//         Ok(lhs)
//     }
//
//     fn parse_product(&mut self) -> Result<Expression, SyntacticalError> {
//         let mut lhs = self.parse_power()?;
//
//         while let Some(token) = self.tokens.next_if_match(&[Kind::Star, Kind::Slash]) {
//             let rhs = self.parse_power()?;
//             let operator = match token.kind() {
//                 Kind::Slash => Operator::Divide,
//                 _ => Operator::Multiply,
//             };
//
//             lhs = Operation::new(lhs, operator, rhs).into();
//         }
//
//         Ok(lhs)
//     }
//
//     fn parse_power(&mut self) -> Result<Expression, SyntacticalError> {
//         let mut lhs = self.parse_call()?;
//
//         while self.tokens.next_if_match(Kind::Caret).is_some() {
//             let rhs = self.parse_call()?;
//             lhs = Operation::new(lhs, Operator::Exponent, rhs).into();
//         }
//
//         Ok(lhs)
//     }
//
//     fn parse_call(&mut self) -> Result<Expression, SyntacticalError> {
//         let mut expression = self.parse_primary()?;
//
//         while let Some(true) = self.tokens.next_matches(Kind::LeftParenthesis) {
//             let arguments = self.parse_arguments()?;
//             expression = Call::new(expression, arguments).into();
//         }
//
//         Ok(expression.into())
//     }
//
//     fn parse_primary(&mut self) -> Result<Expression, SyntacticalError> {
//         let token = self.next_kind(&[
//             Kind::Minus,
//             Kind::Number,
//             Kind::Identifier,
//             Kind::LeftParenthesis,
//         ])?;
//
//         match token.kind() {
//             Kind::Minus | Kind::Number => self.parse_number(token).map(Expression::from),
//             Kind::Identifier => self.parse_identifier(token).map(Expression::from),
//             _ => self.parse_grouping(token).map(Expression::from),
//         }
//     }
//
//     fn parse_number(&mut self, token: Token) -> Result<Number, SyntacticalError> {
//         match token.kind() {
//             Kind::Minus => {
//                 let number = self.next_kind(Kind::Number)?;
//
//                 Ok(Number::new(true, lexical::Number::new(number.as_str())))
//             }
//             _ => Ok(Number::new(false, lexical::Number::new(token.as_str()))),
//         }
//     }
//
//     fn parse_identifier(
//         &mut self,
//         identifier: Token,
//     ) -> Result<lexical::Identifier, SyntacticalError> {
//         Ok(lexical::Identifier::new(identifier.as_str()))
//     }
//
//     fn parse_arguments(&mut self) -> Result<Arguments, SyntacticalError> {
//         self.next_kind(Kind::LeftParenthesis)?;
//
//         let head = self.parse_expression()?;
//         let mut tail = Vec::new();
//
//         while self.tokens.next_if_match(Kind::Comma).is_some() {
//             tail.push(self.parse_expression()?);
//         }
//
//         self.next_kind(Kind::RightParenthesis)?;
//
//         Ok(List::new(head, tail))
//     }
//
//     fn parse_grouping(&mut self, _: Token) -> Result<Grouping, SyntacticalError> {
//         let expression = self.parse_expression()?;
//
//         self.next_kind(Kind::RightParenthesis)?;
//
//         Ok(expression.into())
//     }
//
//     fn parse_assignment(&mut self) -> Result<Assignment, SyntacticalError> {
//         let function = self.parse_function()?;
//
//         self.next_kind(Kind::Equal)?;
//
//         let block = self.parse_block()?;
//
//         Ok(Assignment::new(function, block))
//     }
//
//     fn parse_function(&mut self) -> Result<Function, SyntacticalError> {
//         let name = self.parse_name()?;
//         let parameters = self.parse_parameters()?;
//
//         Ok(Function::new(name, parameters))
//     }
//
//     fn parse_name(&mut self) -> Result<Name, SyntacticalError> {
//         let token = self.next_kind(NAME_KINDS)?;
//
//         match token.kind() {
//             Kind::At => {
//                 let identifier = self.next_kind(Kind::Identifier)?;
//
//                 Ok(Name::from(lexical::Identifier::new(identifier.as_str())))
//             }
//             _ => Ok(Name::Anonymous),
//         }
//     }
//
//     fn parse_parameters(&mut self) -> Result<Vec<Pattern>, SyntacticalError> {
//         let mut parameters = Vec::new();
//
//         if self.tokens.next_if_match(Kind::LeftParenthesis).is_some() {
//             parameters.push(self.parse_pattern()?);
//
//             while self.tokens.next_if_match(Kind::Comma).is_some() {
//                 parameters.push(self.parse_pattern()?);
//             }
//
//             self.next_kind(Kind::RightParenthesis)?;
//         }
//
//         Ok(parameters)
//     }
//
//     fn parse_pattern(&mut self) -> Result<Pattern, SyntacticalError> {
//         if let Some(true) = self.tokens.next_matches(NAME_KINDS) {
//             let name = self.parse_name()?;
//
//             if let Some(true) = self.tokens.next_matches(COMPARISON_KINDS) {
//                 self.parse_refinement(name)
//             } else {
//                 let parameters = self.parse_parameters()?;
//
//                 Ok(Function::new(name, parameters).into())
//             }
//         } else {
//             Ok(self.parse_bounds()?.into())
//         }
//     }
//
//     fn parse_inequality(&mut self) -> Result<Inequality, SyntacticalError> {
//         let operator = match self.next_kind(INEQUALITY_KINDS)?.kind() {
//             Kind::LessThan => Inequality::LessThan,
//             Kind::GreaterThan => Inequality::GreaterThan,
//             Kind::LessThanOrEqualTo => Inequality::LessThanOrEqualTo,
//             _ => Inequality::GreaterThanOrEqualTo,
//         };
//
//         Ok(operator)
//     }
//
//     fn parse_bounds(&mut self) -> Result<Bounds, SyntacticalError> {
//         let left = Bound::new(self.parse_arithmetic()?, self.parse_inequality()?);
//
//         let name = self.parse_name()?;
//
//         let right_inequality = self.parse_inequality()?;
//         let right_constraint = self.parse_arithmetic()?;
//         let right = Bound::new(right_constraint, right_inequality);
//
//         Ok(Bounds::new(left, name, right))
//     }
//
//     fn parse_refinement(&mut self, name: Name) -> Result<Pattern, SyntacticalError> {
//         let comparator = self.parse_comparator()?;
//         let arithmetic = self.parse_arithmetic()?;
//
//         Ok(Refinement::new(name, comparator, arithmetic).into())
//     }
//
//     fn parse_block(&mut self) -> Result<Block, SyntacticalError> {
//         if let Some(Kind::LeftBracket) = self.tokens.peek_kind() {
//             self.next_kind(Kind::LeftBracket)?;
//
//             let head = self.parse_expression()?;
//             let mut tail = vec![self.parse_expression()?];
//
//             while let Some(false) = self.tokens.next_matches(Kind::RightBracket) {
//                 tail.push(self.parse_expression()?);
//             }
//
//             self.next_kind(Kind::RightBracket)?;
//
//             Ok(List::new(head, tail))
//         } else {
//             let head = self.parse_expression()?;
//
//             Ok(List::new(head, Vec::new()))
//         }
//     }
// }
//
// impl FromStr for Program {
//     type Err = SyntacticalError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let tokens = Tokens::try_from(Scanner::from(s))?;
//         let parser = Parser::from(tokens);
//
//         parser.parse()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_number() {
//         assert!("2".parse::<Program>().is_ok());
//     }
//
//     #[test]
//     fn parse_with_panic() {
//         let result = "+x".parse::<Program>();
//
//         assert_eq!(result, Err(SyntacticalError::Multiple));
//     }
//
//     #[test]
//     fn parse_negative_number() {
//         assert!("-2#100".parse::<Program>().is_ok());
//     }
//
//     #[test]
//     fn parse_identifier() {
//         assert!("xyz".parse::<Program>().is_ok());
//     }
//
//     #[test]
//     fn parse_example() {
//         assert!(include_str!("../../../examples/example.ta")
//             .parse::<Program>()
//             .is_ok())
//     }
//
//     #[test]
//     fn parse_factorial() {
//         assert!(include_str!("../../../examples/factorial.ta")
//             .parse::<Program>()
//             .is_ok())
//     }
//
//     #[test]
//     fn parse_bad() {
//         assert!(include_str!("../../../../examples/bad.ta")
//             .parse::<Program>()
//             .is_err())
//     }
// }
