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
// use crate::parser::Parser;
use crate::token::{Token, TokenType};

/// Lexer object. Consists of the line
/// it's currently evaluating and the position
/// in the line it's currently on.
#[derive(Debug)]
pub struct Lexer {
    line: Box<[char]>,
    current: usize,
}

/// Implements all the function necessary to
/// lexicalize a line of quetzal code.
impl Lexer {
    /// Constructs a new [`Lexer`] with `line`
    /// as the line to be evaluated.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![allow(unused)]
    /// # use libquetzal::Lexer;
    /// let lexer = Lexer::new("\"foo\"");
    /// ```
    pub fn new(line: &str) -> Lexer {
        Self {
            line: line.chars().collect(),
            current: 0,
        }
    }

    /// The main entry point for lexing an entire line
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let l = Lexer::new("fn main").lexicalize().unwrap();
    /// let v: Vec<TokenType> = vec![
    ///     TokenType::Function,
    ///     TokenType::Identifier("main".into()),
    /// ];
    /// for (i, k) in l.iter().zip(v) {
    ///     assert_eq!(i.t, k);
    /// }
    /// ```
    pub fn lexicalize(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut line_result: Vec<Token> = Vec::new();
        while let Some(c) = self.line.get(self.current) {
            line_result.push(match c {
                &'"' => self.get_str()?,
                &('0'..='9') | &('A'..='Z') | &('a'..='z') | &'_' => self.get_ident_and_num(),
                o if o.is_ascii_punctuation() => self.get_op()?,
                x if x.is_ascii() => {
                    self.current += 1;
                    continue;
                }
                _ => return Err(LexicalError::InvalidTokenMatch(char::from(*c).into())),
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
    /// let tok = Lexer::new("\"abc\"").get_str().unwrap();
    /// assert_eq!(TokenType::String("abc".into()), tok.t);
    /// ```
    pub fn get_str(&mut self) -> Result<Token, LexicalError> {
        // Due to the way this is used, make sure that the first character
        // is a double quotation so that it can be safely skipped
        if self.line.get(self.current) != Some(&'"') {
            return Err(LexicalError::StringWithoutLiteral);
        }
        self.current += 1;

        let mut strstring = String::new();
        while let Some(c) = self.line.get(self.current) {
            strstring.push(match c {
                &'"' => {
                    self.current += 1;
                    return Ok(Token {
                        t: TokenType::String(strstring.into_boxed_str()),
                        pos: self.current,
                    });
                }
                &'\\' => {
                    self.current += 1;
                    Self::unescape(
                        self.line
                            .get(self.current)
                            .ok_or(LexicalError::SingleLinedLiteralMultiLinedString)?
                            .to_owned()
                            .into(),
                    )?
                }
                &'^' => {
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

    /// Turns an identifier or number into it's corresponding [`Token`] form
    ///
    /// Recognizes `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_`
    /// as valid.
    ///
    /// [`Identifier`] and [`Number`] is differentiated by the
    /// starting character.
    ///
    /// [`Identifier`]: TokenType::Identifier
    /// [`Number`]: TokenType::Number
    /// 
    /// # Panics
    /// 
    /// Panics when the string contains no valid identifier or number.
    ///
    /// # Identifier Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let tok = Lexer::new("asdegagt23_").get_ident_and_num();
    /// assert_eq!(TokenType::Identifier("asdegagt23_".into()), tok.t);
    /// ```
    ///
    /// # Numeric Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let tok = Lexer::new("13412231").get_ident_and_num();
    /// assert_eq!(TokenType::Number("13412231".into()), tok.t);
    /// ```
    pub fn get_ident_and_num(&mut self) -> Token {
        let start = self.current;
        while self
            .line
            .get(self.current)
            .filter(|&&c| c.is_ascii_alphanumeric() || c == '_')
            .is_some()
        {
            self.current += 1;
        }

        // asserts that a number or ident of length at least 1 is found
        assert_ne!(
            start, self.current,
            "somehow, this function is called without a valid identifier or number"
        );

        // this piece of code will never panic because out of bounds is impossible
        // due to previous `.get` on the loop that must be in bounds to continue
        let ident: Box<str> = self.line[start..self.current]
            .iter()
            .collect::<String>()
            .into_boxed_str();

        Token {
            t: if ident.starts_with(|c: char| c.is_ascii_digit()) {
                TokenType::Number(ident)
            } else {
                TokenType::from_keyword(&ident).unwrap_or(TokenType::Identifier(ident))
            },
            pos: self.current,
        }
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
    /// let tok = Lexer::new("+=").get_op().unwrap();
    /// assert_eq!(TokenType::AssignPlus, tok.t);
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

        Ok(Token {
            t: loop {
                match TokenType::from_op(collect.as_str()) {
                    Ok(x) => break x,
                    Err(e) => {
                        if collect.pop().is_some() {
                            self.current -= 1;
                        } else {
                            return Err(e);
                        }
                    }
                }
            },
            pos: self.current,
        })
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
    /// let c = Lexer::unescape('n');
    /// assert_eq!(Ok('\n'), c);
    /// ```
    pub const fn unescape(c: char) -> Result<char, LexicalError> {
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

pub fn has_unclosed_symmetric(tokens: &[Token]) -> Result<bool, LexicalError> {
    let mut a = 0i8;
    let mut br = 0i8;
    let mut bk = 0i8;
    let mut p = 0i8;

    for t in tokens {
        match t.t {
            TokenType::LeftAngle => a += 1,
            TokenType::LeftCurl => br += 1,
            TokenType::LeftBracket => bk += 1,
            TokenType::LeftParen => p += 1,
            TokenType::RightAngle => a -= 1,
            TokenType::RightCurl => br -= 1,
            TokenType::RightBracket => bk -= 1,
            TokenType::RightParen => p -= 1,
            _ => continue,
        }
    }

    if a < 0 || br < 0 || bk < 0 || p < 0 {
        Err(LexicalError::UnexpectedRightSymmetric)
    } else {
        Ok(a > 0 || br > 0 || bk > 0 || p > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::TokenType;

    #[test]
    fn lex_scope() {
        let l = Lexer::new("require use std::io::println")
            .lexicalize()
            .unwrap();
        let v: Vec<TokenType> = vec![
            TokenType::Identifier("require".into()),
            TokenType::Identifier("use".into()),
            TokenType::Identifier("std".into()),
            TokenType::Scope,
            TokenType::Identifier("io".into()),
            TokenType::Scope,
            TokenType::Identifier("println".into()),
        ];
        for (i, k) in l.iter().zip(v) {
            assert_eq!(i.t, k);
        }
    }

    #[test]
    fn lex_complex() {
        let l = Lexer::new("fn main { std::io::println(\"Hello World!\") } -> void")
            .lexicalize()
            .unwrap();
        let v: Vec<TokenType> = vec![
            TokenType::Function,
            TokenType::LeftBracket,
            TokenType::Scope,
            TokenType::RightBracket,
            TokenType::Identifier("main".into()),
            TokenType::LeftCurl,
            TokenType::Identifier("std".into()),
            TokenType::Scope,
            TokenType::Identifier("io".into()),
            TokenType::Scope,
            TokenType::Identifier("println".into()),
            TokenType::LeftParen,
            TokenType::String("Hello World!".into()),
            TokenType::RightParen,
            TokenType::RightCurl,
            TokenType::ThinArrow,
            TokenType::Identifier("void".into()),
        ];
        for (i, k) in l.iter().zip(v) {
            assert_eq!(i.t, k);
        }
    }
}
