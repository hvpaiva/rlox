use crate::evaluator::Value;
use crate::parser::Literal;

#[derive(Debug)]
pub struct LiteralEvaluator;

impl LiteralEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, literal: &Literal) -> Value {
        match literal {
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Number(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::None => Value::Nil,
        }
    }
}
