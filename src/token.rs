use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Bang,
    BangEqual,
    Comma,
    Comment,
    Dot,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    LBrace,
    LParen,
    Less,
    LessEqual,
    Minus,
    Newline,
    Plus,
    RBrace,
    RParen,
    Semicolon,
    Slash,
    Star,
    Whitespace,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::Bang => "BANG",
            Self::BangEqual => "BANG_EQUAL",
            Self::Comma => "COMMA",
            Self::Comment => "COMMENT",
            Self::Dot => "DOT",
            Self::Equal => "EQUAL",
            Self::EqualEqual => "EQUAL_EQUAL",
            Self::Greater => "GREATER",
            Self::GreaterEqual => "GREATER_EQUAL",
            Self::LBrace => "LEFT_BRACE",
            Self::LParen => "LEFT_PAREN",
            Self::Less => "LESS",
            Self::LessEqual => "LESS_EQUAL",
            Self::Minus => "MINUS",
            Self::Newline => "NEWLINE",
            Self::Plus => "PLUS",
            Self::RBrace => "RIGHT_BRACE",
            Self::RParen => "RIGHT_PAREN",
            Self::Semicolon => "SEMICOLON",
            Self::Slash => "SLASH",
            Self::Star => "STAR",
            Self::Whitespace => "WHITESPACE",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenError {
    pub message: String,
    pub line: u16,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

pub type TokenResult = Result<Token, TokenError>;

pub const SINGLE_CHARACTER_LEXEMES: &[(&str, TokenType)] = &[
    ("!", TokenType::Bang),
    (",", TokenType::Comma),
    (".", TokenType::Dot),
    ("=", TokenType::Equal),
    (">", TokenType::Greater),
    ("{", TokenType::LBrace),
    ("(", TokenType::LParen),
    ("<", TokenType::Less),
    ("-", TokenType::Minus),
    ("\n", TokenType::Newline),
    ("+", TokenType::Plus),
    ("}", TokenType::RBrace),
    (")", TokenType::RParen),
    (";", TokenType::Semicolon),
    ("/", TokenType::Slash),
    ("*", TokenType::Star),
    (" ", TokenType::Whitespace),
    ("\t", TokenType::Whitespace),
];

pub const MULTI_CHARACTER_LEXEMES: &[(&str, TokenType)] = &[
    ("!=", TokenType::BangEqual),
    ("//", TokenType::Comment),
    ("==", TokenType::EqualEqual),
    (">=", TokenType::GreaterEqual),
    ("<=", TokenType::LessEqual),
];

pub fn build_lexemes_map(list: &[(&str, TokenType)]) -> HashMap<String, TokenType> {
    HashMap::from_iter(list.iter().map(|n| (n.0.to_owned(), n.1.clone())))
}
