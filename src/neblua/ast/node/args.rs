use std::fmt;

use super::{Exp, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub exp_list: Vec<Exp>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Args")
    }
}

impl Node for Args {
    fn children(&self) -> Vec<&dyn Node> {
        self.exp_list.iter().map(|exp| exp as &dyn Node).collect()
    }
}
