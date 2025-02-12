pub mod node;

use std::fmt;

use node::Node;

pub struct Ast {
    pub root: Box<dyn node::Node>,
}

impl Ast {
    fn fmt_tree(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        print_tree(f, self.root.as_ref(), 0)
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_tree(f)
    }
}

fn print_tree(f: &mut fmt::Formatter, node: &dyn Node, depth: usize) -> Result<(), fmt::Error> {
    write!(f, "{}", "  ".repeat(depth))?;
    node.fmt(f)?;

    for node in node.children() {
        writeln!(f)?;
        print_tree(f, node, depth + 1)?;
    }

    Ok(())
}
