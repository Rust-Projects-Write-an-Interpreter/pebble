use super::*;

use crate::expression::{Expression, Literal};

#[test]
// Test that the method `statement` works
// correctly on a statement in the form
// `print EXPRESSION;`.
fn test_print_valid_statement() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Reserved(Keyword::True)),
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Ok(Statement::Print(Expression::Literal(Literal::Boolean(
            true
        ))))
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `statement` returns
// the correct error when it parses a print
// statement that contains an invalid expression.
fn test_print_invalid_expression() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Equal),
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid primary token"),
            token: dummy_token(TokenType::Equal)
        })
    );
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `statement` returns
// the correct error when it parses a
// statement that does not start with `print`.
fn test_print_missing_keyword() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::True)),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid statement"),
            token: dummy_token(TokenType::Reserved(Keyword::True)),
        })
    );
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `statement` returns
// the correct error when it parses a
// statement that does not end with a semicolon.
fn test_print_missing_semicolon() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Print)),
        dummy_token(TokenType::Reserved(Keyword::True)),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid statement"),
            token: dummy_token(TokenType::Eof)
        })
    );
    assert_eq!(parser.index, 2);
}
