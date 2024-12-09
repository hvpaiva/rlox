use core::panic;
use std::{cmp::Ordering, fmt::Display};

use crate::parser::{Binary, Expr, Literal, Operator, Unary};

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Binary(binary) => self.evaluate_binary(*binary),
            Expr::Unary(unary) => self.evaluate_unary(*unary),
            Expr::Literal(literal) => self.evaluate_literal(literal),
            Expr::Grouping(grouping) => self.evaluate(*grouping),
        }
    }

    fn evaluate_binary(&mut self, binary: Binary) -> Value {
        let Binary {
            left,
            operator,
            right,
        } = binary;
        let left = self.evaluate(*left);
        let right = self.evaluate(*right);
        match operator {
            Operator::BangEqual => Value::Boolean(left != right),
            Operator::EqualEqual => Value::Boolean(left == right),
            Operator::Greater => Value::Boolean(left > right),
            Operator::GreaterEqual => Value::Boolean(left >= right),
            Operator::Less => Value::Boolean(left < right),
            Operator::LessEqual => Value::Boolean(left <= right),
            Operator::Minus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                _ => panic!("Operands must be numbers"),
            },
            Operator::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                (Value::String(l), Value::String(r)) => Value::String(l + &r),
                _ => panic!("Operands must be numbers or strings"),
            },
            Operator::Slash => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                _ => panic!("Operands must be numbers"),
            },
            Operator::Star => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                _ => panic!("Operands must be numbers"),
            },
            _ => panic!("Invalid binary operator"),
        }
    }

    fn evaluate_unary(&mut self, unary: Unary) -> Value {
        let Unary { operator, right } = unary;
        let value = self.evaluate(*right);
        match operator {
            Operator::Bang => Value::Boolean(!self.is_truthy(value)),
            Operator::Minus => match value {
                Value::Number(n) => Value::Number(-n),
                _ => panic!("Operand must be a number"),
            },
            _ => panic!("Invalid unary operator"),
        }
    }

    fn evaluate_literal(&mut self, literal: Literal) -> Value {
        match literal {
            Literal::Boolean(b) => Value::Boolean(b),
            Literal::Number(n) => Value::Number(n),
            Literal::String(s) => Value::String(s),
            Literal::None => Value::Nil,
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Boolean(b) => b,
            Value::Nil => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
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
