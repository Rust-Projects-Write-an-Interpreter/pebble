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

#[test]
// Test that the method `statement` works
// correctly on a statement in the form
// `EXPRESSION;`.
fn test_expression_valid_statement() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::True)),
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Ok(Statement::Expression(Expression::Literal(
            Literal::Boolean(true)
        )))
    );
    assert_eq!(parser.index, 2);
}

#[test]
// Test that the method `statement` returns
// the correct error when it parses an expression
// statement that contains an invalid expression.
fn test_expression_invalid_expression() {
    let tokens = vec![
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
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `statement` works
// correctly on a statement in the form
// `var IDENTIFIER = EXPRESSION;`.
fn test_var_valid_statement_with_expression() {
    let varname = String::from("somevar");

    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Var)),
        Token {
            ttype: TokenType::Identifier,
            lexeme: varname.clone(),
            literal: TokenLiteral::Null,
            line: 1,
        },
        dummy_token(TokenType::Equal),
        dummy_token(TokenType::Reserved(Keyword::True)),
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Ok(Statement::Var(
            varname,
            Expression::Literal(Literal::Boolean(true))
        ))
    );
    assert_eq!(parser.index, 5);
}

#[test]
// Test that the method `statement` works
// correctly on a statement in the form
// `var IDENTIFIER;`.
fn test_var_valid_statement_without_expression() {
    let varname = String::from("somevar");

    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Var)),
        Token {
            ttype: TokenType::Identifier,
            lexeme: varname.clone(),
            literal: TokenLiteral::Null,
            line: 1,
        },
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Ok(Statement::Var(varname, Expression::Literal(Literal::Nil)))
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `statement` returns
// the correct error when it parses a var
// statement without an identifier.
fn test_var_missing_identifier() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Var)),
        dummy_token(TokenType::Equal),
        dummy_token(TokenType::Reserved(Keyword::True)),
        dummy_token(TokenType::Semicolon),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.statement();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid statement"),
            token: dummy_token(TokenType::Equal)
        })
    );
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `statement` returns
// the correct error when it parses a var
// statement with an invalid expression.
fn test_var_invalid_expression() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Var)),
        Token {
            ttype: TokenType::Identifier,
            lexeme: String::from("varname"),
            literal: TokenLiteral::Null,
            line: 1,
        },
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
            token: dummy_token(TokenType::Semicolon)
        })
    );
    assert_eq!(parser.index, 3);
}
