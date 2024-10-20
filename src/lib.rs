use std::{fmt, iter::Peekable};

#[allow(unused)]
struct Token {
    _type: TokenType,
    lexeme: String,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    InvalidTaken(char),
    UnterminatedStr,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTaken(e) => write!(f, "Unexpected character: {}", e),
            Self::UnterminatedStr => write!(f, "Unterminated string."),
        }
    }
}

#[derive(Debug, PartialEq)]
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    String(String),
    Number(String),
    Identifier(String),
    Keyword(Keyword),
}

impl std::fmt::Display for TokenType {
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
            Self::Bang => write!(f, "BANG ! null"),
            Self::BangEqual => write!(f, "BANG_EQUAL != null"),
            Self::Equal => write!(f, "EQUAL = null"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Self::Greater => write!(f, "GREATER > null"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Self::Less => write!(f, "LESS < null"),
            Self::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Self::String(ref s) => write!(f, "STRING \"{}\" {}", s, s),
            Self::Number(ref n) => {
                if n.ends_with(".0") {
                    write!(f, "NUMBER {} {}", n.replace(".0", ""), n)
                } else if !n.contains(".") {
                    write!(f, "NUMBER {} {}.0", n, n)
                } else {
                    let num: f64 = n.parse().unwrap();
                    if num.fract() == 0.0 {
                        write!(f, "NUMBER {} {}.0", n, num)
                    } else {
                        write!(f, "NUMBER {} {}", n, num)
                    }
                }
            }
            Self::Identifier(ref v) => write!(f, "IDENTIFIER {} null", v),
            Self::Keyword(ref k) => write!(f, "{}", k),
        }
    }
}

impl TryFrom<char> for TokenType {
    type Error = char;
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
            '!' => Ok(Self::Bang),
            '=' => Ok(Self::Equal),
            '>' => Ok(Self::Greater),
            '<' => Ok(Self::Less),
            v => Err(v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => write!(f, "AND and null"),
            Self::Class => write!(f, "CLASS class null"),
            Self::Else => write!(f, "ELSE else null"),
            Self::False => write!(f, "FALSE false null"),
            Self::For => write!(f, "FOR for null"),
            Self::Fun => write!(f, "FUN fun null"),
            Self::If => write!(f, "IF if null"),
            Self::Nil => write!(f, "NIL nil null"),
            Self::Or => write!(f, "OR or null"),
            Self::Print => write!(f, "PRINT print null"),
            Self::Return => write!(f, "RETURN return null"),
            Self::Super => write!(f, "SUPER super null"),
            Self::This => write!(f, "THIS this null"),
            Self::True => write!(f, "TRUE true null"),
            Self::Var => write!(f, "VAR var null"),
            Self::While => write!(f, "WHILE while null"),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
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
            _ => Err("Unkown keyword"),
        }
    }
}

pub fn tokenize<I: Iterator<Item = char>>(
    iter: &mut Peekable<I>,
    line: &mut usize,
) -> Result<Option<TokenType>, ErrorType> {
    while let Some(c) = iter.next() {
        match c {
            ' ' | '\r' | '\t' => continue,
            '\n' => {
                *line += 1;
            }
            '=' if iter.peek() == Some(&'=') => {
                iter.next();
                return Ok(Some(TokenType::EqualEqual));
            }
            '!' if iter.peek() == Some(&'=') => {
                iter.next();
                return Ok(Some(TokenType::BangEqual));
            }
            '<' if iter.peek() == Some(&'=') => {
                iter.next();
                return Ok(Some(TokenType::LessEqual));
            }
            '>' if iter.peek() == Some(&'=') => {
                iter.next();
                return Ok(Some(TokenType::GreaterEqual));
            }
            '/' if iter.peek() == Some(&'/') => {
                iter.next();
                while let Some(line_end) = iter.peek() {
                    if line_end == &'\n' {
                        break;
                    }
                    iter.next();
                }
            }
            '"' => {
                let mut s = String::new();
                let mut end = false;
                while let Some(v) = iter.peek() {
                    match *v {
                        '"' => {
                            end = true;
                            iter.next();
                            break;
                        }
                        k => s.push(k),
                    }
                    iter.next();
                }
                if end {
                    return Ok(Some(TokenType::String(s)));
                } else {
                    return Err(ErrorType::UnterminatedStr);
                }
            }
            '0'..='9' => {
                let mut has_didit = false;
                let mut num = String::new();
                num.push(c);
                while let Some(digit) = iter.peek() {
                    match digit {
                        '0'..='9' => {
                            num.push(*digit);
                            iter.next();
                        }
                        '.' => {
                            if has_didit {
                                break;
                            }
                            has_didit = true;
                            num.push(*digit);
                            iter.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                return Ok(Some(TokenType::Number(num)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                ident.push(c);
                loop {
                    match iter.peek() {
                        Some(v) => {
                            if matches!(v, 'a'..='z' | 'A'..='Z'| '0'..='9' | '_') {
                                if let Some(ch) = iter.next() {
                                    ident.push(ch);
                                    continue;
                                }
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                if let Ok(k) = Keyword::try_from(ident.as_str()) {
                    return Ok(Some(TokenType::Keyword(k)));
                }
                return Ok(Some(TokenType::Identifier(ident)));
            }
            v => {
                let t = TokenType::try_from(v);
                if let Ok(t) = t {
                    return Ok(Some(t));
                };
                if let Err(e) = t {
                    return Err(ErrorType::InvalidTaken(e));
                }
                unreachable!()
            }
        }
    }
    Ok(None)
}

#[test]
fn test_tokenize4() {
    let input = r#"+"hello   -"#;
    let mut line = 1;
    let mut iter = input.chars().peekable();

    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Plus)));
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Err(ErrorType::UnterminatedStr)
    );
    assert_eq!(tokenize(&mut iter, &mut line), Ok(None));
}

#[test]
fn test_tokenize3() {
    let input = r#"+"hello"   -"#;
    let mut line = 1;
    let mut iter = input.chars().peekable();

    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Plus)));
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::String(String::from("hello"))))
    );
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Minus)));
}

#[test]
fn test_tokenize2() {
    let input = "({=}){==}";
    let mut line = 1;
    let mut iter = input.chars().peekable();

    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::LeftParen))
    );

    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::LeftBrace))
    );
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Equal)));
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::RightBrace))
    );
}

#[test]
fn test_tokenize1() {
    let input = "(  =}== \n @    + -    ";
    let mut line = 1;
    let mut iter = input.chars().peekable();
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::LeftParen))
    );
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Equal)));
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::RightBrace))
    );
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Ok(Some(TokenType::EqualEqual))
    );
    assert_eq!(line, 1);
    assert_eq!(
        tokenize(&mut iter, &mut line),
        Err(ErrorType::InvalidTaken('@'))
    );
    assert_eq!(line, 2);
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Plus)));
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Minus)));
    assert_eq!(line, 2);
    assert_eq!(tokenize(&mut iter, &mut line), Ok(None));
    assert_eq!(tokenize(&mut iter, &mut line), Ok(None));
    assert_eq!(line, 2);
}
