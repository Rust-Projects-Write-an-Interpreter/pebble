use super::*;

#[test]
// Test that the method `term` works
// correctly with two primaries and
// a token `Plus`.
fn test_plus() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Plus),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.term();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::Plus),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `term` works
// correctly with two primaries and
// a token `Minus`.
fn test_minus() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Minus),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.term();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::Minus),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `term` works
// correctly with multiple primaries
// chained by either Plus or Minus
// tokens.
fn test_chain() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Minus),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        dummy_token(TokenType::Plus),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(789.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.term();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(123.0))),
                operator: dummy_token(TokenType::Minus),
                right: Box::new(Expression::Literal(Literal::Number(456.0))),
            }),
            operator: dummy_token(TokenType::Plus),
            right: Box::new(Expression::Literal(Literal::Number(789.0))),
        })
    );
    assert_eq!(parser.index, 5);
}
