use std::{
    borrow::Cow,
    fmt,
    io::{self, ErrorKind},
    marker::PhantomData,
};

use crate::{
    ast::{BinaryOperator, Expr, UnaryOperator},
    token::TokenKind,
    Token, WithSpan,
};

pub struct Parser<'a, W: io::Write> {
    tokens: &'a [WithSpan<Token<'a>>],
    cursor: usize,
    diagnostics: Option<W>,
    _marker: PhantomData<&'a W>,
}

impl<'a, W: io::Write> Parser<'a, W> {
    pub fn new(tokens: &'a [WithSpan<Token<'a>>], diagnostics: Option<W>) -> Self {
        Self {
            tokens,
            cursor: 0,
            diagnostics,
            _marker: PhantomData,
        }
    }

    pub fn is_eof(&self) -> bool {
        if let Some(v) = self.tokens.get(self.cursor) {
            v.get_value().is_eof()
        } else {
            false
        }
    }

    pub fn error<E: fmt::Display>(&mut self, msg: E) -> io::Result<()> {
        if let Some(f) = self.diagnostics.as_mut() {
            write!(f, "{}", msg)?;
        }
        Ok(())
    }

    #[inline]
    pub fn peek(&self) -> TokenKind {
        if let Some(v) = self.tokens.get(self.cursor) {
            v.get_value().into()
        } else {
            TokenKind::Eof
        }
    }
    pub fn peek_token(&self) -> Cow<'a, WithSpan<Token<'a>>> {
        if let Some(token) = self.tokens.get(self.cursor) {
            Cow::Borrowed(token)
        } else {
            Cow::Owned(WithSpan::empty(Token::Eof))
        }
    }
    #[inline]
    pub fn check(&self, match_token: TokenKind) -> bool {
        self.peek() == match_token
    }

    pub fn advance(&mut self) -> io::Result<&'a WithSpan<Token<'a>>> {
        if let Some(v) = self.tokens.get(self.cursor) {
            self.cursor += 1;
            Ok(v)
        } else {
            Err(io::Error::new(ErrorKind::NotFound, "Token Not Found."))
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> io::Result<&'a WithSpan<Token<'a>>> {
        let token = self.advance()?;
        if TokenKind::from(token.get_value()) == expected {
            Ok(token)
        } else {
            self.error(format!(
                "[line {}] Error at {}: Expect expression.",
                token.get_line(),
                token.get_value()
            ))?;
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Expected Token Not Found.",
            ))
        }
    }

    pub fn optionally(&mut self, expected: TokenKind) -> io::Result<bool> {
        let token = self.peek();
        if TokenKind::from(token) == expected {
            self.expect(expected)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn parse(&mut self) -> io::Result<WithSpan<Expr<'a>>> {
        self.parse_expr(Precedence::None)
    }

    fn parse_expr(&mut self, precedence: Precedence) -> io::Result<WithSpan<Expr<'a>>> {
        let mut expr = self.parse_prefix()?;
        while !self.is_eof() {
            let next_precedence = Precedence::from(self.peek());
            if precedence >= next_precedence {
                break;
            }
            expr = self.parse_infix(expr)?;
        }
        Ok(expr)
    }

    fn parse_prefix(&mut self) -> io::Result<WithSpan<Expr<'a>>> {
        match self.peek() {
            TokenKind::Number
            | TokenKind::Nil
            | TokenKind::This
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Identifier
            | TokenKind::Super
            | TokenKind::String => self.parse_primary(),
            TokenKind::Bang | TokenKind::Minus => self.parse_unary(),
            TokenKind::LeftParen => self.parse_grouping(),
            TokenKind::LeftBracket => todo!(),
            _ => {
                self.error(format!("Unexpected: TODO"))?;
                Err(io::Error::new(ErrorKind::InvalidInput, "Invalid Input."))
            }
        }
    }

    fn parse_infix(&mut self, left: WithSpan<Expr<'a>>) -> io::Result<WithSpan<Expr<'a>>> {
        match self.peek() {
            TokenKind::BangEqual
            | TokenKind::EqualEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash => self.parse_binary(left),
            TokenKind::Or | TokenKind::And => todo!(),
            TokenKind::Equal => todo!(),
            TokenKind::LeftParen => self.parse_grouping(),
            TokenKind::LeftBracket => todo!(),
            TokenKind::Dot => todo!(),
            _ => todo!(),
        }
    }

    fn parse_primary(&mut self) -> io::Result<WithSpan<Expr<'a>>> {
        let tc = self.advance()?;
        match tc.get_value() {
            Token::Nil => Ok(WithSpan::new(Expr::Nil, tc.get_line(), tc.get_span())),
            Token::This => Ok(WithSpan::new(Expr::This, tc.get_line(), tc.get_span())),
            Token::Number(n, _) => Ok(WithSpan::new(
                Expr::Number(*n),
                tc.get_line(),
                tc.get_span(),
            )),
            Token::True => Ok(WithSpan::new(
                Expr::Boolean(true),
                tc.get_line(),
                tc.get_span(),
            )),
            Token::False => Ok(WithSpan::new(
                Expr::Boolean(false),
                tc.get_line(),
                tc.get_span(),
            )),
            Token::String(ref s) => Ok(WithSpan::new(
                Expr::String(Cow::Borrowed(s.as_ref())),
                tc.get_line(),
                tc.get_span(),
            )),
            _ => Err(io::Error::new(ErrorKind::InvalidInput, "Unexpected token")),
        }
    }

    fn parse_binary(&mut self, left: WithSpan<Expr<'a>>) -> io::Result<WithSpan<Expr<'a>>> {
        let precedence = Precedence::from(self.peek());
        let operator = self.parse_binary_op()?;
        let right = self.parse_expr(precedence)?;
        let line_no = left.get_line();
        let range = left.get_span().start..right.get_span().end;
        Ok(WithSpan::new(
            Expr::Binary(Box::new(left), operator, Box::new(right)),
            line_no,
            range,
        ))
    }

    fn parse_binary_op(&mut self) -> io::Result<WithSpan<BinaryOperator>> {
        let tc = self.advance()?;
        let operator = match tc.get_value() {
            Token::Plus => BinaryOperator::Plus,
            Token::Minus => BinaryOperator::Minus,
            Token::Star => BinaryOperator::Star,
            Token::Slash => BinaryOperator::Slash,
            Token::BangEqual => BinaryOperator::BangEqual,
            Token::EqualEqual => BinaryOperator::EqualEqual,
            Token::Less => BinaryOperator::Less,
            Token::LessEqual => BinaryOperator::LessEqual,
            Token::Greater => BinaryOperator::Greater,
            Token::GreaterEqual => BinaryOperator::GreaterEqual,
            _ => {
                self.error(format!(
                    " {} | Unexpected binary operator got: {:?}",
                    tc.get_line(),
                    tc.get_span()
                ))?;
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "Unexpected binary operator",
                ));
            }
        };
        Ok(WithSpan::new(operator, tc.get_line(), tc.get_span()))
    }
    fn parse_grouping(&mut self) -> io::Result<WithSpan<Expr<'a>>> {
        let left_paren = self.expect(TokenKind::LeftParen)?;
        let expr = self.parse_expr(Precedence::None)?;
        let right_paren = self.expect(TokenKind::RightParen)?;
        let range = left_paren.get_span().start..right_paren.get_span().end;
        let line = left_paren.get_line();
        Ok(WithSpan::new(Expr::Grouping(Box::new(expr)), line, range))
    }
    fn parse_unary(&mut self) -> io::Result<WithSpan<Expr<'a>>> {
        let operator = self.parse_unary_op()?;
        let right = self.parse_expr(Precedence::Unary)?;
        let line = operator.get_line();
        let range = operator.get_span().start..right.get_span().end;
        Ok(WithSpan::new(
            Expr::Unary(operator, Box::new(right)),
            line,
            range,
        ))
    }
    fn parse_unary_op(&mut self) -> io::Result<WithSpan<UnaryOperator>> {
        let tc = self.advance()?;
        match tc.get_value() {
            Token::Minus => Ok(WithSpan::new(
                UnaryOperator::Minus,
                tc.get_line(),
                tc.get_span(),
            )),
            Token::Bang => Ok(WithSpan::new(
                UnaryOperator::Bang,
                tc.get_line(),
                tc.get_span(),
            )),
            _ => {
                self.error(format!(
                    " {} | Unexpected unary operator got: {}",
                    tc.get_line(),
                    tc.get_value()
                ))?;
                Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "unexpected unary operator",
                ))
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
enum Precedence {
    None,
    Assign, // =
    Or,
    And,
    Equality,   // == !=
    Comparison, // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // ()
    List,       // []
    Primary,
}

impl<'a> From<TokenKind> for Precedence {
    fn from(token: TokenKind) -> Precedence {
        match token {
            TokenKind::Equal => Precedence::Assign,
            TokenKind::Or => Precedence::Or,
            TokenKind::And => Precedence::And,
            TokenKind::BangEqual | TokenKind::EqualEqual => Precedence::Equality,
            TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary, // Minus is already specified, but I think this is only for infix ops
            TokenKind::LeftParen => Precedence::Call,
            TokenKind::Dot => Precedence::Call,
            TokenKind::LeftBracket => Precedence::List,
            _ => Precedence::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{Lexer, Token, WithSpan};

    use super::Parser;

    #[test]
    fn test_parse() {
        let lx = Lexer::from("2+3.7");
        let tk = lx.collect::<Vec<WithSpan<Token>>>();
        let mut parser = Parser::<io::Sink>::new(tk.as_slice(), None);
        assert_eq!(
            parser.parse().unwrap().get_value().to_string().as_str(),
            "(+ 2.0 3.7)"
        );
    }
}
