use super::*;

#[test]
// Test that the method `factor` works
// correctly with two primaries and
// a token `Slash`.
fn test_slash() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Slash),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.factor();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::Slash),
            right: Box::new(Expression::Literal(Literal::Number(456.0)))
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `factor` works
// correctly with two primaries and
// a token `Star`.
fn test_star() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Star),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.factor();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::Star),
            right: Box::new(Expression::Literal(Literal::Number(456.0)))
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `factor` works
// correctly with multiple primaries
// chained by either Slash or Star
// tokens.
fn test_chain() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Star),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        dummy_token(TokenType::Slash),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(789.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.factor();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(123.0))),
                operator: dummy_token(TokenType::Star),
                right: Box::new(Expression::Literal(Literal::Number(456.0)))
            }),
            operator: dummy_token(TokenType::Slash),
            right: Box::new(Expression::Literal(Literal::Number(789.0)))
        })
    );
    assert_eq!(parser.index, 5);
}
