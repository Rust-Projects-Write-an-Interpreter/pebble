use super::*;

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// print statement with a correct
// expression: the name of an existing
// variable.
fn test_correct() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // The value of the variable
    // we are going to use.
    let var_value = Value::Number(42.0);

    // Define a variable in the environment.
    env.define(var_name.clone(), var_value.clone());

    // A token for the printed variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: var_name.clone(),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process
    // prints the variable.
    let statement = Statement::Print(Expression::Variable(identifier_token));

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert_eq!(output, vec![var_value.clone()]);
    assert_eq!(env.get(&var_name).unwrap(), var_value);
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// print statement with an incorrect
// expression: the name of an undefined
// variable.
fn test_incorrect() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // A token for the printed variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: var_name.clone(),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process
    // prints the variable.
    let statement = Statement::Print(Expression::Variable(identifier_token));

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert!(matches!(result, Err(EvaluationError { .. })));
    assert!(output.is_empty());
    assert!(matches!(env.get(&var_name), Err(EnvironmentError { .. })));
}
