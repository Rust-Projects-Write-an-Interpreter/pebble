use super::*;
use crate::test_utils::dummy_token;

mod binary_arithmetic;
mod binary_comparisons;
mod binary_equality;
mod binary_strings;
mod literal_group;
mod unary;
mod variable;

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
fn dummy_binary_expression(left: Expression, ttype: TokenType, right: Expression) -> Expression {
    Expression::Binary {
        left: Box::new(left),
        operator: dummy_token(ttype),
        right: Box::new(right),
    }
}

// Build a variable expression with
// the given name on the given line.
fn dummy_variable_expression(name: &str, line: u16) -> Expression {
    Expression::Variable(Token {
        lexeme: String::from(name),
        line,
        ..dummy_token(TokenType::Identifier)
    })
}
