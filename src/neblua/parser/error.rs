use std::error;
use std::fmt;

use crate::neblua::token::Token;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub index: usize,
    pub cause: ErrorCause,
}

#[derive(Debug, PartialEq)]
pub enum ErrorCause {
    UnexpectedToken(Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.index, self.cause)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::UnexpectedToken(token) => {
                write!(f, "Unexpected token {}", token)
            }
        }
    }
}
