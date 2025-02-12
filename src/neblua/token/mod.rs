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
    Nil,
    BeginParen,
    EndParen,
    Comma,
    EndOfInput,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.index, self.kind)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::LiteralString(value) => {
                write!(f, "LiteralString('{}')", fmt_u8_vec(value))
            }
            TokenKind::Name(value) => write!(f, "Name('{}')", fmt_u8_vec(value)),
            TokenKind::Nil => write!(f, "nil"),
            TokenKind::BeginParen => write!(f, "'('"),
            TokenKind::EndParen => write!(f, "')'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::EndOfInput => write!(f, "EOF"),
        }
    }
}
