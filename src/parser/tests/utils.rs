use super::*;

#[test]
// Test that the method `current_token`
// returns the correct value.
fn test_current_token() {
    let tokens = vec![dummy_token(TokenType::Identifier)];
    let parser = Parser::new(tokens);

    assert_eq!(parser.current_token(), &dummy_token(TokenType::Identifier));
}

#[test]
// Test that consuming a token makes the
// next one the current token.
fn test_consume() {
    let tokens = vec![
        dummy_token(TokenType::Identifier),
        dummy_token(TokenType::Plus),
        Token::eof(),
    ];
    let mut parser = Parser::new(tokens);
    let previous = parser.consume();

    assert_eq!(previous, &dummy_token(TokenType::Identifier));
    assert_eq!(parser.current_token(), &dummy_token(TokenType::Plus));
}

#[test]
// Test that consuming `Eof` doesn't move
// the index and keeps the current
// token unchanged.
fn test_consume_end() {
    let tokens = vec![Token::eof()];
    let mut parser = Parser::new(tokens);
    parser.consume();
    let previous = parser.consume();

    assert_eq!(previous, &Token::eof());
    assert_eq!(parser.current_token(), &Token::eof());
}

#[test]
// Test that consuming an empty list of
// tokens returns `Eof`.
fn test_consume_empty() {
    let mut parser = Parser::new(Vec::new());
    let previous = parser.consume();

    assert_eq!(previous, &Token::eof());
    assert_eq!(parser.current_token(), &Token::eof());
}

#[test]
// Test that the method `check_current`
// works correctly when the current
// token matches the requested
// token type.
fn test_check_current_true() {
    let tokens = vec![dummy_token(TokenType::Plus)];
    let parser = Parser::new(tokens);

    assert!(parser.check_current(&TokenType::Plus));
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `check_current`
// works correctly when the current
// token doesn't match the requested
// token type.
fn test_check_current_false() {
    let tokens = vec![dummy_token(TokenType::Plus)];
    let parser = Parser::new(tokens);

    assert!(!parser.check_current(&TokenType::Number));
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `check_current`
// works correctly when the current
// token is the EOF.
fn test_check_current_end() {
    let tokens = vec![Token::eof()];
    let parser = Parser::new(tokens);

    // If we reached the end of the token sequence
    // we return false without checking the type.
    assert!(!parser.check_current(&TokenType::Eof));
    assert_eq!(parser.index, 0);
}

#[test]
// Check that the method `match_current` works
// when the type of the current token is
// included in the matching list.
fn test_match_current_true() {
    let tokens = vec![
        dummy_token(TokenType::Number),
        dummy_token(TokenType::Identifier),
    ];
    let mut parser = Parser::new(tokens);
    let matched = parser.match_current(&[TokenType::Number, TokenType::Plus]);

    assert_eq!(matched, Some(dummy_token(TokenType::Number)).as_ref());
    assert_eq!(parser.index, 1);
}

#[test]
// Check that the method `match_current` works
// when the type of the current token is not
// included in the matching list.
fn test_match_current_false() {
    let tokens = vec![dummy_token(TokenType::Identifier), Token::eof()];
    let mut parser = Parser::new(tokens);
    let matched = parser.match_current(&[TokenType::Number]);

    assert_eq!(matched, None);
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `expect` consumes
// and returns the current token when its
// type matches the expected one.
fn test_expect_match() {
    let tokens = vec![
        dummy_token(TokenType::Semicolon),
        dummy_token(TokenType::Identifier),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.expect(&TokenType::Semicolon, "Expected ';'");

    assert_eq!(result, Ok(&dummy_token(TokenType::Semicolon)));
    assert_eq!(parser.index, 1);
}

#[test]
// Test that the method `expect` returns
// an error with the given message and the
// current token when the types don't match.
fn test_expect_no_match() {
    let tokens = vec![dummy_token(TokenType::Identifier), Token::eof()];
    let mut parser = Parser::new(tokens);
    let result = parser.expect(&TokenType::Semicolon, "Expected ';'");

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Expected ';'"),
            token: dummy_token(TokenType::Identifier)
        })
    );
    assert_eq!(parser.index, 0);
}

#[test]
// Test that the method `expect` returns
// an error when the parser reached `Eof`,
// even if `Eof` is the expected type.
fn test_expect_end() {
    let tokens = vec![Token::eof()];
    let mut parser = Parser::new(tokens);
    let result = parser.expect(&TokenType::Eof, "Expected end");

    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("Expected end"),
            token: Token::eof()
        })
    );
    assert_eq!(parser.index, 0);
}
