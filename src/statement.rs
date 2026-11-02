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
            let mut local_env = Environment::new();

            for statement in v.iter() {
                execute(statement, &mut local_env, output)?;
            }
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
