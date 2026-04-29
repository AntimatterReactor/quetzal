// Copyright (C) 2024 Ezra A. Derrian
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Turns Quetzal code into Tokens

use crate::error::{LexicalErrorType, LexicalError, Location};
// use crate::parser::Parser;
use crate::token::{Token, TokenType};
use unicode_ident::{is_xid_continue, is_xid_start};
use std::hint::cold_path;

/// Lexer object. Consists of the line
/// it's currently evaluating and the position
/// in the line it's currently on.
#[derive(Debug)]
pub struct Lexer {
    source: Box<[char]>,
    current_location: Location,
    current_index: usize,
    indent_state: usize
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
    pub fn new(line_input: &str) -> Lexer {
        Self {
            source: line_input.chars().collect(),
            current_location: Location { line: 0, column: 0 },
            current_index: 0,
            indent_state: 0,
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
        let mut indentation_handled = false;
        while let Some(c) = self.source.get(self.current_index) {
            line_result.push(match *c {
                '"' => self.get_str()?,
                i if is_xid_start(i) || i == '_' => self.get_ident(),
                o if o.is_ascii_punctuation() => self.get_op()?,
                n if n.is_ascii_digit() => self.get_number(),
                x if x.is_ascii_control() || x == ' ' => {
                    if !indentation_handled {
                        todo!("fix indentation");
                        indentation_handled = true;
                        if let Some(i) = self.handle_whitespace() {
                            Token{
                                t: i,
                                pos: self.current_location
                            }
                        } else {
                            continue;
                        }
                    } else {
                        self.proceed();
                        continue;
                    }
                }
                e => return Err(self.with_loc(LexicalErrorType::UnknownCharacter(e))),
            })
        }
        Ok(line_result)
    }
    
    fn handle_whitespace(&mut self) -> Option<TokenType> {
        let mut current_indent_state: usize = 0;
        let mut space_counter: usize = 0;
        let mut emit_token_counter: usize = 0;
        loop {
            match self.source.get(self.current_index) {
                Some(&' ') => {
                    space_counter += 1;
                    if space_counter % 4 == 0 {
                        current_indent_state += 1;
                    }
                    self.proceed();
                }
                Some(&'\t') => {
                    current_indent_state += 1;
                    self.proceed();
                }
                Some(&'\n') => {
                    emit_token_counter += 1;
                    self.proceed_newline();
                }
                // TODO: what if \v, \f, \r?
                _ => break,
            }
        }

        if current_indent_state == self.indent_state || emit_token_counter > 1 {
            None
        }
        else if current_indent_state > self.indent_state {
            Some(TokenType::Indent)
        }
        else if current_indent_state < self.indent_state {
            Some(TokenType::Dedent)
        }
        else {
            unreachable!("for indent to have any other value than ==, <, or > last state")
        }
    }

    fn handle_comments(&mut self) {
        // Throws if current char is not comment
        match self.source.get(self.current_index).expect("for function to be called on a comment") {
            '/' => {
                self.proceed();
                match self.source.get(self.current_index).filter(|&&x| x == '*' || x == '/') {
                    Some('*') => {
                    }
                    Some('/') => {
                    }
                    _ => return
                }
            }
            '#' => {
                self.proceed();
                while self.source.get(self.current_index).is_some_and(|&x| !Self::is_newline(x)) {
                    self.proceed();
                }
            }
            _ => {
                unreachable!("handle_comments called not on a comment");
            }
        }
    }

    const fn is_newline(c: char) -> bool {
        match c {
            '\n' | '\r' => true,
            _ => false
        }
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
        // is a double-quotation so that it can be safely skipped
        debug_assert_eq!(self.source.get(self.current_index), Some(&'"'));

        self.current_location.column += 1;

        let mut strstring = String::new();
        while let Some(c) = self.source.get(self.current_index) {
            strstring.push(match *c {
                '"' => {
                    self.current_location.column += 1;
                    return Ok(Token {
                        t: TokenType::String(strstring.into_boxed_str()),
                        pos: self.current_location,
                    });
                }
                '\\' => {
                    self.current_location.column += 1;
                    Self::escape_handler(
                        self.source
                            .get(self.current_index)
                            .ok_or(self.with_loc(LexicalErrorType::UnclosedStringLiteral))?
                            .to_owned()
                    ).map_err(|x| self.with_loc(x))?
                }
                '^' => {
                    self.current_location.column += 1;
                    Self::caret(
                        self.source
                            .get(self.current_index)
                            .ok_or(self.with_loc(LexicalErrorType::UnclosedStringLiteral))?
                            .to_owned()
                    ).map_err(|x| self.with_loc(x))?
                }
                '\n' => return Err(self.with_loc(LexicalErrorType::UnescapedNewlineInString)),
                _ => (*c).into(),
            });
            self.current_location.column += 1;
        }
        Err(self.with_loc(LexicalErrorType::UnclosedStringLiteral))
    }

    pub fn get_char_literal(&mut self) -> Result<Token, LexicalError> {
        // Due to the way this is used, make sure that the first character
        // is a single-quotation so that it can be safely skipped
        debug_assert_eq!(self.source.get(self.current_index), Some(&'\''));

        self.proceed();
        let r: char = match self.source.get(self.current_index).ok_or(self.with_loc(LexicalErrorType::UnclosedCharLiteral))? {
            '\\' => {
                self.proceed();
                Self::escape_handler(
                    self.source
                        .get(self.current_index)
                        .ok_or(self.with_loc(LexicalErrorType::UnclosedCharLiteral))?
                        .to_owned()
                ).map_err(|x| self.with_loc(x))?
            }
            '^' => {
                self.proceed();
                Self::caret(
                    self.source
                        .get(self.current_index)
                        .ok_or(self.with_loc(LexicalErrorType::UnclosedCharLiteral))?
                        .to_owned()
                ).map_err(|x| self.with_loc(x))?
            }
            '\'' => return Err(self.with_loc(LexicalErrorType::EmptyCharLiteral)),
            '\n' => return Err(self.with_loc(LexicalErrorType::UnescapedNewlineInChar)),
            other => *other,
        };

        self.proceed();
        self.source.get(self.current_index).filter(|&&x| x == '\'').ok_or(self.with_loc(LexicalErrorType::UnclosedCharLiteral))?;

        Ok(Token {
            t: TokenType::Char(r),
            pos: self.current_location
        })
    }

    /// Turns an identifier into it's corresponding [`Token`] form
    ///
    /// [`Identifier`] and [`Number`] is differentiated by the
    /// starting character.
    ///
    /// [`Identifier`]: TokenType::Identifier
    /// [`Number`]: TokenType::Number
    ///
    /// # Panics
    ///
    /// Panics when the string contains no valid identifier.
    ///
    /// # Identifier Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let tok = Lexer::new("asdegagt23_").get_ident();
    /// assert_eq!(TokenType::Identifier("asdegagt23_".into()), tok.t);
    /// ```
    pub fn get_ident(&mut self) -> Token {
        let start = self.current_location.column;
        while self
            .source
            .get(self.current_index)
            .filter(|&&c| is_xid_continue(c))
            .is_some()
        {
            self.current_location.column += 1;
        }

        // asserts that a number or ident of length at least 1 is found
        debug_assert_ne!(start, self.current_location.column);

        // this piece of code will never panic because out of bounds is impossible
        // due to previous `.get` on the loop that must be in bounds to continue
        let ident: Box<str> = self.source[start..self.current_location.column]
            .iter()
            .collect::<String>()
            .into_boxed_str();

        // asserts that the function is NOT called on a number
        debug_assert!(!ident.starts_with(|c: char| c.is_ascii_digit()));

        Token {
            t: TokenType::from_keyword(&ident).unwrap_or(TokenType::Identifier(ident)),
            pos: self.current_location,
        }
    }

    /// Turns a number into it's corresponding [`Token`] form
    ///
    /// [`Identifier`] and [`Number`] is differentiated by the
    /// starting character.
    ///
    /// [`Identifier`]: TokenType::Identifier
    /// [`Number`]: TokenType::Number
    ///
    /// # Panics
    ///
    /// Panics when the string contains no valid number.
    ///
    /// # Numeric Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let tok = Lexer::new("13412231").get_number();
    /// assert_eq!(TokenType::Number("13412231".into()), tok.t);
    /// ```
    pub fn get_number(&mut self) -> Token {
        let start = self.current_location.column;
        while self
            .source
            .get(self.current_index)
            .filter(|&&c| c.is_ascii_digit())
            .is_some()
        {
            self.current_location.column += 1;
        }

        // asserts that a number or ident of length at least 1 is found
        debug_assert_ne!(start, self.current_location.column);

        Token {
            // this piece of code will never panic because out of bounds is impossible
            // due to previous `.get` on the loop that must be in bounds to continue
            t: TokenType::Number(
                self.source[start..self.current_location.column]
                    .iter()
                    .collect::<String>()
                    .into_boxed_str(),
            ),
            pos: self.current_location,
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
    /// Panics when `self.line[self.current.column].is_ascii_punctuation() == false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquetzal::{Lexer, Token, TokenType};
    /// let tok = Lexer::new("+=").get_op().unwrap();
    /// assert_eq!(TokenType::AssignPlus, tok.t);
    /// ```
    pub fn get_op(&mut self) -> Result<Token, LexicalError> {
        let c0 = self.source[self.current_location.column];

        // Make sure that the current character is indeed a punctuation
        debug_assert!(c0.is_ascii_punctuation());

        let c1 = self.source.get(self.current_index + 1);
        let c2 = self.source.get(self.current_index + 2);

        // table slop macro
        macro_rules! op_table {
            ($c0:expr, $c1:expr, $c2:expr, $col:expr, [
                $(($pat0:pat, $pat1:pat, $pat2:pat, $advance:expr) => $tok:expr),* $(,)?
            ], $err:expr) => {
                match ($c0, $c1, $c2) {
                    $(($pat0, $pat1, $pat2) => { $col += $advance; $tok })*
                    _ => { $col += 1; return Err($err); }
                }
            };
        }
        
        // Clunky slop, but O(1)
        todo!("some tokens have changed, please reflect accordingly");
        let tokentype: TokenType = op_table!(c0, c1, c2, self.current_location.column, [
            // ----- Three Chars -----
            ('?', Some('!'), Some('='), 3) => TokenType::NotEqual,
            ('?', Some('<'), Some('='), 3) => TokenType::LessThanEqual,
            ('?', Some('>'), Some('='), 3) => TokenType::GreaterThanEqual,

            // ----- Two Chars -----
            ('+', Some('='), _, 2) => TokenType::AssignPlus,
            ('-', Some('='), _, 2) => TokenType::AssignMinus,
            ('*', Some('='), _, 2) => TokenType::AssignMul,
            ('/', Some('='), _, 2) => TokenType::AssignDiv,
            ('%', Some('='), _, 2) => TokenType::AssignModulo,
            ('?', Some('='), _, 2) => TokenType::Equal,
            ('-', Some('>'), _, 2) => TokenType::ThinArrow,
            ('=', Some('>'), _, 2) => TokenType::FatArrow,
            (':', Some(':'), _, 2) => TokenType::Scope,
            ('/', Some('%'), _, 2) => TokenType::DivMod,

            // ----- One Char -----
            ('(', _, _, 1) => TokenType::LeftParen,
            (')', _, _, 1) => TokenType::RightParen,
            ('[', _, _, 1) => TokenType::LeftBracket,
            (']', _, _, 1) => TokenType::RightBracket,
            ('{', _, _, 1) => TokenType::LeftCurl,
            ('}', _, _, 1) => TokenType::RightCurl,
            ('<', _, _, 1) => TokenType::LeftAngle,
            ('>', _, _, 1) => TokenType::RightAngle,
            (';', _, _, 1) => TokenType::Semicolon,
            (',', _, _, 1) => TokenType::Comma,
            ('.', _, _, 1) => TokenType::Dot,
            (':', _, _, 1) => TokenType::Colon,
            ('~', _, _, 1) => TokenType::Tilde,
            ('`', _, _, 1) => TokenType::Tick,
            ('+', _, _, 1) => TokenType::Plus,
            ('-', _, _, 1) => TokenType::Minus,
            ('*', _, _, 1) => TokenType::Mul,
            ('/', _, _, 1) => TokenType::Div,
            ('%', _, _, 1) => TokenType::Modulo,
            ('=', _, _, 1) => TokenType::Assign,
            ('@', _, _, 1) => TokenType::At,
            ('|', _, _, 1) => TokenType::VerticalBar,
        ], self.with_loc(LexicalErrorType::UnknownCharacter(c0)));

        Ok(Token {
            t: tokentype,
            pos: self.current_location,
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
    /// let c = Lexer::escape_handler('n');
    /// assert_eq!(Ok('\n'), c);
    /// ```
    pub const fn escape_handler(c: char) -> Result<char, LexicalErrorType> {
        // TODO: you should be able to "escape" a new line, (somewhat like macros in c)
        // TODO: expand to escape more random crap like line feed, etc
        // !: don't forget to also handle CRLF
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
            '^' | '\\' | '\"' | '\'' | '\n' | '\r' => Ok(c),
            _ => Err(LexicalErrorType::InvalidEscape(c)),
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
    pub const fn caret(c: char) -> Result<char, LexicalErrorType> {
        match c as u32 {
            0x3F => Ok(0x7F as char),
            0x40..=0x5F => Ok((c as u8 - 0x40) as char),
            _ => Err(LexicalErrorType::InvalidCaret(c)),
        }
    }

    #[inline]
    fn proceed(&mut self) {
        self.current_index += 1;
        self.current_location.column += 1;
    }

    #[inline]
    fn proceed_newline(&mut self) {
        self.current_index += 1;
        self.current_location.column = 0;
        self.current_location.line += 1;
    }

    #[inline]
    fn with_loc(&self, err_type: LexicalErrorType) -> LexicalError {
        (err_type, self.current_location).into()
    }
}

pub fn has_unclosed_symmetric(tokens: &[Token]) -> Result<bool, LexicalErrorType> {
    let mut a = 0i8;
    let mut br = 0i8;
    let mut bk = 0i8;
    let mut p = 0i8;
    let mut c: char = '\0';

    for t in tokens {
        todo!("implement c = <last right symm>");
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
        Err(LexicalErrorType::UnexpectedRightSymmetric(c))
    } else {
        Ok(a > 0 || br > 0 || bk > 0 || p > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::{error::LexicalErrorType, token::TokenType::{self, *}};

    fn test_lexer(input: &'static str, expected_tokens: &[TokenType]) {
        let l = Lexer::new(input)
            .lexicalize()
            .expect("valid tokens");
        for (i, k) in l.iter().zip(expected_tokens) {
            assert_eq!(&i.t, k);
        }
    }

    #[test]
    fn lex_scope() {
        test_lexer("require use std::io::println", &[
            Identifier("require".into()),
            Identifier("use".into()),
            Identifier("std".into()),
            Scope,
            Identifier("io".into()),
            Scope,
            Identifier("println".into()),
        ]);
    }

    #[test]
    fn lex_complex() {
        test_lexer("fn main { std::io::println(\"Hello World!\") } -> void", &[
            Function,
            Identifier("main".into()),
            LeftCurl,
            Identifier("std".into()),
            Scope,
            Identifier("io".into()),
            Scope,
            Identifier("println".into()),
            LeftParen,
            String("Hello World!".into()),
            RightParen,
            RightCurl,
            ThinArrow,
            Identifier("void".into()),
        ]);
    }

    
    #[test]
    fn get_str_basic() {
        assert_eq!(
            Lexer::new("\"abcdef\"").get_str().expect("valid string").t,
            String("abcdef".into())
        )
    }

    #[test]
    fn get_str_escape_sequence() {
        assert_eq!(
            Lexer::new("\"\\n \\r \\0\\\\\\\n\"").get_str().expect("valid string").t,
            String("\n \r \0\\\n".into())
        )
    }

    #[test]
    fn get_str_caret_sequence() {
        assert_eq!(
            Lexer::new("\"^J ^M ^@\\^^I\"").get_str().expect("valid string").t,
            String("\n \r \0^\t".into())
        )
    }

    #[test]
    fn get_str_mixed_sequence() {
        assert_eq!(
            Lexer::new("\"\\n ^M\\^\0@\\^^I\\\n\"").get_str().expect("valid string").t,
            String("\n \r^\0@^\t\n".into())
        )
    }

    #[test]
    fn get_str_unclosed_string() {
        assert_eq!(
            *Lexer::new("\"a^Bc\\n\0abc").get_str().expect_err("invalid string").get_type(),
            LexicalErrorType::UnclosedStringLiteral
        )
    }

    #[test]
    fn get_str_invalid_escape() {
        assert_eq!(
            *Lexer::new("\"\\n \\z^M\\^\0@\\^^I\\\n\"").get_str().expect_err("invalid string").get_type(),
            LexicalErrorType::InvalidEscape('z')
        )
    }
    
    #[test]
    fn get_str_invalid_caret() {
        assert_eq!(
            *Lexer::new("\"\\n ^M^0\\^\0@\\^^I\\\n\"").get_str().expect_err("invalid string").get_type(),
            LexicalErrorType::InvalidCaret('0')
        )
    }

    #[test]
    fn get_str_empty_string() {
        assert_eq!(
            Lexer::new("\"\"").get_str().expect("valid string").t,
            String("".into())
        )
    }

    #[test]
    fn get_str_unescaped_newline() {
        assert_eq!(
            *Lexer::new("\"\n\"").get_str().expect_err("invalid string").get_type(),
            LexicalErrorType::UnescapedNewlineInString
        )
    }

    #[test]
    fn get_ident() {
    }

/*
get_ident

plain ident foo
ident with digits/underscore foo_23
all keywords (fn if else while for loop let const return defer true false and or not)

get_number

plain number 12345

get_op

all 3-char ops ?!= ?<= ?>=
all 2-char ops += -= *= /= %= ?= ?< ?> -> => :: /%
all 1-char ops
unknown punct → UnknownCharacter

has_unclosed_symmetric

balanced () [] {} <>
unclosed open → Ok(true)
unexpected close → Err(UnexpectedRightSymmetric)
empty slice → Ok(false)

lexicalize integration

whitespace/control chars skipped
mixed ident + op + string + number
unknown char → UnknownCharacter
*/

}
