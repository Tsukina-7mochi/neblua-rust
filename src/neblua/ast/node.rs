use std::fmt;

use crate::neblua::util::fmt_u8_vec;

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Node::LiteralString(literal_string) => literal_string.fmt_tree(f, 0, ""),
            Node::Name(name) => name.fmt_tree(f, 0, ""),
            Node::FunctionCall(function_call) => function_call.fmt_tree(f, 0, ""),
            Node::Args(args) => args.fmt_tree(f, 0, ""),
            Node::Exp(exp) => exp.fmt_tree(f, 0, ""),
        }
    }
}

trait FmtTree {
    fn fmt_tree(
        &self,
        f: &mut fmt::Formatter,
        depth: usize,
        prefix: &str,
    ) -> Result<(), fmt::Error>;
}

impl FmtTree for LiteralString {
    fn fmt_tree(
        &self,
        f: &mut fmt::Formatter,
        depth: usize,
        prefix: &str,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}LiteralString: {}\n",
            "  ".repeat(depth),
            prefix,
            fmt_u8_vec(&self.value),
        )
    }
}

impl FmtTree for Name {
    fn fmt_tree(
        &self,
        f: &mut fmt::Formatter,
        depth: usize,
        prefix: &str,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}Name: {}\n",
            "  ".repeat(depth),
            prefix,
            fmt_u8_vec(&self.value),
        )
    }
}

impl FmtTree for FunctionCall {
    fn fmt_tree(
        &self,
        f: &mut fmt::Formatter,
        depth: usize,
        prefix: &str,
    ) -> Result<(), fmt::Error> {
        write!(f, "{}{}FunctionCall\n", "  ".repeat(depth), prefix,)?;
        self.name.fmt_tree(f, depth + 1, "Name: ")?;
        self.args.fmt_tree(f, depth + 1, "Args: ")
    }
}

impl FmtTree for Args {
    fn fmt_tree(
        &self,
        f: &mut fmt::Formatter,
        depth: usize,
        prefix: &str,
    ) -> Result<(), fmt::Error> {
        write!(f, "{}{}Args\n", "  ".repeat(depth), prefix)?;
        for exp in &self.exp_list {
            exp.fmt_tree(f, depth + 1, "Item: ")?;
        }
        Ok(())
    }
}

impl FmtTree for Exp {
    fn fmt_tree(
        &self,
        f: &mut fmt::Formatter,
        depth: usize,
        prefix: &str,
    ) -> Result<(), fmt::Error> {
        write!(f, "{}{}Exp\n", "  ".repeat(depth), prefix)?;
        match self {
            Exp::LiteralString(literal_string) => literal_string.fmt_tree(f, depth + 1, "Value: "),
        }?;
        Ok(())
    }
}
