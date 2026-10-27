use super::*;

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains reserved word `true`.
fn test_true() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::True)),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(result, Ok(Expression::Literal(Literal::Boolean(true))));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains reserved word `false`.
fn test_false() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::False)),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(result, Ok(Expression::Literal(Literal::Boolean(false))));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains reserved word `nil`.
fn test_nil() {
    let tokens = vec![dummy_token(TokenType::Reserved(Keyword::Nil)), Token::eof()];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(result, Ok(Expression::Literal(Literal::Nil)));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains a reserved word that is
// currently unknown.
fn test_unknown_reserved() {
    let tokens = vec![
        dummy_token(TokenType::Reserved(Keyword::Class)),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid primary token"),
            token: dummy_token(TokenType::Reserved(Keyword::Class))
        })
    );
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains an integer number.
fn test_number_integer() {
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
    let result = parser.primary();

    assert_eq!(result, Ok(Expression::Literal(Literal::Number(123.0))));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains a float number.
fn test_number_float() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.4),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(result, Ok(Expression::Literal(Literal::Number(123.4))));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `primary` works
// correctly on a list of tokens that
// contains a string.
fn test_string() {
    let tokens = vec![
        Token {
            ttype: TokenType::String,
            lexeme: String::new(),
            literal: TokenLiteral::String(String::from("Some string")),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(
        result,
        Ok(Expression::Literal(Literal::String(String::from(
            "Some string"
        ))))
    );
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `primary` works
// correctly when the list of tokens contains
// a group.
fn test_group() {
    let tokens = vec![
        dummy_token(TokenType::LParen),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.4),
            line: 1,
        },
        dummy_token(TokenType::RParen),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(
        result,
        Ok(Expression::Group(Box::new(Expression::Literal(
            Literal::Number(123.4)
        ))))
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `primary` works
// correctly when the list of tokens contains
// a group that wraps a unary expression.
fn test_group_can_contain_unary() {
    let tokens = vec![
        dummy_token(TokenType::LParen),
        dummy_token(TokenType::Bang),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.4),
            line: 1,
        },
        dummy_token(TokenType::RParen),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.primary();

    assert_eq!(
        result,
        Ok(Expression::Group(Box::new(Expression::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expression::Literal(Literal::Number(123.4)))
        })))
    );
    assert_eq!(parser.index, 4);
}
