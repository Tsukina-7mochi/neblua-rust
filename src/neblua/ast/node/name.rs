use std::fmt;

use super::Node;
use crate::neblua::util::fmt_u8_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub value: Vec<u8>,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Name('{}')", fmt_u8_vec(&self.value))
    }
}

impl Node for Name {
    fn children(&self) -> Vec<&dyn Node> {
        Vec::new()
    }
}
