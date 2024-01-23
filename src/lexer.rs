use crate::error::LexicalError;

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
    /// # Example
    /// # use quetzal::Lexer::escape;
    /// # fn main() {
    /// let c = escape('n');
    /// assert_eq!('\n', c);
    /// # }
    pub fn escape(c: char) -> Result<char, LexicalError> {
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
            _ => Err(LexicalError::InvalidEscape(c)),
        }
    }

    pub fn caret(c: char) -> Result<char, LexicalError> {
        match c as u32 {
            0x3F => Ok(0x7F as char),
            0x40..=0x5F => Ok((c as u8 - 0x40) as char),
            _ => Err(LexicalError::InvalidCaret(c)),
        }
    }
    
    pub fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
}

#[cfg(test)]
mod tests {}
