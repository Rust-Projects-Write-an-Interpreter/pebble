use std::fmt;

use crate::expression::{Expression, Literal};

#[derive(Debug, PartialEq)]
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

pub fn evaluate(expression: &Expression) -> EvaluationResult<Value> {
    match expression {
        Expression::Literal(Literal::Boolean(b)) => Ok(Value::Boolean(*b)),
        Expression::Literal(Literal::Nil) => Ok(Value::Nil),
        Expression::Literal(Literal::Number(n)) => Ok(Value::Number(*n)),
        Expression::Literal(Literal::String(s)) => Ok(Value::String(s.clone())),
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

    #[test]
    // Test that an integer number literal
    // evaluates to a number value.
    fn test_evaluate_literal_number_integer() {
        let expression = Expression::Literal(Literal::Number(42.0));

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that a float number literal
    // evaluates to a number value.
    fn test_evaluate_literal_number_float() {
        let expression = Expression::Literal(Literal::Number(42.5));

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.5)));
    }

    #[test]
    // Test that a string literal
    // evaluates to a string value.
    fn test_evaluate_literal_string() {
        let expression = Expression::Literal(Literal::String(String::from("Some string")));

        assert_eq!(
            evaluate(&expression),
            Ok(Value::String(String::from("Some string")))
        );
    }
}
