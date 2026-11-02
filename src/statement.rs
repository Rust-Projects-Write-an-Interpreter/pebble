use crate::environment::Environment;
use crate::evaluate::{evaluate, EvaluationResult, Value};
use crate::expression::Expression;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(Vec<Self>),
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
        Statement::Block(v) => {
            // Create a local environment for this
            // block. Assign the given environment
            // to it as parent.
            let mut local_env = Environment::new_with_parent(std::mem::take(env));

            // Avoid returning early with `?`. The
            // parent environment has to be put
            // back even if a statement fails.
            let result = v
                .iter()
                .try_for_each(|statement| execute(statement, &mut local_env, output));

            // Restore the parent environment.
            *env = local_env
                .restore_parent()
                .expect("Parent environment not present.");

            result?;
        }
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
