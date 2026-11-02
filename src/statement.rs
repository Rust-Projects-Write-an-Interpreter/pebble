use crate::environment::Environment;
use crate::evaluate::{evaluate, EvaluationResult, Value};
use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Var(String, Expression),
}

pub fn execute(
    statement: &Statement,
    env: &mut Environment,
    output: &mut impl FnMut(Value),
) -> EvaluationResult<()> {
    match statement {
        Statement::Expression(expression) => {
            evaluate(expression, env)?;
        }
        Statement::Print(expression) => output(evaluate(expression, env)?),
        Statement::Var(name, expression) => {
            let value = evaluate(expression, env)?;
            env.define(name.clone(), value);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::environment::EnvironmentError;
    use crate::evaluate::EvaluationError;
    use crate::expression::Literal;
    use crate::token::{Token, TokenLiteral, TokenType};

    // A helper to simplify tests
    // for the function `execute`.
    fn run_execute(
        statement: &Statement,
        env: &mut Environment,
    ) -> (EvaluationResult<()>, Vec<Value>) {
        // A vector to host the output.
        let mut output = Vec::new();

        // Execute the statement.
        let result = execute(statement, env, &mut |v| output.push(v));

        (result, output)
    }

    #[test]
    // Test that the function `execute` works
    // correctly when the statement is an
    // expression statement with a correct
    // expression: an assignment to an
    // existing variable.
    fn test_expression_correct() {
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
    fn test_expression_incorrect() {
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

    #[test]
    // Test that the function `execute` works
    // correctly when the statement is a
    // print statement with a correct
    // expression: the name of an existing
    // variable.
    fn test_print_correct() {
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
    fn test_print_incorrect() {
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

    #[test]
    // Test that the function `execute` works
    // correctly when the statement is a
    // variable definition statement with
    // a correct expression: a number.
    fn test_variable_definition_correct() {
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
    fn test_variable_definition_incorrect() {
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
}
