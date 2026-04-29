// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
pub mod ast;
pub mod types;

use crate::{
    codegen::Codegen,
    error::{LexicalErrorType, ParseError},
    token::{Token, TokenType},
    Lexer,
};
use ast::{Expression, Statement};

#[derive(Debug, PartialEq, Eq)]
pub struct Parser {
    tokens: Box<[Token]>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Box<[Token]>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn from_lexer(mut lexer: Lexer) -> Result<Self, LexicalErrorType> {
        let tokens = lexer.lexicalize()?;
        Ok(Self::new(tokens.into_boxed_slice()))
    }

    /// Entry point — parse full token stream into top-level [`Statement`].
    pub fn construct(&mut self) -> Result<Box<Statement>, ParseError> {
        let mut stmts = Vec::<Statement>::new();
        while self.get().is_some() {
            stmts.push(self.parse_statement()?);
        }
        Ok(Box::new(Statement::Block(stmts)))
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.get().ok_or(ParseError::None)?.t {
            TokenType::Function  => self.parse_function(),
            TokenType::If        => self.parse_if_statement(),
            TokenType::While     => self.parse_while_statement(),
            TokenType::For       => self.parse_for_statement(),
            TokenType::Loop      => self.parse_loop_statement(),
            TokenType::Return    => self.parse_return_statement(),
            TokenType::LetDecl   => self.parse_let_statement(),
            TokenType::ConstDecl => self.parse_const_statement(),
            _                    => self.parse_expr_statement(),
        }
    }

    // ── Functions ────────────────────────────────────────────────────────────

    /// `fn [expr] ident (params) -> type : block`
    pub fn parse_function(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'fn'
        let attach = match self.next().ok_or(ParseError::TrailingFunction)?.t {
            TokenType::LeftBracket => match self.get().ok_or(ParseError::TrailingFunction)?.t {
                TokenType::RightBracket => None,
                _ => Some(self.parse_expression()?),
            },
            TokenType::Identifier(_) => { self.backtrack(); None }
            _ => return Err(ParseError::MalformedFunction),
        };
        let name = self.next_identifier_or(ParseError::MalformedFunction)?;
        let args = match self.peek().ok_or(ParseError::TrailingFunction)?.t {
            TokenType::LeftParen => {
                self.next(); // '('
                match self.get().ok_or(ParseError::TrailingFunction)?.t {
                    TokenType::RightParen => { self.next(); None }
                    _ => Some(self.parse_param_list()?),
                }
            }
            _ => None,
        };
        let rettype = if self.current_in_then_next(&[TokenType::ThinArrow]).unwrap_or(false) {
            self.next_identifier_or(ParseError::MalformedFunction)?
        } else {
            "void".into()
        };
        let body = self.parse_block()?;
        Ok(Statement::Function(attach, name, args, body, rettype))
    }

