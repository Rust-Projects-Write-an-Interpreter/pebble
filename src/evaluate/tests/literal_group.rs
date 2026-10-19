use super::*;

#[test]
// Test that the literal `true`
// evaluates to a boolean value.
fn test_literal_true() {
    let expression = Expression::Literal(Literal::Boolean(true));

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
}

#[test]
// Test that the literal `false`
// evaluates to a boolean value.
fn test_literal_false() {
    let expression = Expression::Literal(Literal::Boolean(false));

    assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
}

#[test]
// Test that the literal `nil`
// evaluates to the value `Nil`.
fn test_literal_nil() {
    let expression = Expression::Literal(Literal::Nil);

    assert_eq!(evaluate(&expression), Ok(Value::Nil));
}

#[test]
// Test that an integer number literal
// evaluates to a number value.
fn test_literal_number_integer() {
    let expression = Expression::Literal(Literal::Number(42.0));

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
}

#[test]
// Test that a float number literal
// evaluates to a number value.
fn test_literal_number_float() {
    let expression = Expression::Literal(Literal::Number(42.5));

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.5)));
}

#[test]
// Test that a string literal
// evaluates to a string value.
fn test_literal_string() {
    let expression = Expression::Literal(Literal::String(String::from("Some string")));

    assert_eq!(
        evaluate(&expression),
        Ok(Value::String(String::from("Some string")))
    );
}

#[test]
// Test that a group evaluates
// to the value of the expression
// it contains.
fn test_group() {
    let expression = Expression::Group(Box::new(Expression::Literal(Literal::Number(42.0))));

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
}

#[test]
// Test that a nested group evaluates
// to the value of the innermost
// expression.
fn test_group_nested() {
    let expression = Expression::Group(Box::new(Expression::Group(Box::new(Expression::Literal(
        Literal::Number(42.0),
    )))));

    assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
}
