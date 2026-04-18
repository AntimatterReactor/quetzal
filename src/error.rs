//! Every error handling thing in Quetzal
// Copyright (C) 2024  Ezra Alvarion

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
