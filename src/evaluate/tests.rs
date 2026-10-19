use super::*;
use crate::test_utils::dummy_token;

mod binary_arithmetic;
mod binary_comparisons;
mod binary_strings;
mod literal_group;
mod unary;

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
