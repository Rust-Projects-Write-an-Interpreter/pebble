use std::fmt;

use crate::expression::{Expression, Literal};

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => f.write_str("nil"),
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

pub fn evaluate(expression: &Expression) -> EvaluationResult<Value> {
    match expression {
        Expression::Literal(Literal::Boolean(b)) => Ok(Value::Boolean(*b)),
        Expression::Literal(Literal::Nil) => Ok(Value::Nil),
        _ => Err(EvaluationError::new(1, expression.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that the literal `true`
    // evaluates to a boolean value.
    fn test_evaluate_literal_true() {
        let expression = Expression::Literal(Literal::Boolean(true));

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
    }

    #[test]
    // Test that the literal `false`
    // evaluates to a boolean value.
    fn test_evaluate_literal_false() {
        let expression = Expression::Literal(Literal::Boolean(false));

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
    }

    #[test]
    // Test that the literal `nil`
    // evaluates to the value `Nil`.
    fn test_evaluate_literal_nil() {
        let expression = Expression::Literal(Literal::Nil);

        assert_eq!(evaluate(&expression), Ok(Value::Nil));
    }
}
