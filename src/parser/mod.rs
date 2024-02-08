pub mod ast;
pub mod types;

use ast::Statement;
use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }
}

impl From<Vec<Token>> for Parser {
    fn from(value: Vec<Token>) -> Self {
        Self::new(value)
    }
}
