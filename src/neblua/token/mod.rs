mod terminal;

pub use terminal::*;

pub enum Token {
    // terminal tokens
    LiteralString(LiteralString),
    Name(Name),

    // non-terminal tokens
    FunctionCall(Name, Args),
    Args(Args),
    ExpList(ExpList),
    Exp(Exp),
}

pub struct Args {
    exp_list: ExpList,
}

pub struct ExpList {
    exp_list: Vec<Exp>,
}

pub enum Exp {
    LiteralString(LiteralString),
}
