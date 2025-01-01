use crate::{LoxError, Token, WithSpan};

#[derive(Debug)]
struct Scanner<'a> {
    val: &'a [u8],
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(value: &'a str) -> Self {
        Self {
            val: value.as_bytes(),
            pos: 0,
        }
    }
    fn advance(&mut self) {
        self.pos += 1;
    }
    fn peek(&mut self) -> Option<u8> {
        self.val.get(self.pos).copied()
    }
    fn consume_if<F: Fn(u8) -> bool>(&mut self, f: F) -> bool {
        if let Some(ch) = self.peek() {
            if f(ch) {
                self.next().unwrap();
                return true;
            }
        }
        false
    }
    fn consume_if_next<F: Fn(u8) -> bool>(&mut self, f: F) -> bool {
        if let Some(&ch) = self.val.get(self.pos + 1) {
            if f(ch) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn consume_while<F: Fn(u8) -> bool>(&mut self, f: F) -> &'a [u8] {
        let start = self.pos;
        while let Some(u) = self.peek() {
            if f(u) {
                self.advance();
            } else {
                break;
            }
        }
        &self.val[start..self.pos]
    }
}

impl Iterator for Scanner<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.peek();
        if nxt.is_some() {
            self.advance();
        }
        nxt
    }
}

impl<'a> From<&'a [u8]> for Scanner<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { val: value, pos: 0 }
    }
}

pub struct Lexer<'a> {
    sc: Scanner<'a>,
    line: usize,
    end: bool,
}

impl<'a> From<&'a [u8]> for Lexer<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            sc: Scanner::from(value),
            line: 0,
            end: false,
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            sc: Scanner::from(value.as_bytes()),
            line: 0,
            end: false,
        }
    }
}

impl<'a> Lexer<'a> {
    fn match_token(&mut self, v: u8) -> Option<Token<'a>> {
        match v {
            b'=' => Some(self.either(b'=', Token::EqualEqual, Token::Equal)),
            b'!' => Some(self.either(b'=', Token::BangEqual, Token::Bang)),
            b'>' => Some(self.either(b'=', Token::GreaterEqual, Token::Greater)),
            b'<' => Some(self.either(b'=', Token::LessEqual, Token::Less)),
            b'/' => {
                if self.sc.consume_if(|u| u == b'/') {
                    self.sc.consume_while(|u| u != b'\n');
                    None
                } else {
                    Some(Token::Slash)
                }
            }
            b'(' => Some(Token::LeftParen),
            b')' => Some(Token::RightParen),
            b'{' => Some(Token::LeftBrace),
            b'}' => Some(Token::RightBrace),
            b'[' => Some(Token::LeftBracket),
            b']' => Some(Token::RightBracket),
            b',' => Some(Token::Comma),
            b'-' => Some(Token::Minus),
            b'+' => Some(Token::Plus),
            b';' => Some(Token::Semicolon),
            b'*' => Some(Token::Star),
            b'.' => Some(Token::Dot),
            _ => todo!(),
        }
    }
    fn either(&mut self, to_match: u8, matched: Token<'a>, unmatched: Token<'a>) -> Token<'a> {
        if self.sc.consume_if(|v| v == to_match) {
            matched
        } else {
            unmatched
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = WithSpan<Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let initial_pos = self.sc.pos;
            let nxt_char = self.sc.next();
            if nxt_char.is_none() {
                if self.end {
                    return None;
                } else {
                    self.end = true;
                    let end_pos = self.sc.pos.checked_sub(1).unwrap_or(0);
                    return Some(WithSpan::new(Token::Eof, end_pos..end_pos));
                }
            }
            let token = self.match_token(nxt_char.unwrap());
            if token.is_none() {
                continue;
            }
            let end_pos = self.sc.pos;
            return Some(WithSpan::new(token.unwrap(), initial_pos..end_pos));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Token, WithSpan};

    use super::Lexer;

    #[test]
    fn test_empty() {
        let input = "";
        let mut lx = Lexer::from(input);
        assert_eq!(lx.next(), Some(WithSpan::new(Token::Eof, 0..0)));
        assert_eq!(lx.next(), None);
    }
    #[test]
    fn test_brace() {
        let input = "())(";
        let mut lx = Lexer::from(input);
        assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftParen, 0..1)));
        assert_eq!(lx.next(), Some(WithSpan::new(Token::RightParen, 1..2)));
        assert_eq!(lx.next(), Some(WithSpan::new(Token::RightParen, 2..3)));
        assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftParen, 3..4)));
        assert_eq!(lx.next(), Some(WithSpan::new(Token::Eof, 3..3)));
        assert_eq!(lx.next(), None);
    }
}
