use std::fmt;

use crate::neblua::util::fmt_u8_vec;

pub trait Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>;

    fn children(&self) -> Vec<&dyn Node>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    pub value: Vec<u8>,
}

impl Node for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "LiteralString('{}')", fmt_u8_vec(&self.value))
    }

    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub value: Vec<u8>,
}

impl Node for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Name('{}')", fmt_u8_vec(&self.value))
    }

    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: Name,
    pub args: Args,
}

impl Node for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "FunctionCall")
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.name, &self.args]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub exp_list: Vec<Exp>,
}

impl Node for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Args")
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.exp_list.iter().map(|exp| exp as &dyn Node).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    LiteralString(LiteralString),
}

impl Node for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Exp")
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Exp::LiteralString(literal_string) => vec![literal_string],
        }
    }
}
