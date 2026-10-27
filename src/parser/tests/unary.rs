use super::*;

#[test]
// Test that the method `unary` works
// correctly with a token `Minus` in front
// of a number.
fn test_minus() {
    let tokens = vec![
        dummy_token(TokenType::Minus),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.4),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.unary();

    assert_eq!(
        result,
        Ok(Expression::Unary {
            operator: dummy_token(TokenType::Minus),
            right: Box::new(Expression::Literal(Literal::Number(123.4)))
        })
    );
    assert_eq!(parser.index, 2);
}

#[test]
// Test that the method `unary` works
// correctly with a token `Bang` in front
// of a number.
fn test_bang() {
    let tokens = vec![
        dummy_token(TokenType::Bang),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.4),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.unary();

    assert_eq!(
        result,
        Ok(Expression::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expression::Literal(Literal::Number(123.4)))
        })
    );
    assert_eq!(parser.index, 2);
}

#[test]
// Test that the method `unary` works
// correctly with a double token `Bang` in front
// of a number.
fn test_double_bang() {
    let tokens = vec![
        dummy_token(TokenType::Bang),
        dummy_token(TokenType::Bang),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.4),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.unary();

    assert_eq!(
        result,
        Ok(Expression::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expression::Unary {
                operator: dummy_token(TokenType::Bang),
                right: Box::new(Expression::Literal(Literal::Number(123.4)))
            }),
        })
    );
    assert_eq!(parser.index, 3);
}
