use super::*;

#[test]
// Test that `Star` works correctly
// with numbers that have no decimals.
// A float with no decimal part is
// the way we represent an integer.
fn test_star_integer() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(6.0)),
        TokenType::Star,
        Expression::Literal(Literal::Number(7.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
}

#[test]
// Test that `Star` works correctly
// with float numbers.
fn test_star_float() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(2.5)),
        TokenType::Star,
        Expression::Literal(Literal::Number(4.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(10.0)));
}

#[test]
// Test that `Slash` between two
// numbers divides them.
fn test_slash() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Slash,
        Expression::Literal(Literal::Number(6.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(7.0)));
}

#[test]
// Test that `Slash` returns a float
// when the division is not exact.
fn test_slash_float_result() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(7.0)),
        TokenType::Slash,
        Expression::Literal(Literal::Number(2.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(3.5)));
}

#[test]
// Test that a chain of binary expressions
// is evaluated following the structure
// of the tree, e.g. (42 / 6) * 2.
fn test_chain() {
    let expression = dummy_binary_expression(
        dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Slash,
            Expression::Literal(Literal::Number(6.0)),
        ),
        TokenType::Star,
        Expression::Literal(Literal::Number(2.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(14.0)));
}

#[test]
// Test that a group used as an operand
// is evaluated before the binary
// expression, e.g. 42 / (6 * 2).
fn test_group_operand() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Slash,
        Expression::Group(Box::new(dummy_binary_expression(
            Expression::Literal(Literal::Number(6.0)),
            TokenType::Star,
            Expression::Literal(Literal::Number(2.0)),
        ))),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(3.5)));
}

#[test]
// Test that a unary expression used
// as an operand is evaluated before
// the binary expression, e.g. -6 * 7.
fn test_unary_operand() {
    let expression = dummy_binary_expression(
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Number(6.0))),
        TokenType::Star,
        Expression::Literal(Literal::Number(7.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(-42.0)));
}

#[test]
// Test that an error in the left
// operand is returned by the
// binary expression.
fn test_propagates_error_left() {
    let expression = dummy_binary_expression(
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
        TokenType::Star,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that an error in the right
// operand is returned by the
// binary expression.
fn test_propagates_error_right() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Star,
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Star` with a boolean
// operand returns an error.
fn test_star_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Star,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Star` with a `nil`
// operand returns an error.
fn test_star_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::Star,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Slash` with a boolean
// operand returns an error.
fn test_slash_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Slash,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Slash` with a `nil`
// operand returns an error.
fn test_slash_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::Slash,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Minus` works correctly
// with numbers that have no decimals.
fn test_minus_integer() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(50.0)),
        TokenType::Minus,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
}

#[test]
// Test that `Minus` works correctly
// with float numbers.
fn test_minus_float() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(5.5)),
        TokenType::Minus,
        Expression::Literal(Literal::Number(2.25)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(3.25)));
}

#[test]
// Test that `Minus` returns a negative
// number when the right operand is
// greater than the left one.
fn test_minus_negative_result() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(8.0)),
        TokenType::Minus,
        Expression::Literal(Literal::Number(50.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(-42.0)));
}

#[test]
// Test that `Minus` with a boolean
// operand returns an error.
fn test_minus_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Minus,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Minus` with a `nil`
// operand returns an error.
fn test_minus_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::Minus,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Plus` works correctly
// with numbers that have no decimals.
fn test_plus_integer() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(34.0)),
        TokenType::Plus,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
}

#[test]
// Test that `Plus` works correctly
// with float numbers.
fn test_plus_float() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(1.25)),
        TokenType::Plus,
        Expression::Literal(Literal::Number(2.5)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Number(3.75)));
}

#[test]
// Test that `Plus` with a boolean
// operand returns an error.
fn test_plus_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Plus,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Plus` with a `nil`
// operand returns an error.
fn test_plus_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::Plus,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}
