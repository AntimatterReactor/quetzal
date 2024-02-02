use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct Location {
    line: usize,
    column: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexicalError {
    InvalidCaret(char),
    InvalidEscape(char),
    InvalidTokenMatch(String),
    SingleLinedLiteralMultiLinedString,

    StringWithoutLiteral,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for LexicalError {}
