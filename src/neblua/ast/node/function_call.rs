use std::fmt;

use super::{Args, Name, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: Name,
    pub args: Args,
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "FunctionCall")
    }
}

impl Node for FunctionCall {
    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.name, &self.args]
    }
}
