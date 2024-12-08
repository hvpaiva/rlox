// token.rs
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::keyword::Keyword;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    DOT,
    STAR,
    PLUS,
    MINUS,
    COMMA,
    SEMICOLON,
    EQUAL,
    EQUAL_EQUAL,
    BANG,
    BANG_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    SLASH,
    STRING(String),
    NUMBER(String),
    IDENTIFIER(String),
    KEYWORD(Keyword),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexer: String,
    pub literal: Literal,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    None,
}

impl Token {
    pub fn new(ty: TokenType) -> Self {
        Self {
            ty: ty.clone(),
            lexer: ty.to_lexer(),
            literal: ty.to_literal(),
        }
    }
}

impl TokenType {
    pub fn to_lexer(&self) -> String {
        match self {
            TokenType::LEFT_PAREN => "(".to_string(),
            TokenType::RIGHT_PAREN => ")".to_string(),
            TokenType::LEFT_BRACE => "{".to_string(),
            TokenType::RIGHT_BRACE => "}".to_string(),
            TokenType::DOT => ".".to_string(),
            TokenType::STAR => "*".to_string(),
            TokenType::PLUS => "+".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::COMMA => ",".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::EQUAL => "=".to_string(),
            TokenType::EQUAL_EQUAL => "==".to_string(),
            TokenType::BANG => "!".to_string(),
            TokenType::BANG_EQUAL => "!=".to_string(),
            TokenType::LESS => "<".to_string(),
            TokenType::LESS_EQUAL => "<=".to_string(),
            TokenType::GREATER => ">".to_string(),
            TokenType::GREATER_EQUAL => ">=".to_string(),
            TokenType::SLASH => "/".to_string(),
            TokenType::STRING(lexer) => lexer.to_string(),
            TokenType::NUMBER(lexer) => lexer.to_string(),
            TokenType::IDENTIFIER(lexer) => lexer.to_string(),
            TokenType::KEYWORD(keyword) => keyword.to_raw_string(),
            TokenType::EOF => "".to_string(),
        }
    }

    pub fn to_literal(&self) -> Literal {
        match self {
            TokenType::STRING(lexer) => Literal::String(lexer.trim_matches('\"').to_string()),
            TokenType::NUMBER(lexer) => Literal::Number(lexer.parse().unwrap()),
            _ => Literal::None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {} {}", self.ty, self.lexer, self.literal)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TokenType::STRING(_) => write!(f, "STRING"),
            TokenType::NUMBER(_) => write!(f, "NUMBER"),
            TokenType::IDENTIFIER(_) => write!(f, "IDENTIFIER"),
            TokenType::KEYWORD(_) => write!(f, "KEYWORD"),
            ty => write!(f, "{:?}", ty),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", format_number(n)),
            Literal::None => write!(f, "null"),
        }
    }
}

fn format_number(value: &f64) -> String {
    if *value == value.trunc() {
        format!("{:.1}", value)
    } else {
        format!("{}", value)
    }
}
