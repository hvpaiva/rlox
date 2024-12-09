use crate::evaluator::{Evaluator, Value};
use crate::parser::{Binary, Operator};
use std::panic;

#[derive(Debug)]
pub struct BinaryEvaluator;

impl BinaryEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, binary: &Binary) -> Value {
        let left = Evaluator::evaluate_static(&binary.left);
        let right = Evaluator::evaluate_static(&binary.right);

        match binary.operator {
            Operator::BangEqual => Value::Boolean(left != right),
            Operator::EqualEqual => Value::Boolean(left == right),
            Operator::Greater => self.compare(left, right, |l, r| l > r),
            Operator::GreaterEqual => self.compare(left, right, |l, r| l >= r),
            Operator::Less => self.compare(left, right, |l, r| l < r),
            Operator::LessEqual => self.compare(left, right, |l, r| l <= r),
            Operator::Minus => self.arithmetic(left, right, |l, r| l - r),
            Operator::Plus => self.addition(left, right),
            Operator::Slash => self.arithmetic(left, right, |l, r| l / r),
            Operator::Star => self.arithmetic(left, right, |l, r| l * r),
            _ => panic!("Invalid binary operator"),
        }
    }

    fn compare<F>(&self, left: Value, right: Value, cmp: F) -> Value
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Value::Boolean(cmp(l, r)),
            _ => panic!("Operands must be numbers"),
        }
    }

    fn arithmetic<F>(&self, left: Value, right: Value, op: F) -> Value
    where
        F: Fn(f64, f64) -> f64,
    {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                if let Operator::Slash = self.current_operator() {
                    if r == 0.0 {
                        panic!("Operands must be numbers")
                    }
                }
                Value::Number(op(l, r))
            }
            _ => panic!("Operands must be numbers"),
        }
    }

    fn addition(&self, left: Value, right: Value) -> Value {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            (Value::String(l), Value::String(r)) => Value::String(l + &r),
            _ => panic!("Operands must be numbers or strings"),
        }
    }

    fn current_operator(&self) -> Operator {
        Operator::Plus
    }
}

impl Evaluator {
    pub fn evaluate_static(expr: &crate::parser::Expr) -> Value {
        let evaluator = Evaluator::new();
        evaluator.evaluate(expr)
    }
}
