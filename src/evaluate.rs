use std::fmt;

use crate::expression::{Expression, Literal};
use crate::token::{Token, TokenType};

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
        Expression::Binary {
            left,
            operator,
            right,
        } => evaluate_binary(operator, left, right),
        Expression::Group(g) => evaluate(g),
        Expression::Literal(Literal::Boolean(b)) => Ok(Value::Boolean(*b)),
        Expression::Literal(Literal::Nil) => Ok(Value::Nil),
        Expression::Literal(Literal::Number(n)) => Ok(Value::Number(*n)),
        Expression::Literal(Literal::String(s)) => Ok(Value::String(s.clone())),
        Expression::Unary { operator, right } => evaluate_unary(operator, right),
    }
}

fn evaluate_unary(operator: &Token, right: &Expression) -> EvaluationResult<Value> {
    let value = evaluate(right)?;

    match operator.ttype {
        TokenType::Bang => match value {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            Value::Number(_) => Ok(Value::Boolean(false)),
            Value::Nil => Ok(Value::Boolean(true)),
            Value::String(_) => Ok(Value::Boolean(false)),
        },
        TokenType::Minus => match value {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(EvaluationError::new(1, "Operand must be a number.")),
        },
        _ => unreachable!("Invalid unary operator {:?}", operator.ttype),
    }
}