    /// Parse `ident: type` params until `)`
    fn parse_param_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut params = Vec::new();
        loop {
            params.push(self.parse_expression()?);
            match self.get().ok_or(ParseError::UnclosedParen)?.t {
                TokenType::Comma      => { self.next(); }
                TokenType::RightParen => { self.next(); break; }
                _                     => return Err(ParseError::MalformedFunction),
            }
        }
        Ok(params)
    }

    // ── Control flow ─────────────────────────────────────────────────────────

    /// `if expr : block [else : block]`
    pub fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'if'
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        let else_block = if self.current_in(&[TokenType::Else]).unwrap_or(false) {
            self.next(); // 'else'
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Statement::If(condition, then_block, else_block))
    }

    /// `while expr : block`
    pub fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'while'
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Statement::While(condition, body))
    }

    /// `for ident in expr : block`
    pub fn parse_for_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'for'
        let var = self.next_identifier_or(ParseError::MalformedFunction)?; // TODO: dedicated error variant
        // consume 'in' identifier
        self.next_identifier_or(ParseError::MalformedFunction)?;
        let iter = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Statement::For(var, iter, body))
    }

    /// `loop : block`
    pub fn parse_loop_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'loop'
        let body = self.parse_block()?;
        Ok(Statement::Loop(body))
    }

    // ── Simple statements ────────────────────────────────────────────────────

    /// `return expr`
    pub fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'return'
        let val = self.parse_expression()?;
        Ok(Statement::Return(val))
    }

    /// `let ident [: type] = expr`
    pub fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'let'
        let name = self.next_identifier_or(ParseError::MalformedFunction)?;
        let ty = if self.current_in(&[TokenType::Colon]).unwrap_or(false) {
            self.next(); // ':'
            Some(self.next_identifier_or(ParseError::MalformedFunction)?)
        } else {
            None
        };
        self.current_in_then_next(&[TokenType::Assign]).ok_or(ParseError::MalformedFunction)?;
        let val = self.parse_expression()?;
        Ok(Statement::Let(name, ty, val))
    }

    /// `const ident [: type] = expr`
    pub fn parse_const_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'const'
        let name = self.next_identifier_or(ParseError::MalformedFunction)?;
        let ty = if self.current_in(&[TokenType::Colon]).unwrap_or(false) {
            self.next();
            Some(self.next_identifier_or(ParseError::MalformedFunction)?)
        } else {
            None
        };
        self.current_in_then_next(&[TokenType::Assign]).ok_or(ParseError::MalformedFunction)?;
        let val = self.parse_expression()?;
        Ok(Statement::Const(name, ty, val))
    }

    /// Bare expression as statement (assignments, calls, etc.)
    pub fn parse_expr_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression()?;
        Ok(Statement::Expr(expr))
    }

    // ── Expressions ──────────────────────────────────────────────────────────

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assign()
    }

    pub fn parse_assign(&mut self) -> Result<Expression, ParseError> {
        let lhs = self.parse_or()?;
        let assign_ops = [
            TokenType::Assign, TokenType::AssignPlus, TokenType::AssignMinus,
            TokenType::AssignMul, TokenType::AssignDiv, TokenType::AssignModulo,
        ];
        if self.current_in(&assign_ops).unwrap_or(false) {
            let op = self.next().unwrap().t.clone();
            let rhs = self.parse_assign()?; // right-assoc
            return Ok(Expression::Assign(Box::new(lhs), Box::new(rhs)));
        }
        Ok(lhs)
    }

    pub fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut lhs = self.parse_and()?;
        while self.current_in(&[TokenType::Or]).unwrap_or(false) {
            self.next();
            let rhs = self.parse_and()?;
            lhs = Expression::BinOp(Box::new(lhs), TokenType::Or, Box::new(rhs));
        }
        Ok(lhs)
    }

    pub fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut lhs = self.parse_not()?;
        while self.current_in(&[TokenType::And]).unwrap_or(false) {
            self.next();
            let rhs = self.parse_not()?;
            lhs = Expression::BinOp(Box::new(lhs), TokenType::And, Box::new(rhs));
        }
        Ok(lhs)
    }

    pub fn parse_not(&mut self) -> Result<Expression, ParseError> {
        if self.current_in(&[TokenType::Not]).unwrap_or(false) {
            self.next();
            return Ok(Expression::UnaryOp(TokenType::Not, Box::new(self.parse_not()?)));
        }
        self.parse_cmp()
    }

    pub fn parse_cmp(&mut self) -> Result<Expression, ParseError> {
        let cmp_ops = [
            TokenType::Equal, TokenType::NotEqual, TokenType::LeftAngle,
            TokenType::RightAngle, TokenType::LessThanEqual, TokenType::GreaterThanEqual,
        ];
        let mut lhs = self.parse_add()?;
        while self.current_in(&cmp_ops).unwrap_or(false) {
            let op = self.next().unwrap().t.clone();
            let rhs = self.parse_add()?;
            lhs = Expression::BinOp(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    pub fn parse_add(&mut self) -> Result<Expression, ParseError> {
        let mut lhs = self.parse_mul()?;
        while self.current_in(&[TokenType::Plus, TokenType::Minus]).unwrap_or(false) {
            let op = self.next().unwrap().t.clone();
            let rhs = self.parse_mul()?;
            lhs = Expression::BinOp(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    pub fn parse_mul(&mut self) -> Result<Expression, ParseError> {
        let mut lhs = self.parse_unary()?;
        while self.current_in(&[TokenType::Mul, TokenType::Div, TokenType::Modulo, TokenType::DivMod]).unwrap_or(false) {
            let op = self.next().unwrap().t.clone();
            let rhs = self.parse_unary()?;
            lhs = Expression::BinOp(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    pub fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        let unary_ops = [TokenType::Minus, TokenType::Tilde, TokenType::Mul];
        if self.current_in(&unary_ops).unwrap_or(false) {
            let op = self.next().unwrap().t.clone();
            return Ok(Expression::UnaryOp(op, Box::new(self.parse_unary()?)));
        }
        self.parse_call()
    }

    /// Postfix: calls, indexing, field access, scope resolution
    pub fn parse_call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.get().map(|t| &t.t) {
                Some(TokenType::LeftParen) => {
                    self.next();
                    let args = self.parse_arg_list()?;
                    expr = Expression::Call(Box::new(expr), args);
                }
                Some(TokenType::LeftBracket) => {
                    self.next();
                    let idx = self.parse_expression()?;
                    self.current_in_then_next(&[TokenType::RightBracket])
                        .ok_or(ParseError::UnclosedBlock)?;
                    expr = Expression::Index(Box::new(expr), Box::new(idx));
                }
                Some(TokenType::Dot) => {
                    self.next();
                    let field = self.next_identifier_or(ParseError::MalformedFunction)?;
                    expr = Expression::Field(Box::new(expr), field);
                }
                Some(TokenType::Scope) => {
                    self.next();
                    let seg = self.next_identifier_or(ParseError::MalformedFunction)?;
                    expr = Expression::Scope(Box::new(expr), seg);
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        if self.current_in(&[TokenType::RightParen]).unwrap_or(false) {
            self.next();
            return Ok(args);
        }
        loop {
            // closure: |params| => expr
            if self.current_in(&[TokenType::Or]).unwrap_or(false) {
                args.push(self.parse_closure()?);
            } else {
                args.push(self.parse_expression()?);
            }
            match self.get().ok_or(ParseError::UnclosedParen)?.t {
                TokenType::Comma      => { self.next(); }
                TokenType::RightParen => { self.next(); break; }
                _                     => return Err(ParseError::UnclosedParen),
            }
        }
        Ok(args)
    }

    /// `|params| => expr`
    pub fn parse_closure(&mut self) -> Result<Expression, ParseError> {
        self.next(); // '|'
        let mut params = Vec::new();
        while !self.current_in(&[TokenType::Or]).unwrap_or(false) {
            params.push(self.next_identifier_or(ParseError::MalformedFunction)?);
            if self.current_in(&[TokenType::Comma]).unwrap_or(false) { self.next(); }
        }
        self.next(); // closing '|'
        self.current_in_then_next(&[TokenType::FatArrow]).ok_or(ParseError::MalformedFunction)?;
        let body = self.parse_expression()?;
        Ok(Expression::Closure(params, Box::new(body)))
    }

    pub fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.get().ok_or(ParseError::TokenGetFailure)?.t.clone() {
            TokenType::Identifier(_)  => Ok(Expression::Ident(
                self.next_identifier_or(ParseError::TokenNextFailure)?
            )),
            TokenType::Number(n)      => { self.next(); Ok(Expression::Number(n)) }
            TokenType::String(s)      => { self.next(); Ok(Expression::Str(s)) }
            TokenType::Boolean(b)     => { self.next(); Ok(Expression::Bool(b)) }
            TokenType::LeftParen      => self.parse_paren(),
            TokenType::LeftBracket    => self.parse_array_or_tuple(),
            TokenType::LeftCurl       => self.parse_block(),
            other                         => {
                eprintln!("{self:#?}");
                eprintln!("found: '{other:?}'");
                Err(ParseError::None)
            }
        }
    }

    // ── Existing helpers (unchanged) ─────────────────────────────────────────

    pub fn parse_paren(&mut self) -> Result<Expression, ParseError> {
        self.next(); // '('
        let expression = Box::new(self.parse_expression()?);
        self.current_in_then_next(&[TokenType::RightParen])
            .filter(|&t| t)
            .ok_or(ParseError::UnclosedParen)
            .and(Ok(Expression::Paren(expression)))
    }

    pub fn parse_block(&mut self) -> Result<Expression, ParseError> {
        let use_indent = match self.next().ok_or(ParseError::None)?.t {
            TokenType::LeftCurl => false, // '{'
            TokenType::Colon => true,     // ':'
            _ => return Err(ParseError::MalformedFunction)
        };
        let mut statements = Vec::<Statement>::new();
        while !self.current_in(&[TokenType::RightCurl]).ok_or(ParseError::UnclosedBlock)? {
            statements.push(self.parse_statement()?);
        }
        self.next(); // '}'
        Ok(Expression::Block(statements))
    }

    pub fn parse_array_or_tuple(&mut self) -> Result<Expression, ParseError> {
        let beg = self.next()
            .filter(|t| [TokenType::LeftBracket, TokenType::LeftParen].contains(&t.t))
            .expect("'[' or '('")
            .t.clone();
        let close = match beg {
            TokenType::LeftBracket => TokenType::RightBracket,
            TokenType::LeftParen   => TokenType::RightParen,
            _ => unreachable!(),
        };
        let mut expressions = Vec::new();
        while !self.current_in(&[close.clone()]).ok_or(ParseError::UnclosedBlock)? {
            expressions.push(self.parse_expression()?);
            if self.current_in(&[TokenType::Comma]).ok_or(ParseError::UnclosedBlock)? {
                self.current += 1;
            }
        }
        self.next(); // closing bracket
        Ok(match beg {
            TokenType::LeftBracket => Expression::Array(expressions),
            TokenType::LeftParen   => Expression::Tuple(expressions),
            _ => unreachable!(),
        })
    }

    // ── Cursor helpers ───────────────────────────────────────────────────────

    fn get(&self) -> Option<&Token> { self.tokens.get(self.current) }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.current + 1) }

    fn current_in(&self, t: &[TokenType]) -> Option<bool> {
        Some(t.contains(&self.get()?.t))
    }

    fn next(&mut self) -> Option<&Token> {
        let r = self.tokens.get(self.current);
        self.current += 1;
        r
    }

    fn backtrack(&mut self) { self.current -= 1; }

    fn current_in_then_next(&mut self, t: &[TokenType]) -> Option<bool> {
        Some(t.contains(&self.next()?.t))
    }

    fn next_identifier_or(&mut self, error: ParseError) -> Result<Box<str>, ParseError> {
        self.next()
            .map(|t| match &t.t {
                TokenType::Identifier(s) => Some(s.clone()),
                _ => None,
            })
            .flatten()
            .ok_or(error)
    }
}

impl TryInto<Codegen> for Parser {
    type Error = ParseError;
    fn try_into(mut self) -> Result<Codegen, Self::Error> {
        self.construct().map(|x| x.into())
    }
}
