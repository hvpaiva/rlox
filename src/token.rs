// token.rs
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexer: String,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(ty: TokenType, lexer: &str) -> Self {
        Self {
            ty,
            lexer: lexer.to_string(),
            literal: None,
        }
    }

    pub fn new_with_literal(ty: TokenType, literal: String) -> Self {
        if ty == TokenType::NUMBER {
            Self {
                ty,
                lexer: literal.clone(),
                literal: Some(literal),
            }
        } else {
            Self {
                ty,
                lexer: format!("\"{literal}\""),
                literal: Some(literal),
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.ty == TokenType::NUMBER {
            let parsed = self.literal.clone().unwrap();
            let formated = format_to_float_output(&parsed);

            write!(f, "{:?} {} {}", self.ty, self.lexer, formated)
        } else {
            write!(
                f,
                "{:?} {} {}",
                self.ty,
                self.lexer,
                self.literal.clone().unwrap_or("null".to_owned())
            )
        }
    }
}

fn format_to_float_output(input: &str) -> String {
    let parsed: f64 = input
        .parse()
        .expect("Invalid input: could not parse to float");
    if parsed.fract() == 0.0 {
        format!("{:.1}", parsed)
    } else {
        parsed
            .to_string()
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
