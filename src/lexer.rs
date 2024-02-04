//! Turns Quetzal code into Tokens
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

use crate::error::LexicalError;
use crate::token::{Token, TokenType};

/// Lexer object. Consists of the line
/// it's currently evaluating and the position
/// in the line it's currently on.
#[derive(Debug)]
pub struct Lexer {
    line: Vec<u8>,
    current: usize,
}

/// Implements all the function necessary to
/// lexicalize a line of quetzal code.
impl Lexer {
    /// Constructs a new empty [`Lexer`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![allow(unused_mut)]
    /// # use libquetzal::Lexer;
    /// let mut lexer = Lexer::new();
    /// ```
    pub fn new() -> Lexer {
        Lexer {
            line: Vec::new(),
            current: 0,
        }
    }

    /// Changes the current stored line into the
    /// next/new one
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let mut lexer = Lexer::new();
    /// lexer.line("\"foo\"".to_string()); // line is now "abc"
    /// lexer.line("bar12".to_string()); // line is now bar12
    /// ```
    pub fn line(&mut self, line: String) -> &mut Self {
        self.line = line.into_bytes();
        self.current = 0;
        self
    }

    /// The main entry point for lexing an entire line
    ///
    /// # Example
    /// 
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let mut lexer = Lexer::new();
    /// let l = lexer.line("fn main".to_string()).tokenify().unwrap();
    /// let v: Vec<Token> = vec![
    ///     Token(TokenType::Identifier, "fn".to_string()),
    ///     Token(TokenType::Identifier, "main".to_string()),
    /// ];
    /// assert_eq!(l, v);
    /// ```
    pub fn tokenify(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut line_result: Vec<Token> = Vec::new();
        while let Some(c) = self.line.get(self.current) {
            line_result.push(match c {
                &b'"' => self.get_str()?,
                &(b'0'..=b'9') => self.get_int(),
                &(b'A'..=b'Z') | &(b'a'..=b'z') | &b'_' => self.get_ident(),
                o if o.is_ascii_punctuation() => self.get_op()?,
                x if x.is_ascii() => {
                    self.current += 1;
                    continue;
                }
                _ => {
                    return Err(LexicalError::InvalidTokenMatch(
                        char::from(c.to_owned()).into(),
                    ))
                }
            })
        }
        Ok(line_result)
    }

    /// Turns a string into it's corresponding [`Token`] form
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let mut lexer = Lexer::new();
    /// let tok = lexer.line("\"abc\"".to_string()).get_str();
    /// assert_eq!(Ok(Token(TokenType::StringLiteral, "abc".to_string())), tok);
    /// ```
    pub fn get_str(&mut self) -> Result<Token, LexicalError> {
        // Due to the way this is used, make sure that the first character
        // is a double quotation so that it can be safely skipped
        if self.line.get(self.current) != Some(&b'"') {
            return Err(LexicalError::StringWithoutLiteral);
        }
        self.current += 1;

        let mut strstring = String::new();
        while let Some(c) = self.line.get(self.current) {
            strstring.push(match c {
                &b'"' => {
                    self.current += 1;
                    return Ok(Token(TokenType::StringLiteral, strstring));
                }
                &b'\\' => {
                    self.current += 1;
                    Self::escape(
                        self.line
                            .get(self.current)
                            .ok_or(LexicalError::SingleLinedLiteralMultiLinedString)?
                            .to_owned()
                            .into(),
                    )?
                }
                &b'^' => {
                    self.current += 1;
                    Self::caret(
                        self.line
                            .get(self.current)
                            .ok_or(LexicalError::SingleLinedLiteralMultiLinedString)?
                            .to_owned()
                            .into(),
                    )?
                }
                &_ => (*c).into(),
            });
            self.current += 1;
        }
        Err(LexicalError::SingleLinedLiteralMultiLinedString)
    }

    /// Turns an integer into it's corresponding [`Token`] form
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let mut lexer = Lexer::new();
    /// let tok = lexer.line("13412231".to_string()).get_int();
    /// assert_eq!(Token(TokenType::NumericLiteral, "13412231".to_string()), tok);
    /// ```
    pub fn get_int(&mut self) -> Token {
        let mut number = String::new();
        while let Some(x @ b'0'..=b'9') = self.line.get(self.current) {
            number.push(*x as char);
            self.current += 1;
        }
        Token(TokenType::NumericLiteral, number)
    }

    /// Turns an identifier into it's corresponding [`Token`] form
    ///
    /// Recognizes `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_`
    /// as valid identifier.
    ///
    /// Note that under normal circumstances because `int` is searched first,
    /// identifying `121341` as [`Identifier`] should not happen
    ///
    /// [`Identifier`]: TokenType::Identifier
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let mut lexer = Lexer::new();
    /// let tok = lexer.line("asdegagt23_".to_string()).get_ident();
    /// assert_eq!(Token(TokenType::Identifier, "asdegagt23_".to_string()), tok);
    /// ```
    pub fn get_ident(&mut self) -> Token {
        let mut number = String::new();
        while let Some(x @ b'0'..=b'9' | x @ b'A'..=b'Z' | x @ b'a'..=b'z' | x @ b'_') =
            self.line.get(self.current)
        {
            number.push(*x as char);
            self.current += 1;
        }
        Token(TokenType::Identifier, number)
    }

