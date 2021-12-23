pub enum Token {
    Number(Number),
    Identifier(Identifier),
    Delimiter(Delimiter),
    Punctuation(Punctuation)
}

pub struct Number;
pub struct Identifier;

pub enum Delimiter {
        /// (
        LeftParenthesis,
        /// )
        RightParenthesis,
        
        /// {
        LeftBrace,
        /// }
        RightBrace,
        
        //. [
        LeftBracket,
        /// ]
        RightBracket,
}

pub enum Punctuation {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    /// 
    /// Modulo. See <https://stackoverflow.com/questions/13683563/whats-the-difference-between-mod-and-remainder>
    Percent,
    /// ^
    Caret,
    /// ~
    /// 
    /// Epsilon operator (i.e. + or - some amount)
    Tilde,

    /// =
    /// 
    /// Pattern match operator
    Equal,
    /// <>
    NotEqual,
    /// <
    LessThan,
    /// <=
    LessThanOrEqualTo,
    /// >
    GreaterThan,
    /// >=
    GreaterThanOrEqualTo,

    /// ,
    Comma,
}