fn evaluate_binary(
    operator: &Token,
    left: &Expression,
    right: &Expression,
) -> EvaluationResult<Value> {
    let lvalue = evaluate(left)?;
    let rvalue = evaluate(right)?;

    match (&operator.ttype, lvalue, rvalue) {
        (TokenType::Minus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        (TokenType::Plus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (TokenType::Slash, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
        (TokenType::Star, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        _ => Err(EvaluationError::new(1, "Operands must be numbers.")),
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenLiteral;

    use super::*;

    fn dummy_token(ttype: TokenType) -> Token {
        Token {
            ttype,
            lexeme: String::new(),
            literal: TokenLiteral::Null,
            line: 1,
        }
    }

    // Build a unary expression with the
    // given operator and right operand.
    fn dummy_unary_expression(ttype: TokenType, right: Expression) -> Expression {
        Expression::Unary {
            operator: dummy_token(ttype),
            right: Box::new(right),
        }
    }

    // Build a binary expression with the
    // given operator and operands.
    fn dummy_binary_expression(
        left: Expression,
        ttype: TokenType,
        right: Expression,
    ) -> Expression {
        Expression::Binary {
            left: Box::new(left),
            operator: dummy_token(ttype),
            right: Box::new(right),
        }
    }

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

    #[test]
    // Test that a group evaluates
    // to the value of the expression
    // it contains.
    fn test_evaluate_group() {
        let expression = Expression::Group(Box::new(Expression::Literal(Literal::Number(42.0))));

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that a nested group evaluates
    // to the value of the innermost
    // expression.
    fn test_evaluate_group_nested() {
        let expression = Expression::Group(Box::new(Expression::Group(Box::new(
            Expression::Literal(Literal::Number(42.0)),
        ))));

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that `Bang` in front of
    // `true` returns `false`.
    fn test_evaluate_unary_bang_true() {
        let expression =
            dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Boolean(true)));

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
    }

    #[test]
    // Test that `Bang` in front of
    // `false` returns `true`.
    fn test_evaluate_unary_bang_false() {
        let expression = dummy_unary_expression(
            TokenType::Bang,
            Expression::Literal(Literal::Boolean(false)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
    }

    #[test]
    // Test that `Bang` in front of
    // a number returns `false`.
    fn test_evaluate_unary_bang_number() {
        let expression =
            dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Number(42.0)));

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
    }

    #[test]
    // Test that `Bang` in front of
    // the number zero returns `false`.
    fn test_evaluate_unary_bang_zero() {
        let expression =
            dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Number(0.0)));

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(false)));
    }

    #[test]
    // Test that `Bang` in front of
    // `nil` returns `true`.
    fn test_evaluate_unary_bang_nil() {
        let expression = dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Nil));

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
    }

    #[test]
    // Test that a double `Bang` in front
    // of a boolean returns the original boolean.
    fn test_evaluate_unary_double_bang_boolean() {
        let expression = dummy_unary_expression(
            TokenType::Bang,
            dummy_unary_expression(TokenType::Bang, Expression::Literal(Literal::Boolean(true))),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Boolean(true)));
    }

    #[test]
    // Test that `Minus` in front of
    // a number negates it.
    fn test_evaluate_unary_minus_number() {
        let expression =
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Number(42.0)));

        assert_eq!(evaluate(&expression), Ok(Value::Number(-42.0)));
    }

    #[test]
    // Test that a double `Minus` in front
    // of a number returns the original number.
    fn test_evaluate_unary_double_minus_number() {
        let expression = dummy_unary_expression(
            TokenType::Minus,
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Number(42.0))),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that `Minus` in front of
    // a group negates the value of the group.
    fn test_evaluate_unary_minus_group() {
        let expression = dummy_unary_expression(
            TokenType::Minus,
            Expression::Group(Box::new(Expression::Literal(Literal::Number(42.0)))),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(-42.0)));
    }

    #[test]
    // Test that `Minus` in front of
    // a boolean returns an error.
    fn test_evaluate_unary_minus_boolean() {
        let expression = dummy_unary_expression(
            TokenType::Minus,
            Expression::Literal(Literal::Boolean(true)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Minus` in front of
    // a string returns an error.
    fn test_evaluate_unary_minus_string() {
        let expression = dummy_unary_expression(
            TokenType::Minus,
            Expression::Literal(Literal::String(String::from("Some string"))),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Minus` in front of
    // `nil` returns an error.
    fn test_evaluate_unary_minus_nil() {
        let expression =
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil));

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that an error in the right
    // operand is returned by the
    // outer unary expression.
    fn test_evaluate_unary_propagates_error() {
        let expression = dummy_unary_expression(
            TokenType::Bang,
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Star` works correctly
    // with numbers that have no decimals.
    // A float with no decimal part is
    // the way we represent an integer.
    fn test_evaluate_binary_star_integer() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(6.0)),
            TokenType::Star,
            Expression::Literal(Literal::Number(7.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that `Star` works correctly
    // with float numbers.
    fn test_evaluate_binary_star_float() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(2.5)),
            TokenType::Star,
            Expression::Literal(Literal::Number(4.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(10.0)));
    }

    #[test]
    // Test that `Slash` between two
    // numbers divides them.
    fn test_evaluate_binary_slash() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Slash,
            Expression::Literal(Literal::Number(6.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(7.0)));
    }

    #[test]
    // Test that `Slash` returns a float
    // when the division is not exact.
    fn test_evaluate_binary_slash_float_result() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(7.0)),
            TokenType::Slash,
            Expression::Literal(Literal::Number(2.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(3.5)));
    }

    #[test]
    // Test that a chain of binary expressions
    // is evaluated following the structure
    // of the tree, e.g. (42 / 6) * 2.
    fn test_evaluate_binary_chain() {
        let expression = dummy_binary_expression(
            dummy_binary_expression(
                Expression::Literal(Literal::Number(42.0)),
                TokenType::Slash,
                Expression::Literal(Literal::Number(6.0)),
            ),
            TokenType::Star,
            Expression::Literal(Literal::Number(2.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(14.0)));
    }

    #[test]
    // Test that a group used as an operand
    // is evaluated before the binary
    // expression, e.g. 42 / (6 * 2).
    fn test_evaluate_binary_group_operand() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Slash,
            Expression::Group(Box::new(dummy_binary_expression(
                Expression::Literal(Literal::Number(6.0)),
                TokenType::Star,
                Expression::Literal(Literal::Number(2.0)),
            ))),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(3.5)));
    }

    #[test]
    // Test that a unary expression used
    // as an operand is evaluated before
    // the binary expression, e.g. -6 * 7.
    fn test_evaluate_binary_unary_operand() {
        let expression = dummy_binary_expression(
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Number(6.0))),
            TokenType::Star,
            Expression::Literal(Literal::Number(7.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(-42.0)));
    }

    #[test]
    // Test that an error in the left
    // operand is returned by the
    // binary expression.
    fn test_evaluate_binary_propagates_error_left() {
        let expression = dummy_binary_expression(
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
            TokenType::Star,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that an error in the right
    // operand is returned by the
    // binary expression.
    fn test_evaluate_binary_propagates_error_right() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Star,
            dummy_unary_expression(TokenType::Minus, Expression::Literal(Literal::Nil)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Star` with a boolean
    // operand returns an error.
    fn test_evaluate_binary_star_boolean() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Star,
            Expression::Literal(Literal::Boolean(true)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Star` with a string
    // operand returns an error.
    fn test_evaluate_binary_star_string() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::String(String::from("Some string"))),
            TokenType::Star,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Star` with a `nil`
    // operand returns an error.
    fn test_evaluate_binary_star_nil() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Nil),
            TokenType::Star,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Slash` with a boolean
    // operand returns an error.
    fn test_evaluate_binary_slash_boolean() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Slash,
            Expression::Literal(Literal::Boolean(true)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Slash` with a string
    // operand returns an error.
    fn test_evaluate_binary_slash_string() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::String(String::from("Some string"))),
            TokenType::Slash,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Slash` with a `nil`
    // operand returns an error.
    fn test_evaluate_binary_slash_nil() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Nil),
            TokenType::Slash,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Minus` works correctly
    // with numbers that have no decimals.
    fn test_evaluate_binary_minus_integer() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(50.0)),
            TokenType::Minus,
            Expression::Literal(Literal::Number(8.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that `Minus` works correctly
    // with float numbers.
    fn test_evaluate_binary_minus_float() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(5.5)),
            TokenType::Minus,
            Expression::Literal(Literal::Number(2.25)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(3.25)));
    }

    #[test]
    // Test that `Minus` returns a negative
    // number when the right operand is
    // greater than the left one.
    fn test_evaluate_binary_minus_negative_result() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(8.0)),
            TokenType::Minus,
            Expression::Literal(Literal::Number(50.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(-42.0)));
    }

    #[test]
    // Test that `Minus` with a boolean
    // operand returns an error.
    fn test_evaluate_binary_minus_boolean() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Minus,
            Expression::Literal(Literal::Boolean(true)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Minus` with a `nil`
    // operand returns an error.
    fn test_evaluate_binary_minus_nil() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Nil),
            TokenType::Minus,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Minus` cannot be
    // used between strings.
    fn test_evaluate_binary_minus_strings() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::String(String::from("StringA"))),
            TokenType::Minus,
            Expression::Literal(Literal::String(String::from("StringB"))),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Plus` works correctly
    // with numbers that have no decimals.
    fn test_evaluate_binary_plus_integer() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(34.0)),
            TokenType::Plus,
            Expression::Literal(Literal::Number(8.0)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(42.0)));
    }

    #[test]
    // Test that `Plus` works correctly
    // with float numbers.
    fn test_evaluate_binary_plus_float() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(1.25)),
            TokenType::Plus,
            Expression::Literal(Literal::Number(2.5)),
        );

        assert_eq!(evaluate(&expression), Ok(Value::Number(3.75)));
    }

    #[test]
    // Test that `Plus` with a boolean
    // operand returns an error.
    fn test_evaluate_binary_plus_boolean() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Number(42.0)),
            TokenType::Plus,
            Expression::Literal(Literal::Boolean(true)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }

    #[test]
    // Test that `Plus` with a `nil`
    // operand returns an error.
    fn test_evaluate_binary_plus_nil() {
        let expression = dummy_binary_expression(
            Expression::Literal(Literal::Nil),
            TokenType::Plus,
            Expression::Literal(Literal::Number(42.0)),
        );

        assert!(matches!(evaluate(&expression), Err(EvaluationError { .. })));
    }
}
