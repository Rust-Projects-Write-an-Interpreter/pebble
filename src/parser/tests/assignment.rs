use super::*;

#[test]
// Test that the method `assignment` works
// correctly with an expression in the form
// var = PRIMARY.
fn test_assignment_valid() {
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: String::from("varname"),
        literal: TokenLiteral::Null,
        line: 1,
    };

    let tokens = vec![
        identifier_token.clone(),
        dummy_token(TokenType::Equal),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(42.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.assignment();

    assert_eq!(
        result,
        Ok(Expression::Assignment {
            identifier: identifier_token,
            expression: Box::new(Expression::Literal(Literal::Number(42.0)))
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `assignment` works
// correctly with an expression that is
// missing the equal sign.
fn test_assignment_missing_equal() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(42.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.assignment();

    assert_eq!(result, Ok(Expression::Literal(Literal::Number(42.0))));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `assignment` returns
// the correct error when it parses an expression
// whose left side is an invalid assignment target.
fn test_assignment_invalid_target() {
    let tokens = vec![
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(0.0),
            line: 1,
        },
        dummy_token(TokenType::Equal),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(42.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.assignment();

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Invalid assignment target"),
            token: dummy_token(TokenType::Equal),
        })
    );
    assert_eq!(parser.index, 3);
}

#[test]
// Test that the method `assignment` works
// correctly with multiple assignments
// chained by equality tokens in the form
// var_a = var_b = PRIMARY.
fn test_assignment_chain() {
    let identifier_token_a = Token {
        ttype: TokenType::Identifier,
        lexeme: String::from("var_a"),
        literal: TokenLiteral::Null,
        line: 1,
    };

    let identifier_token_b = Token {
        ttype: TokenType::Identifier,
        lexeme: String::from("var_b"),
        literal: TokenLiteral::Null,
        line: 1,
    };

    let tokens = vec![
        identifier_token_a.clone(),
        dummy_token(TokenType::Equal),
        identifier_token_b.clone(),
        dummy_token(TokenType::Equal),
        Token {
            ttype: TokenType::Number,
            lexeme: String::new(),
            literal: TokenLiteral::Number(42.0),
            line: 1,
        },
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.assignment();

    assert_eq!(
        result,
        Ok(Expression::Assignment {
            identifier: identifier_token_a,
            expression: Box::new(Expression::Assignment {
                identifier: identifier_token_b,
                expression: Box::new(Expression::Literal(Literal::Number(42.0)))
            })
        })
    );
    assert_eq!(parser.index, 5);
}
