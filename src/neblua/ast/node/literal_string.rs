use std::fmt;

use super::Node;
use crate::neblua::util::fmt_u8_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    pub value: Vec<u8>,
}

impl fmt::Display for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "LiteralString('{}')", fmt_u8_vec(&self.value))
    }
}

impl Node for LiteralString {
    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
    }
}
