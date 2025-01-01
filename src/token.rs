use std::{borrow::Cow, fmt};

use crate::LoxError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    String(Cow<'a, str>),

    Error(LoxError),

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
            Self::Star => f.write_str("STAR * null"),
            Self::Dot => f.write_str("DOT . null"),
            Self::Comma => f.write_str("COMMA , null"),
            Self::Plus => f.write_str("PLUS + null"),
            Self::Minus => f.write_str("MINUS - null"),
            Self::Semicolon => f.write_str("SEMICOLON ; null"),
            Self::Equal => f.write_str("EQUAL = null"),
            Self::Bang => f.write_str("BANG ! null"),
            Self::Slash => f.write_str("SLASH / null"),
            Self::Less => f.write_str("LESS < null"),
            Self::Greater => f.write_str("GREATER > null"),
            Self::EqualEqual => f.write_str("EQUAL_EQUAL == null"),
            Self::BangEqual => f.write_str("BANG_EQUAL != null"),
            Self::LessEqual => f.write_str("LESS_EQUAL <= null"),
            Self::GreaterEqual => f.write_str("GREATER_EQUAL >= null"),
            Self::String(s) => write!(f, "STRING \"{}\" {}", s, s),
            Self::Error(e) => write!(f, "{}", e),
            Self::Eof => f.write_str("EOF  null"),
        }
    }
}

impl Token<'_> {
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Error(_))
    }
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        !matches!(self, Self::Error(_))
    }
}
