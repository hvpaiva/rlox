use std::ops::ControlFlow;

use crate::{
    keyword::Keyword,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
    had_error: bool,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let c = self.advance();
            match c {
                '(' => tokens.push(Token::new(TokenType::LEFT_PAREN)),
                ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN)),
                '{' => tokens.push(Token::new(TokenType::LEFT_BRACE)),
                '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE)),
                '.' => tokens.push(Token::new(TokenType::DOT)),
                ',' => tokens.push(Token::new(TokenType::COMMA)),
                '*' => tokens.push(Token::new(TokenType::STAR)),
                '+' => tokens.push(Token::new(TokenType::PLUS)),
                '-' => tokens.push(Token::new(TokenType::MINUS)),
                ';' => tokens.push(Token::new(TokenType::SEMICOLON)),
                '=' => self.parse_eq(&mut tokens),
                '!' => self.parse_bang(&mut tokens),
                '<' => self.parse_lesser(&mut tokens),
                '>' => self.parse_greater(&mut tokens),
                '/' => self.parse_slash(&mut tokens),
                '"' => {
                    if let ControlFlow::Break(_) = self.parse_string(&mut tokens) {
                        continue;
                    }
                }
                c if c.is_ascii_digit() => self.parse_number(&mut tokens),
                c if c.is_ascii_alphabetic() || c == '_' => self.parse_identifier(&mut tokens),
                '\n' => self.line += 1,
                ' ' | '\r' | '\t' => {}
                ch => self.not_parser(ch),
            }
        }

        tokens.push(Token::new(TokenType::EOF));
        tokens
    }

    fn not_parser(&mut self, ch: char) {
        print_err(self.line, &format!("Unexpected character: {}", ch));
        self.had_error = true;
    }

    fn parse_string(&mut self, tokens: &mut Vec<Token>) -> ControlFlow<()> {
        let start = self.current;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            print_err(self.line, "Unterminated string.");
            self.had_error = true;
            return ControlFlow::Break(());
        }
        self.advance();
        let literal = self.source[start - 1..self.current]
            .iter()
            .collect::<String>();
        tokens.push(Token::new(TokenType::STRING(literal)));

        ControlFlow::Continue(())
    }

    fn parse_number(&mut self, tokens: &mut Vec<Token>) {
        let start = self.current - 1;
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let literal = self.source[start..self.current].iter().collect::<String>();

        tokens.push(Token::new(TokenType::NUMBER(literal)));
    }

    fn parse_lesser(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            tokens.push(Token::new(TokenType::LESS_EQUAL));
        } else {
            tokens.push(Token::new(TokenType::LESS));
        }
    }

    fn parse_greater(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            tokens.push(Token::new(TokenType::GREATER_EQUAL));
        } else {
            tokens.push(Token::new(TokenType::GREATER));
        }
    }

    fn parse_slash(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
        } else {
            tokens.push(Token::new(TokenType::SLASH));
        }
    }

    fn parse_bang(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            tokens.push(Token::new(TokenType::BANG_EQUAL));
        } else {
            tokens.push(Token::new(TokenType::BANG));
        }
    }

    fn parse_eq(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            tokens.push(Token::new(TokenType::EQUAL_EQUAL));
        } else {
            tokens.push(Token::new(TokenType::EQUAL));
        }
    }

    fn parse_identifier(&mut self, tokens: &mut Vec<Token>) {
        let start = self.current;
        while self.peek().is_ascii_alphabetic()
            || self.peek().is_ascii_digit()
            || self.peek() == '_'
        {
            self.advance();
        }

        let lexeme = self.source[start - 1..self.current]
            .iter()
            .collect::<String>();

        if let Some(keyword) = Keyword::from(&lexeme) {
            tokens.push(Token::new(TokenType::KEYWORD(keyword)));
        } else {
            tokens.push(Token::new(TokenType::IDENTIFIER(lexeme)));
        };
    }
}

fn print_err(line: usize, message: &str) {
    eprintln!("[line {}] Error: {}", line, message);
}
