#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalToken {
    LiteralString(LiteralString),
    Name(Name),
    BeginParen,
    EndParen,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    value: Vec<u8>,
}

impl LiteralString {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    value: Vec<u8>,
}

impl Name {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
}
