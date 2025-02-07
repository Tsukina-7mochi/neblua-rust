#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    LiteralString(LiteralString),
    Name(Name),

    // non-terminal tokens
    FunctionCall(FunctionCall),
    Args(Args),
    Exp(Exp),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LiteralString {
    pub value: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    pub value: Vec<u8>,
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
