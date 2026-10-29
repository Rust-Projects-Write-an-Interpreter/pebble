use super::*;

use crate::environment::Environment;

#[test]
// Test that a defined variable
// returns its value.
fn test_variable_defined() {
    let mut env = Environment::new();
    env.define(String::from("somevar"), Value::Number(42.0));

    let expression = dummy_variable_expression("somevar", 1);

    assert_eq!(evaluate(&expression, &mut env), Ok(Value::Number(42.0)));
}

#[test]
// Test that an undefined variable
// returns an error with the message
// of the environment and the line
// of the variable token.
fn test_variable_undefined() {
    let mut env = Environment::new();

    let expression = dummy_variable_expression("somevar", 42);

    assert_eq!(
        evaluate(&expression, &mut env),
        Err(EvaluationError {
            message: String::from("Undefined variable somevar."),
            line: 42,
        })
    );
}
