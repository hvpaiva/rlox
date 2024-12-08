use std::fmt::{Display, Formatter, Result as FmtResult};

use super::keyword::Keyword;

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
    STRING,
    NUMBER,
    IDENTIFIER,
    KEYWORD(Keyword),
    EOF,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    None,
}

impl Token {
    pub fn new_with_lexeme(ty: &TokenType, lexeme: String, line: usize) -> Self {
        Self {
            ty: ty.clone(),
            lexeme: lexeme.clone(),
            literal: Literal::from_lexeme(ty, lexeme),
            line,
        }
    }

    pub fn new(ty: &TokenType, line: usize) -> Self {
        Self::new_with_lexeme(ty, ty.to_lexeme().to_string(), line)
    }
}

impl TokenType {
    pub fn to_lexeme(&self) -> &'static str {
        match self {
            TokenType::LEFT_PAREN => "(",
            TokenType::RIGHT_PAREN => ")",
            TokenType::LEFT_BRACE => "{",
            TokenType::RIGHT_BRACE => "}",
            TokenType::DOT => ".",
            TokenType::STAR => "*",
            TokenType::PLUS => "+",
            TokenType::MINUS => "-",
            TokenType::COMMA => ",",
            TokenType::SEMICOLON => ";",
            TokenType::EQUAL => "=",
            TokenType::EQUAL_EQUAL => "==",
            TokenType::BANG => "!",
            TokenType::BANG_EQUAL => "!=",
            TokenType::LESS => "<",
            TokenType::LESS_EQUAL => "<=",
            TokenType::GREATER => ">",
            TokenType::GREATER_EQUAL => ">=",
            TokenType::SLASH => "/",
            TokenType::KEYWORD(k) => k.as_str(),
            TokenType::STRING => "<string>",
            TokenType::NUMBER => "<number>",
            TokenType::IDENTIFIER => "<identifier>",
            TokenType::EOF => "",
        }
    }
}

impl Literal {
    pub fn from_lexeme(ty: &TokenType, lexeme: String) -> Self {
        match ty {
            TokenType::STRING => Literal::String(lexeme.trim_matches('"').to_string()),
            TokenType::NUMBER => Literal::Number(lexeme.parse().unwrap_or_default()),
            TokenType::IDENTIFIER => Literal::None,
            _ => Literal::None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {} {}", self.ty, self.lexeme, self.literal)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TokenType::KEYWORD(key) => write!(f, "{:?}", key),
            ty => write!(f, "{:?}", ty),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", format_number(*n)),
            Literal::None => write!(f, "null"),
        }
    }
}

fn format_number(value: f64) -> String {
    if value == value.trunc() {
        format!("{:.1}", value)
    } else {
        format!("{}", value)
    }
}
