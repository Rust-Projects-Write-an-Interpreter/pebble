use super::*;

#[test]
// Test that the method `equality` works
// correctly with two primaries and
// a token `BangEqual`.
fn test_bang_equal() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::BangEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.equality();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::BangEqual),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `equality` works
// correctly with two primaries and
// a token `EqualEqual`.
fn test_equal_equal() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::EqualEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.equality();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: dummy_token(TokenType::EqualEqual),
            right: Box::new(Expression::Literal(Literal::Number(456.0))),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `equality` works
// correctly with multiple primaries
// chained by equality tokens.
fn test_chain() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(123.0),
            line: 1,
        },
        dummy_token(TokenType::BangEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(456.0),
            line: 1,
        },
        dummy_token(TokenType::EqualEqual),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(789.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.equality();

    assert_eq!(
        result,
        Ok(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(123.0))),
                operator: dummy_token(TokenType::BangEqual),
                right: Box::new(Expression::Literal(Literal::Number(456.0))),
            }),
            operator: dummy_token(TokenType::EqualEqual),
            right: Box::new(Expression::Literal(Literal::Number(789.0))),
        })
    );
    assert_eq!(parser.index, 5);
}
