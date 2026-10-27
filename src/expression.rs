use std::fmt;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => f.write_str("nil"),
            Self::Number(n) if n.fract() == 0.0 => write!(f, "{n:.1}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => f.write_str(s),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary {
        left: Box<Self>,
        operator: Token,
        right: Box<Self>,
    },
    Group(Box<Self>),
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Self>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: String = match self {
            Self::Binary {
                left,
                operator,
                right,
            } => format!("({} {} {})", operator.lexeme, left, right),
            Self::Group(p) => format!("(group {})", p),
            Self::Literal(s) => s.to_string(),
            Self::Unary { operator, right } => format!("({} {})", operator.lexeme, right),
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that integer numbers are displayed
    // correctly with a decimal part of `.0`.
    fn test_literal_display_integer_number() {
        assert_eq!(Literal::Number(42.0).to_string(), "42.0");
    }

    #[test]
    // Test that float numbers are displayed
    // correctly with a decimal part.
    fn test_literal_display_decimal_number() {
        assert_eq!(Literal::Number(42.5).to_string(), "42.5");
    }
}
