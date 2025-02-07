mod terminal;

pub use terminal::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // terminal tokens
    LiteralString(LiteralString),
    Name(Name),

    // non-terminal tokens
    FunctionCall(FunctionCall),
    Args(Args),
    Exp(Exp),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: Name,
    pub args: Args,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Args {
    pub exp_list: Vec<Exp>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Exp {
    LiteralString(LiteralString),
}
