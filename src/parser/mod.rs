pub mod ast;
pub mod types;

use crate::{
    codegen::Codegen,
    error::{LexicalError, ParseError},
    token::{Token, TokenType},
    Lexer,
};
use ast::{Statement, Expression};

#[derive(Debug, PartialEq, Eq)]
pub struct Parser {
    tokens: Box<[Token]>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Box<[Token]>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn from_lexer(mut lexer: Lexer) -> Result<Self, LexicalError> {
        let tokens = lexer.lexicalize()?;
        Ok(Self::new(tokens.into_boxed_slice()))
    }

    pub fn construct(&mut self) -> Result<Box<Statement>, ParseError> {
        todo!()
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    /// Parse a funtion into [`Statement::Function`]
    ///
    /// # Context Free Grammar
    ///
    /// The CFG for a function is as follows:
    ///
    /// ```text
    /// function ::= 'fn' ( '[' expr ']' )? ident ( '(' (expr ',' )* expr ')' )? block ( '->' ident )?
    /// ```
    ///
    /// # Example
    /// TODO
    ///
    pub fn parse_function(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'fn'
        let attach = match self.next().ok_or(ParseError::TrailingFunction)?.t {
            TokenType::LeftBracket => match self.get().ok_or(ParseError::TrailingFunction)?.t {
                TokenType::RightBracket => None,
                _ => Some(self.parse_expression()?), // TODO: what about `]` after expr?
            },
            TokenType::Identifier(_) => {
                self.backtrack();
                None
            }
            _ => return Err(ParseError::MalformedFunction),
        };
        let name = self.next_identifier_or(ParseError::MalformedFunction)?;
        let args = match self.next().ok_or(ParseError::TrailingFunction)?.t {
            TokenType::LeftParen => match self.get().ok_or(ParseError::TrailingFunction)?.t {
                TokenType::RightParen => None,
                _ => Some(match self.parse_array_or_tuple()? {
                    Expression::Tuple(x) => x,
                    _ => unreachable!(),
                }),
            },
            TokenType::Identifier(_) => None,
            _ => return Err(ParseError::MalformedFunction),
        };
        let body = self.parse_block()?;
        let rettype = if self
            .current_in_then_next(&[TokenType::ThinArrow])
            .unwrap_or(false)
        {
            self.next_identifier_or(ParseError::MalformedFunction)?
        } else {
            "void".into()
        };
        Ok(Statement::Function(attach, name, args, body, rettype))
    }

    ///
    pub fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.next(); // 'if'
        let condition = self.parse_expression()?;
        self.current_in_then_next(&[TokenType::RightParen])
            .filter(|&t| t); //TODO
        let block = self.parse_expression();
        todo!()
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        todo!()
    }

    pub fn parse_paren(&mut self) -> Result<Expression, ParseError> {
        self.next(); // '('
        let expression = Box::<Expression>::new(self.parse_expression()?);
        self.current_in_then_next(&[TokenType::RightParen])
            .filter(|&t| t)
            .ok_or(ParseError::UnclosedParen)
            .and(Ok(Expression::Paren(expression))) // ')'
    }

    pub fn parse_block(&mut self) -> Result<Expression, ParseError> {
        self.next(); // '{'
        let mut statements = Vec::<Statement>::new();
        while !self
            .current_in(&[TokenType::RightCurl])
            .ok_or(ParseError::UnclosedBlock)?
        {
            statements.push(self.parse_statement()?);
        }
        self.next(); // '}'
        Ok(Expression::Block(statements))
    }

    /// Parse an array into [`Expression::Array`] and tuple into [`Expression::Tuple`]
    /// 
    /// # Context Free Grammar
    /// 
    /// The CFG for an array or tuple is as follows:
    /// 
    /// //TODO: add CFG for array and tuple
    /// 
    /// # Example //TODO: make doctest work (tuple and array accept expression not literals)
    /// 
    /// Tuple:
    /// 
    /// ```rust
    /// # use libquetzal::{Lexer, Parser, ast::Expression};
    /// let parser = Parser::from_lexer(Lexer::new("(1, 2, 3)")).unwrap();
    /// assert_eq!(parser.parse_array_or_tuple().unwrap(), Expression::Tuple(vec![1, 2, 3]));
    /// ```
    /// 
    /// Array:
    /// 
    /// ```rust
    /// # use libquetzal::{Lexer, Parser, ast::Expression};
    /// let parser = Parser::from_lexer(Lexer::new("[1, 2, 3]")).unwrap();
    /// assert_eq!(parser.parse_array_or_tuple().unwrap(), Expression::Array(vec![1, 2, 3]));
    /// ```
    pub fn parse_array_or_tuple(&mut self) -> Result<Expression, ParseError> {
        let beg = self
            .next()
            .filter(|t| [TokenType::LeftBracket, TokenType::LeftParen].contains(&t.t))
            .expect("'[' for array or '(' for tuple")
            .t
            .clone(); // TODO: evaluate whether clone or move is best here
                      // '[' or '('

        let mut expressions = Vec::<Expression>::new();
        while !self
            .current_in(&[match beg {
                TokenType::LeftBracket => TokenType::RightBracket,
                TokenType::LeftParen => TokenType::RightParen,
                _ => unreachable!(),
            }])
            .ok_or(ParseError::UnclosedBlock)?
        {
            expressions.push(self.parse_expression()?);
            if self
                .current_in(&[TokenType::Comma])
                .ok_or(ParseError::UnclosedBlock)?
            {
                self.current += 1;
            } // ',' or ']'
        }
        Ok(match beg {
            TokenType::LeftBracket => Expression::Array(expressions),
            TokenType::LeftParen => Expression::Tuple(expressions),
            _ => unreachable!(),
        })
    }

    pub fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let lhs = self.parse_expression()?;
        self.next(); // '='
        let rhs = self.parse_expression()?;
        Ok(Expression::Assign(lhs.into(), rhs.into()))
    }

    fn get(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn current_in(&self, t: &[TokenType]) -> Option<bool> {
        Some(t.contains(&self.get()?.t))
    }

    fn next(&mut self) -> Option<&Token> {
        let r = self.tokens.get(self.current);
        self.current += 1;
        r
    }

    fn backtrack(&mut self) {
        self.current -= 1;
    }

    fn current_in_then_next(&mut self, t: &[TokenType]) -> Option<bool> {
        Some(t.contains(&self.next()?.t))
    }

    fn next_identifier_or(&mut self, error: ParseError) -> Result<Box<str>, ParseError> {
        Ok(self.next()
            .map(|t| match &t.t {
                TokenType::Identifier(s) => Some(s),
                _ => None,
            })
            .flatten()
            .ok_or(error)?
            .clone()) // TODO: evaluate whether clone or move is best here
    }
}

impl TryInto<Codegen> for Parser {
    type Error = ParseError;
    fn try_into(mut self) -> Result<Codegen, Self::Error> {
        self.construct().map(|x| x.into())
    }
}
