use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Expression),
}
