use std::iter::Peekable;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
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
            '!' => Ok(Self::Bang),
            '=' => Ok(Self::Equal),
            '>' => Ok(Self::Greater),
            '<' => Ok(Self::Less),
            _ => Err("Invalid token"),
        }
    }
}

pub fn tokenize<I: Iterator<Item = char>>(
    iter: &mut Peekable<I>,
    line_no: &mut usize,
) -> Result<Option<TokenType>, String> {
    while let Some(c) = iter.next() {
        match c {
            ' ' => continue,
            '\n' => {
                *line_no += 1;
            }
            '=' if iter.peek() == Some(&'=') => {
                dbg!(c);
                iter.next();
                return Ok(Some(TokenType::EqualEqual));
            }
            '=' => return Ok(Some(TokenType::Equal)),
            '!' if iter.peek() == Some(&'=') => {
                iter.next();
                return Ok(Some(TokenType::BangEqual));
            }
            '!' => return Ok(Some(TokenType::Bang)),
            v => {
                if let Ok(t) = TokenType::try_from(v) {
                    return Ok(Some(t));
                } else {
                    return Err(format!(
                        "[line {}] Error: Unexpected character: {}",
                        line_no, v
                    ));
                }
            }
        }
    }
    Ok(None)
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
        Err(format!("[line {}] Error: Unexpected character: {}", 2, '@'))
    );
    assert_eq!(line, 2);
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Plus)));
    assert_eq!(tokenize(&mut iter, &mut line), Ok(Some(TokenType::Minus)));
    assert_eq!(line, 2);
    assert_eq!(tokenize(&mut iter, &mut line), Ok(None));
    assert_eq!(tokenize(&mut iter, &mut line), Ok(None));
    assert_eq!(line, 2);
}
