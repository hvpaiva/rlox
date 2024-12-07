use crate::token::{Token, TokenType};

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
                '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, "(", self.line)),
                ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, ")", self.line)),
                '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, "{", self.line)),
                '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, "}", self.line)),
                '.' => tokens.push(Token::new(TokenType::DOT, ".", self.line)),
                ',' => tokens.push(Token::new(TokenType::COMMA, ",", self.line)),
                '*' => tokens.push(Token::new(TokenType::STAR, "*", self.line)),
                '+' => tokens.push(Token::new(TokenType::PLUS, "+", self.line)),
                '-' => tokens.push(Token::new(TokenType::MINUS, "-", self.line)),
                ';' => tokens.push(Token::new(TokenType::SEMICOLON, ";", self.line)),

                '=' => {
                    if self.match_next('=') {
                        tokens.push(Token::new(TokenType::EQUAL_EQUAL, "==", self.line));
                    } else {
                        tokens.push(Token::new(TokenType::EQUAL, "=", self.line));
                    }
                }

                '!' => {
                    if self.match_next('=') {
                        tokens.push(Token::new(TokenType::BANG_EQUAL, "!=", self.line));
                    } else {
                        tokens.push(Token::new(TokenType::BANG, "!", self.line));
                    }
                }

                '\n' => {
                    self.line += 1;
                }

                ' ' | '\r' | '\t' => {
                    // ignorar espaços em branco
                }

                ch => {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, ch);
                    self.had_error = true;
                }
            }
        }

        tokens.push(Token::new(TokenType::EOF, "", self.line));
        tokens
    }
}
