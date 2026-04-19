// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Every error handling thing in Quetzal

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexicalError {
    InvalidCaret(char),
    InvalidEscape(char),
    InvalidTokenMatch(String),

    UnknownCharacter(char),
    
    SingleLinedLiteralMultiLinedString,
    StringWithoutLiteral,

    UnexpectedRightSymmetric,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::InvalidTokenMatch(s) = self {
            return write!(f, "Invalid Token '{s}'");
        }

        let c = match self {
            Self::InvalidCaret(a) => a,
            Self::InvalidEscape(e) => e,
            Self::UnknownCharacter(x) => x,
            _ => &'*'
        };
        write!(f, "{self:?}")
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
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ParseError {}
