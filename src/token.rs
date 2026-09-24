use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Bang,
    BangEqual,
    Comma,
    Dot,
    Equal,
    EqualEqual,
    LBrace,
    LParen,
    Minus,
    Plus,
    RBrace,
    RParen,
    Semicolon,
    Star,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::Bang => "BANG",
            Self::BangEqual => "BANG_EQUAL",
            Self::Comma => "COMMA",
            Self::Dot => "DOT",
            Self::Equal => "EQUAL",
            Self::EqualEqual => "EQUAL_EQUAL",
            Self::LBrace => "LEFT_BRACE",
            Self::LParen => "LEFT_PAREN",
            Self::Minus => "MINUS",
            Self::Plus => "PLUS",
            Self::RBrace => "RIGHT_BRACE",
            Self::RParen => "RIGHT_PAREN",
            Self::Semicolon => "SEMICOLON",
            Self::Star => "STAR",
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
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line 1] Error: {}", self.message)
    }
}

pub type TokenResult = Result<Token, TokenError>;

pub const SINGLE_CHARACTER_LEXEMES: &[(&str, TokenType)] = &[
    ("!", TokenType::Bang),
    (",", TokenType::Comma),
    (".", TokenType::Dot),
    ("=", TokenType::Equal),
    ("{", TokenType::LBrace),
    ("(", TokenType::LParen),
    ("-", TokenType::Minus),
    ("+", TokenType::Plus),
    ("}", TokenType::RBrace),
    (")", TokenType::RParen),
    (";", TokenType::Semicolon),
    ("*", TokenType::Star),
];

pub const MULTI_CHARACTER_LEXEMES: &[(&str, TokenType)] =
    &[("!=", TokenType::BangEqual), ("==", TokenType::EqualEqual)];

pub fn build_lexemes_map(list: &[(&str, TokenType)]) -> HashMap<String, TokenType> {
    HashMap::from_iter(list.iter().map(|n| (n.0.to_owned(), n.1.clone())))
}
