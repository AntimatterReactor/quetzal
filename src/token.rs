use crate::error::LexicalError;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum TokenType {
    #[default] None,

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
    AssignPlus,
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

    ThinArrow,
    FatArrow,

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

impl TryFrom<&str> for TokenType {
    type Error = LexicalError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "(" => Ok(Self::LeftParen),
            ")" => Ok(Self::RightParen),
            "{" => Ok(Self::LeftBrace),
            "}" => Ok(Self::RightBrace),
            "[" => Ok(Self::LeftBracket),
            "]" => Ok(Self::RightBracket),
            "<" => Ok(Self::LeftAngle),
            ">" => Ok(Self::RightAngle),
            ";" => Ok(Self::Semicolon),
            "," => Ok(Self::Comma),
            "." => Ok(Self::Dot),
            ":" => Ok(Self::Colon),
            "::" => Ok(Self::Scope),
            "~" => Ok(Self::Tilde),
            "`" => Ok(Self::Tick),
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "%" => Ok(Self::Modulo),
            "/%" => Ok(Self::DivMod),
            "=" => Ok(Self::Assign),
            "+=" => Ok(Self::AssignPlus),
            "-=" => Ok(Self::AssignMinus),
            "*=" => Ok(Self::AssignMul),
            "/=" => Ok(Self::AssignDiv),
            "%=" => Ok(Self::AssignModulo),
            "?=" => Ok(Self::Equal),
            "?!=" => Ok(Self::NotEqual),
            "?<" => Ok(Self::LessThan),
            "?>" => Ok(Self::GreaterThan),
            "?<=" => Ok(Self::LessThanEqual),
            "?>=" => Ok(Self::GreaterThanEqual),
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "loop" => Ok(Self::Loop),
            "while" => Ok(Self::While),
            "for" => Ok(Self::For),
            "fn" => Ok(Self::Function),
            "let" => Ok(Self::LetDecl),
            "const" => Ok(Self::ConstDecl),
            "ret" => Ok(Self::Return),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            _ => Err(LexicalError::InvalidTokenMatch(s.to_string()))
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Token(pub TokenType, pub String);
