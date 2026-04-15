//! Everything regarding the tokens themselves
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

use crate::error::{LexicalError, Location};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum TokenType {
    #[default]
    None,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurl,
    RightCurl,
    LeftAngle,
    RightAngle,

    Semicolon,
    Comma,
    Dot,
    Colon,
    Scope,
    Tilde,
    Tick,

    Plus,
    Minus,
    Mul,
    Div,
    Modulo,
    DivMod,

    Assign,
    AssignPlus,
    AssignMinus,
    AssignMul,
    AssignDiv,
    AssignModulo,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    ThinArrow,
    FatArrow,

    And,
    Or,
    Not,

    If,
    Else,

    Loop,
    While,
    For,

    Function,
    LetDecl,
    ConstDecl,
    Return,
    Defer,

    
    Boolean(bool),
    String(Box<str>),
    Number(Box<str>),
    Identifier(Box<str>),
}

impl TokenType {
    pub fn from_op(s: &str) -> Result<Self, LexicalError> {
        match s {
            "(" => Ok(Self::LeftParen),
            ")" => Ok(Self::RightParen),
            "[" => Ok(Self::LeftBracket),
            "]" => Ok(Self::RightBracket),
            "{" => Ok(Self::LeftCurl),
            "}" => Ok(Self::RightCurl),
            "<" => Ok(Self::LeftAngle),
            ">" => Ok(Self::RightAngle),
            ";" => Ok(Self::Semicolon),
            "," => Ok(Self::Comma),
            "." => Ok(Self::Dot),
            ":" => Ok(Self::Colon),
            "::" => Ok(Self::Scope),
            "~" => Ok(Self::Tilde),
            "`" => Ok(Self::Tick),
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "%" => Ok(Self::Modulo),
            "/%" => Ok(Self::DivMod),
            "=" => Ok(Self::Assign),
            "+=" => Ok(Self::AssignPlus),
            "-=" => Ok(Self::AssignMinus),
            "*=" => Ok(Self::AssignMul),
            "/=" => Ok(Self::AssignDiv),
            "%=" => Ok(Self::AssignModulo),
            "?=" => Ok(Self::Equal),
            "?!=" => Ok(Self::NotEqual),
            "?<" => Ok(Self::LessThan),
            "?>" => Ok(Self::GreaterThan),
            "?<=" => Ok(Self::LessThanEqual),
            "?>=" => Ok(Self::GreaterThanEqual),
            "->" => Ok(Self::ThinArrow),
            "=>" => Ok(Self::FatArrow),
            _ => Err(LexicalError::InvalidTokenMatch(s.to_string())),
        }
    }

    pub fn from_keyword(s: &str) -> Result<Self, LexicalError> {
        match s {
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "loop" => Ok(Self::Loop),
            "while" => Ok(Self::While),
            "for" => Ok(Self::For),
            "fn" => Ok(Self::Function),
            "let" => Ok(Self::LetDecl),
            "const" => Ok(Self::ConstDecl),
            "return" => Ok(Self::Return),
            "defer" => Ok(Self::Defer),
            "true" => Ok(Self::Boolean(true)),
            "false" => Ok(Self::Boolean(false)),
            _ => Err(LexicalError::InvalidTokenMatch(s.to_string())),
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Plus => "+", Self::Minus => "-", Self::Mul => "*",
            Self::Div => "/", Self::Modulo => "%", Self::DivMod => "/%",
            Self::And => "and", Self::Or => "or", Self::Not => "not",
            Self::Equal => "?=", Self::NotEqual => "?!=",
            Self::LessThan => "?<", Self::GreaterThan => "?>",
            Self::LessThanEqual => "?<=", Self::GreaterThanEqual => "?>=",
            Self::Assign => "=", Self::AssignPlus => "+=",
            Self::AssignMinus => "-=", Self::AssignMul => "*=",
            Self::AssignDiv => "/=", Self::AssignModulo => "%=",
            Self::Tilde => "~",
            other => return write!(f, "{other:?}"),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Token {
    pub t: TokenType,
    pub pos: Location, // TODO: pos should be a tuple of (usize, usize), and has line and column
}
