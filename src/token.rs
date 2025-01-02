use std::{borrow::Cow, fmt, str::FromStr};

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

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Import,

    // Literals.
    Identifier(Cow<'a, str>),
    String(Cow<'a, str>),
    Number(f64, Cow<'a, str>),

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
            Self::String(s) => write!(f, "STRING \"{s}\" {s}"),
            Self::Number(g, v) => {
                if g.fract() == 0.0 {
                    write!(f, "NUMBER {} {}.0", v, *g as i64)
                } else {
                    write!(f, "NUMBER {v} {g}")
                }
            }
            Self::Error(e) => write!(f, "{e}"),
            Self::And => f.write_str("AND and null"),
            Self::Class => f.write_str("CLASS class null"),
            Self::Else => f.write_str("ELSE else null"),
            Self::False => f.write_str("FALSE false null"),
            Self::For => f.write_str("FOR for null"),
            Self::Fun => f.write_str("FUN fun null"),
            Self::If => f.write_str("IF if null"),
            Self::Nil => f.write_str("NIL nil null"),
            Self::Or => f.write_str("OR or null"),
            Self::Print => f.write_str("PRINT print null"),
            Self::Return => f.write_str("RETURN return null"),
            Self::Super => f.write_str("SUPER super null"),
            Self::This => f.write_str("THIS this null"),
            Self::True => f.write_str("TRUE true null"),
            Self::Var => f.write_str("VAR var null"),
            Self::While => f.write_str("WHILE while null"),
            Self::Import => f.write_str("IMPORT import null"),
            Self::Identifier(i) => write!(f, "IDENTIFIER {i} null"),
            Self::Eof => f.write_str("EOF  null"),
        }
    }
}

impl Token<'_> {
    #[inline]
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    #[inline]
    pub fn is_ok(&self) -> bool {
        !matches!(self, Self::Error(_))
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        *self == Self::Eof
    }
}

impl FromStr for Token<'_> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(Self::And),
            "class" => Ok(Self::Class),
            "else" => Ok(Self::Else),
            "false" => Ok(Self::False),
            "for" => Ok(Self::For),
            "fun" => Ok(Self::Fun),
            "if" => Ok(Self::If),
            "nil" => Ok(Self::Nil),
            "or" => Ok(Self::Or),
            "print" => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super" => Ok(Self::Super),
            "this" => Ok(Self::This),
            "true" => Ok(Self::True),
            "var" => Ok(Self::Var),
            "while" => Ok(Self::While),
            "import" => Ok(Self::Import),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {
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

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Import,

    // Other.
    Error,
    Eof,
}

impl From<&Token<'_>> for TokenKind {
    fn from(token: &Token) -> Self {
        match token {
            Token::LeftParen => TokenKind::LeftParen,
            Token::RightParen => TokenKind::RightParen,
            Token::LeftBrace => TokenKind::LeftBrace,
            Token::RightBrace => TokenKind::RightBrace,
            Token::LeftBracket => TokenKind::LeftBracket,
            Token::RightBracket => TokenKind::RightBracket,
            Token::Comma => TokenKind::Comma,
            Token::Dot => TokenKind::Dot,
            Token::Minus => TokenKind::Minus,
            Token::Plus => TokenKind::Plus,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Slash => TokenKind::Slash,
            Token::Star => TokenKind::Star,
            Token::Bang => TokenKind::Bang,
            Token::BangEqual => TokenKind::BangEqual,
            Token::Equal => TokenKind::Equal,
            Token::EqualEqual => TokenKind::EqualEqual,
            Token::Greater => TokenKind::Greater,
            Token::GreaterEqual => TokenKind::GreaterEqual,
            Token::Less => TokenKind::Less,
            Token::LessEqual => TokenKind::LessEqual,
            Token::Identifier(_) => TokenKind::Identifier,
            Token::String(_) => TokenKind::String,
            Token::Number(_, _) => TokenKind::Number,
            Token::And => TokenKind::And,
            Token::Class => TokenKind::Class,
            Token::Else => TokenKind::Else,
            Token::False => TokenKind::False,
            Token::Fun => TokenKind::Fun,
            Token::For => TokenKind::For,
            Token::If => TokenKind::If,
            Token::Nil => TokenKind::Nil,
            Token::Or => TokenKind::Or,
            Token::Print => TokenKind::Print,
            Token::Return => TokenKind::Return,
            Token::Super => TokenKind::Super,
            Token::This => TokenKind::This,
            Token::True => TokenKind::True,
            Token::Var => TokenKind::Var,
            Token::While => TokenKind::While,
            Token::Import => TokenKind::Import,
            Token::Error(_) => TokenKind::Error,
            Token::Eof => TokenKind::Eof,
        }
    }
}
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::LeftParen => "')'",
                TokenKind::RightParen => "')'",
                TokenKind::LeftBrace => "'{'",
                TokenKind::RightBrace => "'}'",
                TokenKind::LeftBracket => "'['",
                TokenKind::RightBracket => "']'",
                TokenKind::Comma => "','",
                TokenKind::Dot => "'.'",
                TokenKind::Minus => "'-'",
                TokenKind::Plus => "'+'",
                TokenKind::Semicolon => "';'",
                TokenKind::Slash => "'/'",
                TokenKind::Star => "'*'",
                TokenKind::Bang => "'!'",
                TokenKind::BangEqual => "'!='",
                TokenKind::Equal => "'='",
                TokenKind::EqualEqual => "'=='",
                TokenKind::Greater => "'>'",
                TokenKind::GreaterEqual => "'>='",
                TokenKind::Less => "'<'",
                TokenKind::LessEqual => "'<='",
                TokenKind::Identifier => "identifier",
                TokenKind::String => "string",
                TokenKind::Number => "number",
                TokenKind::And => "'and'",
                TokenKind::Class => "'class'",
                TokenKind::Else => "'else'",
                TokenKind::False => "'false'",
                TokenKind::Fun => "'fun'",
                TokenKind::For => "'for'",
                TokenKind::If => "'if'",
                TokenKind::Nil => "nil",
                TokenKind::Or => "'or'",
                TokenKind::Print => "'print'",
                TokenKind::Return => "'return'",
                TokenKind::Super => "'super'",
                TokenKind::This => "'this'",
                TokenKind::True => "'true'",
                TokenKind::Var => "'var'",
                TokenKind::While => "'while'",
                TokenKind::Import => "'import'",
                TokenKind::Eof => "<EOF>",
                TokenKind::Error => "<Unknown>",
            }
        )
    }
}
