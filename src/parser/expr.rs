use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::formatter::format_number;
use crate::parser::Operator;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expr::Binary(binary) => {
                write!(f, "({} {} {})", binary.operator, binary.left, binary.right)
            }
            Expr::Unary(unary) => write!(f, "({} {})", unary.operator, unary.right),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Number(n) => write!(f, "{}", format_number(*n)),
            Literal::String(s) => write!(f, "{}", s),
            Literal::None => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Operator,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Operator,
    pub right: Box<Expr>,
}
