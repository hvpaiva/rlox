use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::scanner::Token;
use crate::scanner::TokenType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    EqualEqual,
    Equal,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl Operator {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token.ty {
            TokenType::EQUAL => Some(Self::Equal),
            TokenType::EQUAL_EQUAL => Some(Self::EqualEqual),
            TokenType::BANG => Some(Self::Bang),
            TokenType::BANG_EQUAL => Some(Self::BangEqual),
            TokenType::LESS => Some(Self::Less),
            TokenType::LESS_EQUAL => Some(Self::LessEqual),
            TokenType::GREATER => Some(Self::Greater),
            TokenType::GREATER_EQUAL => Some(Self::GreaterEqual),
            TokenType::PLUS => Some(Self::Plus),
            TokenType::MINUS => Some(Self::Minus),
            TokenType::STAR => Some(Self::Star),
            TokenType::SLASH => Some(Self::Slash),
            _ => None,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Bang => write!(f, "!"),
            Operator::BangEqual => write!(f, "!="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Star => write!(f, "*"),
            Operator::Slash => write!(f, "/"),
        }
    }
}
