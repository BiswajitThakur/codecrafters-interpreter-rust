use std::str::FromStr;
use std::{borrow::Cow, ops::Range};

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
            line: 1,
            end: false,
        }
    }
}

impl<'a> Lexer<'a> {
    #[inline(always)]
    pub fn get_line(&self) -> usize {
        self.line
    }
    fn match_token(&mut self, v: u8) -> Option<Token<'a>> {
        match v {
            s if (s as char).is_ascii_whitespace() => None,
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
            b'"' => {
                let st = self.sc.consume_while(|u| u != b'"');
                if self.sc.next().is_some() {
                    Some(Token::String(unsafe {
                        Cow::Borrowed(std::str::from_utf8_unchecked(st))
                    }))
                } else {
                    Some(Token::Error(LoxError::UnterminatedStr(self.line)))
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let rng = self.identifier();
                let start = rng.start.checked_sub(1).unwrap_or(rng.start);
                let st = unsafe { std::str::from_utf8_unchecked(&self.sc.val[start..rng.end]) };
                if let Ok(token) = Token::from_str(st) {
                    Some(token)
                } else {
                    Some(Token::Identifier(Cow::Borrowed(st)))
                }
            }
            b'0'..=b'9' => {
                let rng = self.number();
                let start = rng.start.checked_sub(1).unwrap_or(rng.start);
                let st = unsafe { std::str::from_utf8_unchecked(&self.sc.val[start..rng.end]) };
                Some(Token::Number(
                    f64::from_str(st).unwrap_or_default(),
                    Cow::Borrowed(st),
                ))
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
            e => Some(Token::Error(LoxError::InvalidChar(self.line, e as char))),
        }
    }
    fn either(&mut self, to_match: u8, matched: Token<'a>, unmatched: Token<'a>) -> Token<'a> {
        if self.sc.consume_if(|v| v == to_match) {
            matched
        } else {
            unmatched
        }
    }
    fn number(&mut self) -> Range<usize> {
        let start = self.sc.pos;
        self.sc.consume_while(|u| matches!(u, b'0'..=b'9'));
        if let Some(v) = self.sc.peek() {
            if v == b'.' {
                if self.sc.consume_if_next(|v| matches!(v, b'0'..=b'9')) {
                    self.sc.consume_while(|u| matches!(u, b'0'..=b'9'));
                }
            }
        }
        start..self.sc.pos
    }
    fn identifier(&mut self) -> Range<usize> {
        let start = self.sc.pos;
        self.sc
            .consume_while(|u| matches!(u, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'));
        start..self.sc.pos
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
                    return Some(WithSpan::new(Token::Eof, self.line, end_pos..end_pos));
                }
            }
            let nxt_char = nxt_char.unwrap();
            if nxt_char == b'\n' {
                self.line += 1;
            }
            let token = self.match_token(nxt_char);
            if token.is_none() {
                continue;
            }
            let end_pos = self.sc.pos;
            return Some(WithSpan::new(
                token.unwrap(),
                self.line,
                initial_pos..end_pos,
            ));
        }
    }
}
