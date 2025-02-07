#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LiteralString(Vec<u8>),
    Name(Vec<u8>),
    BeginParen,
    EndParen,
    Comma,
}
