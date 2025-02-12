use std::fmt;

use super::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nil {}

impl fmt::Display for Nil {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "nil")
    }
}

impl Node for Nil {
    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
    }
}

pub const NIL: Nil = Nil {};
