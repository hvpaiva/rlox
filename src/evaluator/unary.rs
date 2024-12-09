// src/evaluator/unary.rs

use crate::evaluator::{Evaluator, Value};
use crate::parser::{Operator, Unary};
use std::panic;

#[derive(Debug)]
pub struct UnaryEvaluator;

impl UnaryEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, unary: &Unary) -> Value {
        let right = Evaluator::evaluate_static(&unary.right);

        match unary.operator {
            Operator::Bang => Value::Boolean(!Evaluator::is_truthy_static(&right)),
            Operator::Minus => match right {
                Value::Number(n) => Value::Number(-n),
                _ => panic!("Operand must be a number"),
            },
            _ => panic!("Invalid unary operator"),
        }
    }
}

impl Evaluator {
    pub fn is_truthy_static(value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }
}
