use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    line: usize,
    column: usize,
    cause: ErrorCause,
}

#[derive(Debug, PartialEq)]
pub enum ErrorCause {
    UnexpectedCharacter(char),
    UnexpectedEndOfInput,
}

impl Error {
    pub fn new_unexpected_character(line: usize, column: usize, c: char) -> Self {
        Self {
            line,
            column,
            cause: ErrorCause::UnexpectedCharacter(c),
        }
    }

    pub fn new_unexpected_end_of_input(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            cause: ErrorCause::UnexpectedEndOfInput,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.cause {
            ErrorCause::UnexpectedCharacter(c) => write!(
                f,
                "Unexpected character '{}' at line {} column {}",
                c, self.line, self.column
            ),
            ErrorCause::UnexpectedEndOfInput => write!(
                f,
                "Unexpected end of input at line {} column {}",
                self.line, self.column
            ),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
