pub mod binary;
pub mod literals;
pub mod unary;

use std::cmp::Ordering;
use std::fmt::Display;

use crate::parser::Expr;
use binary::BinaryEvaluator;
use literals::LiteralEvaluator;
use unary::UnaryEvaluator;

#[derive(Debug)]
pub struct Evaluator {
    binary_evaluator: BinaryEvaluator,
    unary_evaluator: UnaryEvaluator,
    literal_evaluator: LiteralEvaluator,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            binary_evaluator: BinaryEvaluator::new(),
            unary_evaluator: UnaryEvaluator::new(),
            literal_evaluator: LiteralEvaluator::new(),
        }
    }

    pub fn evaluate(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Binary(binary) => self.binary_evaluator.evaluate(binary),
            Expr::Unary(unary) => self.unary_evaluator.evaluate(unary),
            Expr::Literal(literal) => self.literal_evaluator.evaluate(literal),
            Expr::Grouping(grouping) => self.evaluate(grouping),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l == r,
            (Self::String(l), Self::String(r)) => l == r,
            (Self::Boolean(l), Self::Boolean(r)) => l == r,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;
        match (self, other) {
            (Number(l), Number(r)) => l.partial_cmp(r).unwrap(),
            (Number(_), _) => Ordering::Less,
            (_, Number(_)) => Ordering::Greater,
            (String(l), String(r)) => l.cmp(r),
            (String(_), _) => Ordering::Less,
            (_, String(_)) => Ordering::Greater,
            (Boolean(l), Boolean(r)) => l.cmp(r),
            (Boolean(_), _) => Ordering::Less,
            (_, Boolean(_)) => Ordering::Greater,
            (Nil, Nil) => Ordering::Equal,
        }
    }
}
