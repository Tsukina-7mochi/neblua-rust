use std::fmt;

use crate::neblua::util::fmt_u8_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LiteralString(Vec<u8>),
    Name(Vec<u8>),
    BeginParen,
    EndParen,
    Comma,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LiteralString(value) => write!(f, "LiteralString({})", fmt_u8_vec(value)),
            Token::Name(value) => write!(f, "Name({})", fmt_u8_vec(value)),
            Token::BeginParen => write!(f, "'('"),
            Token::EndParen => write!(f, "')'"),
            Token::Comma => write!(f, "','"),
        }
    }
}
