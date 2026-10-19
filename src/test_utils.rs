use crate::token::{Token, TokenLiteral, TokenType};

// Build a token of the given type
// with no lexeme and no literal.
pub fn dummy_token(ttype: TokenType) -> Token {
    Token {
        ttype,
        lexeme: String::new(),
        literal: TokenLiteral::Null,
        line: 1,
    }
}
