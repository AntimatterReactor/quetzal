use super::types::{ImportType, Type};
use crate::token::{Token, TokenType};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    VarDecl(Vec<(String, Expression)>),
    If(Expression, Vec<Statement>, Option<Box<Statement>>),
    While(Expression, Expression),
    Break,
    Continue,
    Return(Option<Expression>),
    Function(
        Option<Expression>,
        Box<str>,
        Option<Vec<Expression>>,
        Expression,
        Box<str>,
    ),
    Require(Token, ImportType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Assign(Box<Expression>, Box<Expression>),
    Binary(Box<Expression>, TokenType, Box<Expression>),
    Paren(Box<Expression>),
    Tuple(Vec<Expression>),
    Array(Vec<Expression>),
    Literal(Type), // TODO: add actual values to literal (currently only the type)
    Unary(TokenType, Box<Expression>),
    Variable(Box<str>),
    Block(Vec<Statement>),
    Logical(Box<Expression>, Token, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    Call(
        Box<Expression>,
        Box<str>,
        Vec<Expression>,
        HashMap<Box<str>, Expression>,
    ),
    IfExpr(
        Box<Expression>,
        Vec<Statement>,
        Vec<(Expression, Vec<Statement>)>,
        Option<Vec<Statement>>,
    ),
    Get(Box<Expression>, Token, Box<Expression>),
    Set(Box<Expression>, Token, Box<Expression>, Box<Expression>),
    Prop(Box<Expression>, Token),
    Map(Vec<(Expression, Expression)>),
    Range(Box<Expression>, Token, Box<Expression>, bool),
}
