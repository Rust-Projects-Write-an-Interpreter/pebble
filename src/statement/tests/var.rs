use super::*;

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// variable definition statement with
// a correct expression: a number.
fn test_correct() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // The value we are going to use.
    let value = 42.0;

    // The statement we are going to process.
    let statement = Statement::Var(
        var_name.clone(),
        Expression::Literal(Literal::Number(value)),
    );

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert!(output.is_empty());
    assert_eq!(env.get(&var_name).unwrap(), Value::Number(value));
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// variable definition statement with
// an incorrect expression: the name of
// an undefined variable.
fn test_incorrect() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // A token for the undefined variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: String::from("undefined"),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process.
    let statement = Statement::Var(var_name.clone(), Expression::Variable(identifier_token));

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert!(matches!(result, Err(EvaluationError { .. })));
    assert!(output.is_empty());
    assert!(matches!(env.get(&var_name), Err(EnvironmentError { .. })));
}

