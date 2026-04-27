// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Everything regarding the tokens themselves

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
    At,
    VerticalBar, //'|'

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
    LessThanEqual,
    GreaterThanEqual,

    ThinArrow,
    FatArrow,

    And,
    Or,
    Not,

    If,
    Else,
    Switch,
    Case,
    Default,

    Loop,
    While,
    For,
    In,
    Break,
    Continue,

    Function,
    LetDecl,
    ConstDecl,
    Import,
    Return,
    Enum,
    As,

    Indent,
    Dedent,
    
    Boolean(bool),
    Char(char),
    String(Box<str>),
    Number(Box<str>),
    Identifier(Box<str>),
}

impl TokenType {
    pub fn from_keyword(s: &str) -> Result<Self, LexicalError> {
        match s {
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "switch" => Ok(Self::Switch),
            "case" => Ok(Self::Case),
            "default" => Ok(Self::Default),
            "loop" => Ok(Self::Loop),
            "while" => Ok(Self::While),
            "for" => Ok(Self::For),
            "in" => Ok(Self::In),
            "break" => Ok(Self::Break),
            "continue" => Ok(Self::Continue),
            "fn" => Ok(Self::Function),
            "let" => Ok(Self::LetDecl),
            "const" => Ok(Self::ConstDecl),
            "return" => Ok(Self::Return),
            "import" => Ok(Self::Import),
            "enum" => Ok(Self::Enum),
            "as" => Ok(Self::As),
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
            Self::Equal => "==", Self::NotEqual => "!=",
            Self::LessThanEqual => "<=", Self::GreaterThanEqual => ">=",
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
    pub pos: Location,
}
