// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
//! A wise language

mod lexer;
pub use lexer::Lexer;

mod token;
pub use token::{Token, TokenType};

mod parser;
pub use parser::{ast, Parser};

pub mod codegen;

pub mod error;
