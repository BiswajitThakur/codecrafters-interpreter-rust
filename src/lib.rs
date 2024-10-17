use std::{char, fmt::Display};

#[allow(unused)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Self::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Self::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Self::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Self::Comma => write!(f, "COMMA , null"),
            Self::Dot => write!(f, "DOT . null"),
            Self::Minus => write!(f, "MINUS - null"),
            Self::Plus => write!(f, "PLUS + null"),
            Self::Semicolon => write!(f, "SEMICOLON ; null"),
            Self::Slash => write!(f, "SLASH / null"),
            Self::Star => write!(f, "STAR * null"),
        }
    }
}

impl TryFrom<char> for TokenType {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::LeftParen),
            ')' => Ok(Self::RightParen),
            '{' => Ok(Self::LeftBrace),
            '}' => Ok(Self::RightBrace),
            ',' => Ok(Self::Comma),
            '.' => Ok(Self::Dot),
            '-' => Ok(Self::Minus),
            '+' => Ok(Self::Plus),
            ';' => Ok(Self::Semicolon),
            '/' => Ok(Self::Slash),
            '*' => Ok(Self::Star),
            _ => Err("Invalid token"),
        }
    }
}
