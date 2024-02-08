use std::collections::HashMap;

#[derive(Debug)]
pub enum Statement {
    Function,
    Declaration,
    If,
    Else,
    Attribute,
    Block,
}

#[derive(Debug)]
pub enum Expression {
    Block
}

