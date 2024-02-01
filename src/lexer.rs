use crate::error::LexicalError;
use crate::token::{Token, TokenType};

pub enum TokenType {
    None = 0,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,

    Semicolon,
    Comma,
    Dot,
    Colon,
    Scope,
    Tilde,
    Tick,

    Plus,
    Minus,
    Mul,
    Div,
    Modulo,
    DivMod,

    Assign,
    AssignMinus,
    AssignMul,
    AssignDiv,
    AssignModulo,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    And,
    Or,
    Not,

    If,
    Else,

    Loop,
    While,
    For,

    Function,
    LetDecl,
    ConstDecl,
    Return,

    True,
    False,

    StringLiteral,
    NumericLiteral,
    Identifier,
}

impl TokenType {
    pub fn to_token_type(s: &str) -> Self {
        match s {
            "(" => Self::LeftParen,
            ")" => Self::RightParen,
            "{" => Self::LeftBrace,
            "}" => Self::RightBrace,
            "[" => Self::LeftBracket,
            "]" => Self::RightBracket,
            "<" => Self::LeftAngle,
            ">" => Self::RightAngle,
            ";" => Self::Semicolon,
            "," => Self::Comma,
            "." => Self::Dot,
            ":" => Self::Colon,
            "::" => Self::Scope,
            "~" => Self::Tilde,
            "`" => Self::Tick,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Modulo,
            "/%" => Self::DivMod,
            "=" => Self::Assign,
            "-=" => Self::AssignMinus,
            "*=" => Self::AssignMul,
            "/=" => Self::AssignDiv,
            "%=" => Self::AssignModulo,
            "?=" => Self::Equal,
            "?!=" => Self::NotEqual,
            "?<" => Self::LessThan,
            "?>" => Self::GreaterThan,
            "?<=" => Self::LessThanEqual,
            "?>=" => Self::GreaterThanEqual,
            "and" => Self::And,
            "or" => Self::Or,
            "not" => Self::Not,
            "if" => Self::If,
            "else" => Self::Else,
            "loop" => Self::Loop,
            "while" => Self::While,
            "for" => Self::For,
            "fn" => Self::Function,
            "let" => Self::LetDecl,
            "const" => Self::ConstDecl,
            "ret" => Self::Return,
            "true" => Self::True,
            "false" => Self::False,
            _ => Self::None,
        }
    }
}

pub struct Token(TokenType, String);

pub struct Lexer {
    tokens: Vec<Token>,
    current: usize,
    line: String,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: Vec::new(), current: 0, line: String::new() }
    }


    pub fn tokenify(&self) -> Result<&Self, LexicalError> {
        for c in self.line.chars() {

        }
        Ok(self)
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
    /// # use quetzal::Lexer;
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
    /// # use quetzal::Lexer;
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
