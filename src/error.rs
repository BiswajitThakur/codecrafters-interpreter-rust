use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoxError {
    InvalidChar(usize, char),
    UnterminatedStr(usize),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChar(line, c) => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, c)
            }
            Self::UnterminatedStr(line) => write!(f, "[line {}] Error: Unterminated string.", line),
        }
    }
}

impl From<(usize, char)> for LoxError {
    fn from((line, c): (usize, char)) -> Self {
        Self::InvalidChar(line, c)
    }
}
