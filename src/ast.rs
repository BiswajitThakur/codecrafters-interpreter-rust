use std::{
    borrow::Cow,
    fmt::{self, Write},
};

use crate::position::WithSpan;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
}

pub enum Expr<'a> {
    Binary(
        Box<WithSpan<Expr<'a>>>,
        WithSpan<BinaryOperator>,
        Box<WithSpan<Expr<'a>>>,
    ),
    Grouping(Box<WithSpan<Expr<'a>>>),
    Number(f64),
    Boolean(bool),
    Nil,
    This,
    String(Cow<'a, str>),
    Unary(WithSpan<UnaryOperator>, Box<WithSpan<Expr<'a>>>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => f.write_str("nil"),
            Self::This => f.write_str("this"),
            Self::Boolean(v) => write!(f, "{v}"),
            Self::Number(v) => {
                if v.fract() == 0.0 {
                    write!(f, "{v}.0")
                } else {
                    write!(f, "{v}")
                }
            }
            Self::Binary(a, o, b) => {
                f.write_char('(')?;
                f.write_str(match o.get_value() {
                    &BinaryOperator::Plus => "+ ",
                    &BinaryOperator::Minus => "- ",
                })?;
                write!(f, "{} {})", a.get_value(), b.get_value())
            }
            Self::Grouping(g) => write!(f, "(group {})", g.get_value()),
            Self::String(v) => f.write_str(v),
            Self::Unary(u, v) => match u.get_value() {
                UnaryOperator::Bang => write!(f, "(! {})", v.get_value()),
                UnaryOperator::Minus => write!(f, "(- {})", v.get_value()),
            },
        }
    }
}

pub enum Stmt<'a> {
    Expression(Box<WithSpan<Expr<'a>>>),
}

impl fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(v) => write!(f, "{}", v.get_value()),
        }
    }
}

pub type Ast<'a> = Vec<WithSpan<Stmt<'a>>>;
