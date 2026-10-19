use super::*;

#[test]
// Test that `EqualEqual` returns `true`
// for two equal numbers.
fn test_equal_equal_same_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `EqualEqual` returns `false`
// for two different numbers.
fn test_equal_equal_different_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `EqualEqual` returns `true`
// for two equal strings.
fn test_equal_equal_same_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::EqualEqual,
        Expression::Literal(Literal::String(String::from("StringA"))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `EqualEqual` returns `false`
// for two different strings.
fn test_equal_equal_different_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::EqualEqual,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `EqualEqual` returns `true`
// for two equal booleans.
fn test_equal_equal_same_booleans() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Boolean(true)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `EqualEqual` returns `false`
// for two different booleans.
fn test_equal_equal_different_booleans() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Boolean(true)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Boolean(false)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `EqualEqual` returns `true`
// when both operands are `nil`.
fn test_equal_equal_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Nil),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `EqualEqual` returns `false`
// for a number and a string with the
// same content.
fn test_equal_equal_number_string() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::String(String::from("42"))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `EqualEqual` returns `false`
// for `nil` and `false`, even though
// both are falsey.
fn test_equal_equal_nil_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Boolean(false)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `EqualEqual` returns `false`
// for zero and `false`.
fn test_equal_equal_zero_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(0.0)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Boolean(false)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `EqualEqual` compares the
// values of its operands and not their
// structure.
fn test_equal_equal_expression_operand() {
    let expression = dummy_binary_expression(
        dummy_binary_expression(
            Expression::Literal(Literal::Number(6.0)),
            TokenType::Star,
            Expression::Literal(Literal::Number(7.0)),
        ),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that an error in an operand
// is returned by `EqualEqual`.
fn test_equal_equal_propagates_error() {
    let expression = dummy_binary_expression(
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
        TokenType::EqualEqual,
        Expression::Literal(Literal::Nil),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `BangEqual` returns `false`
// for two equal numbers.
fn test_bang_equal_same_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::BangEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `BangEqual` returns `true`
// for two different numbers.
fn test_bang_equal_different_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::BangEqual,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `BangEqual` returns `false`
// for two equal strings.
fn test_bang_equal_same_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::BangEqual,
        Expression::Literal(Literal::String(String::from("StringA"))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `BangEqual` returns `true`
// for two different strings.
fn test_bang_equal_different_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::BangEqual,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `BangEqual` returns `false`
// for two equal booleans.
fn test_bang_equal_same_booleans() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Boolean(true)),
        TokenType::BangEqual,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `BangEqual` returns `true`
// for two different booleans.
fn test_bang_equal_different_booleans() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Boolean(true)),
        TokenType::BangEqual,
        Expression::Literal(Literal::Boolean(false)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `BangEqual` returns `false`
// when both operands are `nil`.
fn test_bang_equal_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::BangEqual,
        Expression::Literal(Literal::Nil),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `BangEqual` returns `true`
// for a number and a string with the
// same content.
fn test_bang_equal_number_string() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::BangEqual,
        Expression::Literal(Literal::String(String::from("42"))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `BangEqual` returns `true`
// for `nil` and `false`.
fn test_bang_equal_nil_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::BangEqual,
        Expression::Literal(Literal::Boolean(false)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that an error in an operand
// is returned by `BangEqual`.
fn test_bang_equal_propagates_error() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::BangEqual,
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}
