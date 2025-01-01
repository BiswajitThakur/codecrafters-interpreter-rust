use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    String(Cow<'a, str>),

    Eof,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen => f.write_str("LEFT_PAREN ( null"),
            Self::RightParen => f.write_str("RIGHT_PAREN ) null"),
            Self::LeftBrace => f.write_str("LEFT_BRACE { null"),
            Self::RightBrace => f.write_str("RIGHT_BRACE } null"),
            Self::LeftBracket => f.write_str("LEFT_BRACKET [ null"),
            Self::RightBracket => f.write_str("RIGHT_BRACKET ] null"),
            Self::Eof => f.write_str("EOF  null"),
            _ => Ok(()),
        }
    }
}