    /// Turns an operator into it's corresponding [`Token`] form
    ///
    /// The usage of this function by itself is not recommended,
    /// as it will panic when called incorrectly, that is, when
    /// the current starting byte is not a punctuation.
    ///
    /// # Panics
    ///
    /// Panics when `self.line[self.current].is_ascii_punctuation() == false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let mut lexer = Lexer::new();
    /// let tok = lexer.line("+=".to_string()).get_op();
    /// assert_eq!(Ok(Token(TokenType::AssignPlus, "+=".to_string())), tok);
    /// ```
    pub fn get_op(&mut self) -> Result<Token, LexicalError> {
        // Make sure that the current character is indeed a punctuation
        assert!(self.line[self.current].is_ascii_punctuation());

        let mut collect = String::new();

        while let Some(c) = self.line.get(self.current) {
            if c.is_ascii_punctuation() {
                collect.push((*c).into());
            } else {
                break;
            }
            self.current += 1;
        }

        Ok(Token(
            loop {
                match TokenType::try_from(collect.as_str()) {
                    Ok(x) => break x,
                    Err(_) => {
                        collect.pop();
                        self.current -= 1;
                    }
                }
            },
            collect,
        ))
    }

    /// Turn escaped characters into their intended form
    ///
    /// Only accepts `a b e f n r t v 0 ^ \ " '`,
    /// otherwise will return [`InvalidEscape`]
    ///
    /// [`InvalidEscape`]: LexicalError::InvalidEscape
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::Lexer;
    /// let c = Lexer::escape('n');
    /// assert_eq!(Ok('\n'), c);
    /// ```
    pub const fn escape(c: char) -> Result<char, LexicalError> {
        match c {
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            '0' => Ok('\0'),
            'a' => Ok(0x07 as char),
            'b' => Ok(0x08 as char),
            'v' => Ok(0x0B as char),
            'f' => Ok(0x0C as char),
            'e' => Ok(0x1B as char),
            '^' | '\\' | '\"' | '\'' => Ok(c),
            _ => Err(LexicalError::InvalidEscape(c)),
        }
    }

    /// Caret characters like in terminals
    ///
    /// Any alphabetic character following the caret must be
    /// an uppercase letter. Using a lowercase letter will
    /// result in an error being returned.
    ///
    /// # Further Reference
    ///
    /// See [Wikipedia's article on C0 (and C1) control codes](https://en.wikipedia.org/wiki/C0_and_C1_control_codes#C0_controls)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::Lexer;
    /// let c = Lexer::caret('J');
    /// assert_eq!(Ok('\n'), c);
    /// ```
    pub const fn caret(c: char) -> Result<char, LexicalError> {
        match c as u32 {
            0x3F => Ok(0x7F as char),
            0x40..=0x5F => Ok((c as u8 - 0x40) as char),
            _ => Err(LexicalError::InvalidCaret(c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn lex_scope() {
        let mut lexer = Lexer::new();
        let l = lexer
            .line("require use std::io::IO".to_string())
            .tokenify()
            .unwrap();
        let v: Vec<Token> = vec![
            Token(TokenType::Identifier, "require".to_string()),
            Token(TokenType::Identifier, "use".to_string()),
            Token(TokenType::Identifier, "std".to_string()),
            Token(TokenType::Scope, "::".to_string()),
            Token(TokenType::Identifier, "io".to_string()),
            Token(TokenType::Scope, "::".to_string()),
            Token(TokenType::Identifier, "IO".to_string()),
        ];
        assert_eq!(l, v);
    }

    #[test]
    fn lex_complex() {
        let mut lexer = Lexer::new();
        let l = lexer
            .line("fn main start IO::init().println(\"Hello World!\") end -> 0".to_string())
            .tokenify()
            .unwrap();
        let v: Vec<Token> = vec![
            Token(TokenType::Identifier, "fn".to_string()),
            Token(TokenType::Identifier, "main".to_string()),
            Token(TokenType::Identifier, "start".to_string()),
            Token(TokenType::Identifier, "IO".to_string()),
            Token(TokenType::Scope, "::".to_string()),
            Token(TokenType::Identifier, "init".to_string()),
            Token(TokenType::LeftParen, "(".to_string()),
            Token(TokenType::RightParen, ")".to_string()),
            Token(TokenType::Dot, ".".to_string()),
            Token(TokenType::Identifier, "println".to_string()),
            Token(TokenType::LeftParen, "(".to_string()),
            Token(TokenType::StringLiteral, "Hello World!".to_string()),
            Token(TokenType::RightParen, ")".to_string()),
            Token(TokenType::Identifier, "end".to_string()),
            Token(TokenType::ThinArrow, "->".to_string()),
            Token(TokenType::NumericLiteral, "0".to_string()),
        ];
        assert_eq!(l, v);
    }
}
