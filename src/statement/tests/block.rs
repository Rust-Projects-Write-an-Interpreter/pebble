use super::*;

#[test]
// Test that the function `execute` works
// correctly when the statement is an
// empty block statement.
fn test_empty() {
    // The parent environment.
    let mut env = Environment::new();

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert!(output.is_empty());
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// block statement that contains correct
// statements.
fn test_correct() {
    // The parent environment.
    let mut env = Environment::new();

    // The value we are going to use.
    let value = 42.0;

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![Statement::Print(Expression::Literal(
        Literal::Number(value),
    ))]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert_eq!(output, vec![Value::Number(value)]);
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// block statement that contains incorrect
// statements.
fn test_incorrect() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // A token for the printed variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: var_name,
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![Statement::Print(Expression::Variable(
        identifier_token,
    ))]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert!(matches!(result, Err(EvaluationError { .. })));
    assert!(output.is_empty());
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// block statement that contains correct
// statements, returning the output
// in the correct order.
fn test_output_order() {
    // The parent environment.
    let mut env = Environment::new();

    // Three values we are going to use.
    let value1 = 41.0;
    let value2 = 42.0;
    let value3 = 43.0;

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![
        Statement::Print(Expression::Literal(Literal::Number(value1))),
        Statement::Print(Expression::Literal(Literal::Number(value2))),
        Statement::Print(Expression::Literal(Literal::Number(value3))),
    ]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert_eq!(
        output,
        vec![
            Value::Number(value1),
            Value::Number(value2),
            Value::Number(value3),
        ]
    );
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// block statement that contains correct
// and incorrect statements, stopping
// at the first incorrect one.
fn test_error_stops_execution() {
    // The parent environment.
    let mut env = Environment::new();

    // A token for the undefined variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: String::from("undefined"),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // Three values we are going to use.
    let value1 = 41.0;
    let value2 = 42.0;
    let value3 = 43.0;

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![
        Statement::Print(Expression::Literal(Literal::Number(value1))),
        Statement::Print(Expression::Literal(Literal::Number(value2))),
        Statement::Print(Expression::Variable(identifier_token)),
        Statement::Print(Expression::Literal(Literal::Number(value3))),
    ]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert!(matches!(result, Err(EvaluationError { .. })));
    assert_eq!(output, vec![Value::Number(value1), Value::Number(value2)]);
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// block statement that contains correct
// statements and that statements share
// the same local environment.
fn test_share_environment() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // The value we are going to use.
    let var_value = 42.0;

    // A token for the printed variable name.
    let identifier_token = Token {
        ttype: TokenType::Identifier,
        lexeme: var_name.clone(),
        literal: TokenLiteral::Null,
        line: 1,
    };

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![
        Statement::Var(var_name, Expression::Literal(Literal::Number(var_value))),
        Statement::Print(Expression::Variable(identifier_token)),
    ]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert_eq!(output, vec![Value::Number(var_value)]);
}

#[test]
// Test that the function `execute` works
// correctly when the statement is a
// block statement that contains correct
// statements and that defining a variable
// in the local environment does not affect the
// parent one.
fn test_define_variable_is_local() {
    // The parent environment.
    let mut env = Environment::new();

    // The name of the variable
    // we are going to use.
    let var_name = String::from("somevar");

    // The value we are going to use.
    let var_value = 42.0;

    // The statement we are going to process
    // contains a block.
    let statement = Statement::Block(vec![Statement::Var(
        var_name.clone(),
        Expression::Literal(Literal::Number(var_value)),
    )]);

    // Execute the statement.
    let (result, output) = run_execute(&statement, &mut env);

    assert_eq!(result, Ok(()));
    assert!(output.is_empty());
    assert!(matches!(env.get(&var_name), Err(EnvironmentError { .. })));
}
