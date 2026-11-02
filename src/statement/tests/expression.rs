use super::*;

#[test]
// Test that the function `execute` works
// correctly when the statement is an
// expression statement with a correct
// expression: an assignment to an
// existing variable.
fn test_correct() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // The value of the variable
    // we are going to use.
    let var_value = 42.0;

    // Define a variable in the environment.
    env.define(var_name.clone(), Value::Boolean(true));

    // A token for the assigned variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: var_name.clone(),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process
    // contains an assignment expression.
    let statement = Statement::Expression(Expression::Assignment {
        identifier: identifier_token,
        expression: Box::new(Expression::Literal(Literal::Number(var_value))),
    });

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert!(output.is_empty());
    assert_eq!(env.get(&var_name).unwrap(), Value::Number(var_value));
}

#[test]
// Test that the function `execute` works
// correctly when the statement is an
// expression statement with an incorrect
// expression: an assignment to an
// undefined variable.
fn test_incorrect() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // The value of the variable
    // we are going to use.
    let var_value = 42.0;

    // A token for the assigned variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: var_name.clone(),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process
    // contains an assignment expression.
    let statement = Statement::Expression(Expression::Assignment {
        identifier: identifier_token,
        expression: Box::new(Expression::Literal(Literal::Number(var_value))),
    });

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert!(matches!(result, Err(EvaluationError { .. })));
    assert!(output.is_empty());
    assert!(matches!(env.get(&var_name), Err(EnvironmentError { .. })));
}
