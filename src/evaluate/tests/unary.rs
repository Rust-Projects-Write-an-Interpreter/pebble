use super::*;

use crate::environment::Environment;

#[test]
// Test that `Bang` in front of
// `true` returns `false`.
fn test_bang_true() {
    let mut env = Environment::new();

    let expression =
        dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Boolean(true)));

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Bang` in front of
// `false` returns `true`.
fn test_bang_false() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Bang,
        Expression::Literal(Literal::Boolean(false)),
    );

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Boolean(true)));
}

#[test]
// Test that `Bang` in front of
// a number returns `false`.
fn test_bang_number() {
    let mut env = Environment::new();

    let expression =
        dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Number(42.0)));

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Bang` in front of
// the number zero returns `false`.
fn test_bang_zero() {
    let mut env = Environment::new();

    let expression =
        dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Number(0.0)));

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Boolean(false)));
}

#[test]
// Test that `Bang` in front of
// `nil` returns `true`.
fn test_bang_nil() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Nil));

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Boolean(true)));
}

#[test]
// Test that a double `Bang` in front
// of a boolean returns the original boolean.
fn test_double_bang_boolean() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Bang,
        dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Boolean(true))),
    );

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Boolean(true)));
}

#[test]
// Test that `Minus` in front of
// a number negates it.
fn test_minus_number() {
    let mut env = Environment::new();

    let expression =
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Number(42.0)));

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Number(-42.0)));
}

#[test]
// Test that a double `Minus` in front
// of a number returns the original number.
fn test_double_minus_number() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Minus,
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Number(42.0))),
    );

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Number(42.0)));
}

#[test]
// Test that `Minus` in front of
// a group negates the value of the group.
fn test_minus_group() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Minus,
        Expression::Group(Box::new(Expression::Literal(Literal::Number(42.0)))),
    );

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Number(-42.0)));
}

#[test]
// Test that `Minus` in front of
// a boolean returns an error.
fn test_minus_boolean() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Minus,
        Expression::Literal(Literal::Boolean(true)),
    );

    assert!(matches!(
        evaluate(&expression, &mut env),
        Err(EvaluationError { .. })
    ));
}

#[test]
// Test that `Minus` in front of
// a string returns an error.
fn test_minus_string() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Minus,
        Expression::Literal(Literal::String(String::from("Some string"))),
    );

    assert!(matches!(
        evaluate(&expression, &mut env),
        Err(EvaluationError { .. })
    ));
}

#[test]
// Test that `Minus` in front of
// `nil` returns an error.
fn test_minus_nil() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil));

    assert!(matches!(
        evaluate(&expression, &mut env),
        Err(EvaluationError { .. })
    ));
}

#[test]
// Test that an error in the right
// operand is returned by the
// outer unary expression.
fn test_propagates_error() {
    let mut env = Environment::new();

    let expression = dummy_unary_expression(
        TokenType::Bang,
        dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
    );

    assert!(matches!(
        evaluate(&expression, &mut env),
        Err(EvaluationError { .. })
    ));
}
