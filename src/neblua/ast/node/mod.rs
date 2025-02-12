mod args;
mod exp;
mod function_call;
mod literal_string;
mod name;
mod nil;

use std::fmt;

pub use args::Args;
pub use exp::Exp;
pub use function_call::FunctionCall;
pub use literal_string::LiteralString;
pub use name::Name;
#[allow(unused_imports)]
pub use nil::Nil;

pub trait Node: fmt::Display {
    fn children(&self) -> Vec<&dyn Node>;
}
