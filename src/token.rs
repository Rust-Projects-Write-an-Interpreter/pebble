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

pub const KEYWORDS: &[(&str, Keyword)] = &[
    ("and", Keyword::And),
    ("class", Keyword::Class),
    ("else", Keyword::Else),
    ("false", Keyword::False),
    ("for", Keyword::For),
    ("fun", Keyword::Fun),
    ("if", Keyword::If),
    ("nil", Keyword::Nil),
    ("or", Keyword::Or),
    ("print", Keyword::Print),
    ("return", Keyword::Return),
    ("super", Keyword::Super),
    ("this", Keyword::This),
    ("true", Keyword::True),
    ("var", Keyword::Var),
    ("while", Keyword::While),
];

impl Keyword {
    // This is the string representation of
    // a keyword, that is how a keyword appears
    // in the source code.
    pub fn as_str(&self) -> &'static str {
        KEYWORDS
            // Iterate through all keywords.
            .iter()
            // Find the element that corresponds
            // to the current variant.
            .find(|(_, k)| k == self)
            // Map the value we found,
            // dereferencing the &str.
            .map(|(s, _)| *s)
            // Return the Some value or crash.
            .expect("Keyword missing from KEYWORDS list")
    }

    #[cfg(test)]
    // Output the lexeme corresponding
    // to the variant.
    pub fn lexeme(&self) -> String {
        self.as_str().to_string()
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KEYWORDS
            // Iterate through all keywords.
            .iter()
            // Find the element that corresponds
            // to the current string.
            .find(|(name, _)| *name == s)
            // Map the value we found,
            // dereferencing the &Keyword.
            .map(|(_, k)| *k)
            // Convert the Option into a Result.
            .ok_or(())
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str().to_uppercase())
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
    Eof,
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
            Self::Eof => "EOF",
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: u16,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal)
    }
}

impl Token {
    pub const fn eof() -> Self {
        Self {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: TokenLiteral::Null,
            line: 1,
        }
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

    #[test]
    // Test that a keyword is converted
    // into the corresponding string.
    fn test_keyword_as_str() {
        assert_eq!(Keyword::While.as_str(), "while");
    }

    #[test]
    // Test that an Eof token can
    // be correctly created.
    fn test_eof() {
        assert_eq!(
            Token::eof(),
            Token {
                ttype: TokenType::Eof,
                lexeme: String::new(),
                literal: TokenLiteral::Null,
                line: 1,
            }
        );
    }
}
