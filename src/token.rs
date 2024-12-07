// token.rs
use std::fmt::{Display, Formatter, Result as FmtResult};

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
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

    EOF,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    pub ty: TokenType,
    pub lexer: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexer: &str, line: usize) -> Self {
        Self {
            ty,
            lexer: lexer.to_string(),
            literal: None,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{:?} {} {}",
            self.ty,
            self.lexer,
            self.literal.clone().unwrap_or("null".to_owned())
        )
    }
}
