use super::*;

#[test]
// Test that `Star` with a string
// operand returns an error.
fn test_star_string() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("Some string"))),
        TokenType::Star,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Slash` with a string
// operand returns an error.
fn test_slash_string() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("Some string"))),
        TokenType::Slash,
        Expression::Literal(Literal::Number(42.0)),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Minus` cannot be
// used between strings.
fn test_minus_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::Minus,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
}

#[test]
// Test that `Plus` can be
// used between strings.
fn test_plus_strings() {
    let expression = dummy_binary_expression(
        Expression::Literal(Literal::String(String::from("StringA"))),
        TokenType::Plus,
        Expression::Literal(Literal::String(String::from("StringB"))),
    );

    assert_eq!(
        evaluate(&expression),
        Ok(Value::String(String::from("StringAStringB")))
    );
}
