use std::{error::Error, fmt::{Display, self, Formatter}};

#[derive(Debug)]
pub struct Location {
    line: usize,
    column: usize
}

#[derive(Debug)]
pub enum LexicalError {
    InvalidCaret(char),
    InvalidEscape(char),
    SyntaxError
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for LexicalError {}
