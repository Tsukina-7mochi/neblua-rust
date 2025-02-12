use std::fmt;

use super::{nil::NIL, LiteralString, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    LiteralString(LiteralString),
    Nil,
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Exp")
    }
}

impl Node for Exp {
    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Exp::LiteralString(literal_string) => vec![literal_string],
            Exp::Nil => vec![&NIL],
        }
    }
}
