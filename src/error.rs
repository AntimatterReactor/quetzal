// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Every error handling thing in Quetzal

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let line = self.line;
        let column = self.column;
        write!(f, "line: {line}, column: {column}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexicalErrorType {
    InvalidCaret(char),
    InvalidEscape(char),
    InvalidTokenMatch(String),

    UnknownCharacter(char),
    
    UnclosedStringLiteral,
    UnescapedNewlineInString,
    
    EmptyCharLiteral,
    UnclosedCharLiteral,
    UnescapedNewlineInChar,

    UnexpectedRightSymmetric(char),
}

#[derive(Debug)]
pub struct LexicalError {
    err_type: LexicalErrorType,
    beg_loc: Location,
    end_loc: Option<Location>
}

impl LexicalError {
    pub fn get_type(&self) -> &LexicalErrorType {
        &self.err_type
    }
}

impl From<(LexicalErrorType, Location)> for LexicalError {
    fn from(source: (LexicalErrorType, Location)) -> Self {
        Self {
            err_type: source.0,
            beg_loc:  source.1,
            end_loc:  None,
        }
    }
}

impl From<(LexicalErrorType, Location, Location)> for LexicalError {
    fn from(source: (LexicalErrorType, Location, Location)) -> Self {
        Self {
            err_type: source.0,
            beg_loc:  source.1,
            end_loc:  Some(source.2),
        }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        use LexicalErrorType::*;

        let loc = self.beg_loc;

        match self.err_type {
            InvalidCaret(c) => write!(f, "Invalid caret character '^{c}' found at {loc}"),
            InvalidEscape(c) => write!(f, "Invalid escape character '\\{c}' found at {loc}"),
            // InvalidTokenMatch(s) => write!(f, "Invalid"), //TODO: figure out if this enum is ever called

            UnknownCharacter(c) => write!(f, "Unknown character '{c}' found at {loc}"),
            
            UnclosedStringLiteral => {
                let end = self.end_loc.expect("for there to be a valid begin of string");
                write!(f, "Unclosed string literal found beginning at {loc}; terminating at {end}")
            }
            UnescapedNewlineInString => {
                let end = self.end_loc.expect("for there to be a valid begin of string");
                write!(f, "Unescaped newline found at {end}; with string beginning at {loc}")
            }
            UnexpectedRightSymmetric(c) => write!(f, "Unexpected '{c}' without left pair found at {loc}"),
            _ => write!(f, "{self:?}")
        }
    }
}

impl Error for LexicalError {}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    None,
    TrailingIf,
    TrailingFunction,
    MalformedFunction,
    UnclosedBlock,
    UnclosedParen,
    
    TokenGetFailure,
    TokenNextFailure
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ParseError {}
