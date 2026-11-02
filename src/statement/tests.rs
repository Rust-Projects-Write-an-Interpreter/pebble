use super::*;

use crate::environment::EnvironmentError;
use crate::evaluate::EvaluationError;
use crate::expression::Literal;
use crate::token::{Token, TokenLiteral, TokenType};

mod expression;
mod print;
mod var;

// A helper to simplify tests
// for the function `execute`.
fn run_execute(statement: &Statement, env: &mut Environment) -> (EvaluationResult<()>, Vec<Value>) {
    // A vector to host the output.
    let mut output = Vec::new();

    // Execute the statement.
    let result = execute(statement, env, &mut |v| output.push(v));

    (result, output)
}
