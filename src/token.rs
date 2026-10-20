use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(Self::And),
            "class" => Ok(Self::Class),
            "else" => Ok(Self::Else),
            "false" => Ok(Self::False),
            "for" => Ok(Self::For),
            "fun" => Ok(Self::Fun),
            "if" => Ok(Self::If),
            "nil" => Ok(Self::Nil),
            "or" => Ok(Self::Or),
            "print" => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super" => Ok(Self::Super),
            "this" => Ok(Self::This),
            "true" => Ok(Self::True),
            "var" => Ok(Self::Var),
            "while" => Ok(Self::While),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::And => "and",
            Self::Class => "class",
            Self::Else => "else",
            Self::False => "false",
            Self::For => "for",
            Self::Fun => "fun",
            Self::If => "if",
            Self::Nil => "nil",
            Self::Or => "or",
            Self::Print => "print",
            Self::Return => "return",
            Self::Super => "super",
            Self::This => "this",
            Self::True => "true",
            Self::Var => "var",
            Self::While => "while",
        };
        write!(f, "{}", name.to_uppercase())
    }
}

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
    Identifier,
    LBrace,
    LParen,
    Less,
    LessEqual,
    Minus,
    Newline,
    Number,
    Plus,
    RBrace,
    RParen,
    Reserved(Keyword),
    Semicolon,
    Slash,
    Star,
    String,
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
            Self::Identifier => "IDENTIFIER",
            Self::LBrace => "LEFT_BRACE",
            Self::LParen => "LEFT_PAREN",
            Self::Less => "LESS",
            Self::LessEqual => "LESS_EQUAL",
            Self::Minus => "MINUS",
            Self::Newline => "NEWLINE",
            Self::Number => "NUMBER",
            Self::Plus => "PLUS",
            Self::RBrace => "RIGHT_BRACE",
            Self::RParen => "RIGHT_PAREN",
            Self::Reserved(s) => &s.to_string(),
            Self::Semicolon => "SEMICOLON",
            Self::Slash => "SLASH",
            Self::Star => "STAR",
            Self::String => "STRING",
            Self::Whitespace => "WHITESPACE",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenLiteral {
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Number(n) if n.fract() == 0.0 => write!(f, "{n:.1}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that TokenLiteral::Number is
    // displayed correctly when it contains
    // an integer (the decimal part is zero).
    fn test_token_literal_number_integer() {
        let literal = TokenLiteral::Number(42.0);

        assert_eq!(literal.to_string(), "42.0");
    }

    #[test]
    // Test that TokenLiteral::Number is
    // displayed correctly when it contains
    // a float (the decimal part is not zero).
    fn test_token_literal_number_float() {
        let literal = TokenLiteral::Number(42.42);

        assert_eq!(literal.to_string(), "42.42");
    }

    #[test]
    // Test that a valid keyword string
    // is converted into the corresponding
    // Keyword variant.
    fn test_keyword_from_str_valid() {
        assert_eq!("while".parse::<Keyword>(), Ok(Keyword::While));
    }

    #[test]
    // Test that a string that is not
    // a keyword returns an error.
    fn test_keyword_from_str_unknown() {
        assert_eq!("foo".parse::<Keyword>(), Err(()));
    }

    #[test]
    // Test that the conversion is
    // case-sensitive, so the uppercase
    // version of a keyword is not valid.
    fn test_keyword_from_str_case_sensitive() {
        assert_eq!("WHILE".parse::<Keyword>(), Err(()));
    }

    #[test]
    // Test that an empty string
    // returns an error.
    fn test_keyword_from_str_empty() {
        assert_eq!("".parse::<Keyword>(), Err(()));
    }
}
