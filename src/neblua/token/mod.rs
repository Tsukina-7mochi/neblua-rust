use std::fmt;

use crate::neblua::util::fmt_u8_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub index: usize,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(index: usize, kind: TokenKind) -> Self {
        Self { index, kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    LiteralString(Vec<u8>),
    Name(Vec<u8>),
    BeginParen,
    EndParen,
    Comma,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            TokenKind::LiteralString(value) => {
                write!(f, "{}: LiteralString({})", self.index, fmt_u8_vec(value))
            }
            TokenKind::Name(value) => write!(f, "{}: Name({})", self.index, fmt_u8_vec(value)),
            TokenKind::BeginParen => write!(f, "{}: '('", self.index),
            TokenKind::EndParen => write!(f, "{}: ')'", self.index),
            TokenKind::Comma => write!(f, "{}: ','", self.index),
        }
    }
}
