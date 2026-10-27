use super::*;

#[test]
// Test that the method `comparison` works
// correctly with two primaries and
// a token `Less`.
fn test_less() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Less),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.comparison();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::Less),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `comparison` works
// correctly with two primaries and
// a token `LessEqual`.
fn test_less_equal() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::LessEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.comparison();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::LessEqual),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `comparison` works
// correctly with two primaries and
// a token `Greater`.
fn test_greater() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Greater),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.comparison();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::Greater),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `comparison` works
// correctly with two primaries and
// a token `GreaterEqual`.
fn test_greater_equal() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::GreaterEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.comparison();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::GreaterEqual),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `comparison` works
// correctly with multiple primaries
// chained by comparison
// tokens.
fn test_chain() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::Less),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        dummy_token(TokenType::GreaterEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(789.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.comparison();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(123.0))),
                operator: dummy_token(TokenType::Less),
                right: Box::new(Expression::Literal(Literal::Number(456.0))),
            }),
            operator: dummy_token(TokenType::GreaterEqual),
            right: Box::new(Expression::Literal(Literal::Number(789.0))),
        })
    );
    assert_eq!(parser.index, 5);
}
