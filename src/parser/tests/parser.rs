use super::*;

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
