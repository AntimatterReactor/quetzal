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
    pub fn line(&mut self, line: String) -> &mut Self {
        self.line = line.into_bytes();
        self.current = 0;
        self
    }

    pub fn tokenify(&mut self) -> Result<Vec<Token>, LexicalError> {
        while self.current + 1 < self.line.len() {
            break;
        }
        Err(LexicalError::SingleLinedLiteralMultiLinedString)
    }

    /// Turns a string into it's corresponding [`Token`] form
    /// 
    /// The usage of this function by itself is not recommended,
    /// as it will panic when called incorrectly, that is, when
    /// the current starting byte is not a double quotation mark.
    /// 
    /// # Panics
    /// 
    /// Panics when `self.line[self.current] != b'"'`.
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
        if self.line[self.current] != b'"' {
            return Err(LexicalError::StringWithoutLiteral);
        }
        self.current += 1;

        let mut strstring = String::new();
        while let Some(c) = self.line.get(self.current) {
            strstring.push(match c {
                &b'"' => return Ok(Token(TokenType::StringLiteral, strstring)),
                &b'\\' => {
                    self.current += 1;
                    Self::escape(self.line.get(self.current)
                        .ok_or(LexicalError::SingleLinedLiteralMultiLinedString)?
                        .to_owned().into())?
                },
                &b'^' => {
                    self.current += 1;
                    Self::caret(self.line.get(self.current)
                        .ok_or(LexicalError::SingleLinedLiteralMultiLinedString)?
                        .to_owned().into())?
                },
                &_ => (*c).into()
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
        while let Some(x @ 0x30..=0x39) = self.line.get(self.current) {
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
        while let Some(x @ 0x30..=0x39 | x @ 0x41..=0x5A | x @ 0x61..=0x7A | x @ 0x5F)
            = self.line.get(self.current) {
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
                break
            }
            self.current += 1;
        }

        Ok(Token(TokenType::try_from(collect.as_str())?, collect))
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
mod tests {}
