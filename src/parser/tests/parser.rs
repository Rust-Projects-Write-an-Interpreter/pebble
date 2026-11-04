use super::*;

use crate::expression::{Expression, Literal};

#[test]
// Test that the method `new` works
// correctly when the list of provided
// tokens ends with an Eof.
fn test_new_last_token_is_eof() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);

    // Consume the first token.
    parser.consume();

    assert_eq!(parser.current_token(), &Token::eof());
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `new` works
// correctly when the list of provided
// tokens doesn't end with an Eof.
fn test_new_last_token_is_not_eof() {
    let tokens = vec![Token {
        ttype: TokenType::Number,
        lexeme: String::new(),
        literal: TokenLiteral::Number(123.0),
        line: 1,
    }];
    let mut parser = Parser::new(tokens);

    // Consume the first and only token.
    parser.consume();

    assert_eq!(parser.current_token(), &Token::eof());
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `new` works
// correctly when the list of provided
// tokens is empty.
fn test_new_no_tokens() {
    let tokens = Vec::new();
    let mut parser = Parser::new(tokens);

    // Try to consume a token.
    parser.consume();

    assert_eq!(parser.current_token(), &Token::eof());
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `parse` works
// correctly on a list of valid statements.
fn test_parse_valid_statements() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Reserved(Keyword::True)),
        dummy_token(TokenType::Semicolon),
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Reserved(Keyword::False)),
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert_eq!(
        result,
        Ok(vec![
            Statement::Print(Expression::Literal(Literal::Boolean(true))),
            Statement::Print(Expression::Literal(Literal::Boolean(false))),
        ])
    );
    assert_eq!(parser.index, 6);
}

#[test]
// Test that the method `parse` works
// correctly on a list that contains
// an invalid statement.
fn test_parse_invalid_statement() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Reserved(Keyword::True)),
        dummy_token(TokenType::Semicolon),
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Reserved(Keyword::False)),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid statement"),
            token: dummy_token(TokenType::Eof)
        })
    );
    assert_eq!(parser.index, 5);
}
