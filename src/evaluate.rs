use std::fmt;

use crate::environment::Environment;
use crate::expression::{Expression, Literal};
use crate::token::{Token, TokenType};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => f.write_str("nil"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => f.write_str(s),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EvaluationError {
    pub message: String,
    pub line: u16,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.line)
    }
}

impl EvaluationError {
    fn new(line: u16, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line,
        }
    }
}

pub type EvaluationResult<T> = Result<T, EvaluationError>;

pub fn evaluate(expression: &Expression, env: &mut Environment) -> EvaluationResult<Value> {
    match expression {
        Expression::Binary {
            left,
            operator,
            right,
        } => evaluate_binary(operator, left, right, env),
        Expression::Group(g) => evaluate(g, env),
        Expression::Literal(Literal::Boolean(b)) => Ok(Value::Boolean(*b)),
        Expression::Literal(Literal::Nil) => Ok(Value::Nil),
        Expression::Literal(Literal::Number(n)) => Ok(Value::Number(*n)),
        Expression::Literal(Literal::String(s)) => Ok(Value::String(s.clone())),
        Expression::Unary { operator, right } => evaluate_unary(operator, right, env),
        Expression::Variable(t) => env
            .get(&t.lexeme)
            .map_err(|e| EvaluationError::new(t.line, e.message)),
    }
}

fn evaluate_unary(
    operator: &Token,
    right: &Expression,
    env: &mut Environment,
) -> EvaluationResult<Value> {
    let value = evaluate(right, env)?;

    match operator.ttype {
        TokenType::Bang => match value {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            Value::Number(_) => Ok(Value::Boolean(false)),
            Value::Nil => Ok(Value::Boolean(true)),
            Value::String(_) => Ok(Value::Boolean(false)),
        },
        TokenType::Minus => match value {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(EvaluationError::new(
                operator.line,
                "Operand must be a number.",
            )),
        },
        _ => unreachable!("Invalid unary operator {:?}", operator.ttype),
    }
}

fn evaluate_binary(
    operator: &Token,
    left: &Expression,
    right: &Expression,
    env: &mut Environment,
) -> EvaluationResult<Value> {
    let lvalue = evaluate(left, env)?;
    let rvalue = evaluate(right, env)?;

    match (&operator.ttype, lvalue, rvalue) {
        // Arithmetic
        (TokenType::Minus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        (TokenType::Plus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (TokenType::Slash, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
        (TokenType::Star, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        // Comparisons
        (TokenType::Greater, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
        (TokenType::GreaterEqual, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
        (TokenType::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
        (TokenType::LessEqual, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
        // Equality
        (TokenType::BangEqual, l, r) => Ok(Value::Boolean(l != r)),
        (TokenType::EqualEqual, l, r) => Ok(Value::Boolean(l == r)),
        // Strings
        (TokenType::Plus, Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
        (TokenType::Plus, _, _) => Err(EvaluationError::new(
            operator.line,
            "Operands must be two numbers or two strings.",
        )),
        _ => Err(EvaluationError::new(
            operator.line,
            "Operands must be numbers.",
        )),
    }
}
