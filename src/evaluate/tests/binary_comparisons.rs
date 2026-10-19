use super::*;

#[test]
// Test that `Greater` returns `true`
// when the comparison holds.
fn test_greater_true() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(50.0)),
        TokenType::Greater,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `Greater` returns `false`
// when the comparison does not hold.
fn test_greater_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(8.0)),
        TokenType::Greater,
        Expression::Literal(Literal::Number(50.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Greater` between two
// equal numbers returns `false`.
fn test_greater_same_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Greater,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Greater` with a boolean
// operand returns an error.
fn test_greater_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Greater,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Greater` with a `nil`
// operand returns an error.
fn test_greater_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::Greater,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Greater` with two string
// operands returns an error.
fn test_greater_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::Greater,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `GreaterEqual` returns `true`
// when the comparison holds.
fn test_greater_equal_true() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(50.0)),
        TokenType::GreaterEqual,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `GreaterEqual` returns `false`
// when the comparison does not hold.
fn test_greater_equal_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(8.0)),
        TokenType::GreaterEqual,
        Expression::Literal(Literal::Number(50.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `GreaterEqual` between two
// equal numbers returns `true`.
fn test_greater_equal_same_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::GreaterEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `GreaterEqual` with a boolean
// operand returns an error.
fn test_greater_equal_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::GreaterEqual,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `GreaterEqual` with a `nil`
// operand returns an error.
fn test_greater_equal_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::GreaterEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `GreaterEqual` with two string
// operands returns an error.
fn test_greater_equal_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::GreaterEqual,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Less` returns `true`
// when the comparison holds.
fn test_less_true() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(8.0)),
        TokenType::Less,
        Expression::Literal(Literal::Number(50.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `Less` returns `false`
// when the comparison does not hold.
fn test_less_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(50.0)),
        TokenType::Less,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Less` between two
// equal numbers returns `false`.
fn test_less_same_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Less,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Less` with a boolean
// operand returns an error.
fn test_less_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::Less,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Less` with a `nil`
// operand returns an error.
fn test_less_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::Less,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Less` with two string
// operands returns an error.
fn test_less_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::Less,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `LessEqual` returns `true`
// when the comparison holds.
fn test_less_equal_true() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(8.0)),
        TokenType::LessEqual,
        Expression::Literal(Literal::Number(50.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `LessEqual` returns `false`
// when the comparison does not hold.
fn test_less_equal_false() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(50.0)),
        TokenType::LessEqual,
        Expression::Literal(Literal::Number(8.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that `LessEqual` between two
// equal numbers returns `true`.
fn test_less_equal_same_numbers() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::LessEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that `LessEqual` with a boolean
// operand returns an error.
fn test_less_equal_boolean() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Number(42.0)),
        TokenType::LessEqual,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `LessEqual` with a `nil`
// operand returns an error.
fn test_less_equal_nil() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::Nil),
        TokenType::LessEqual,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `LessEqual` with two string
// operands returns an error.
fn test_less_equal_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::LessEqual,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}
