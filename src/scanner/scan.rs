use crate::{report::Reporter, Process};

use super::{
    keyword::Keyword,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
    reporter: Reporter,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new() -> Self {
        Self {
            source: Vec::new(),
            current: 0,
            line: 1,
            reporter: Reporter::new(),
        }
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

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let c = self.advance();
            match c {
                '(' => self.push_simple_token(&mut tokens, TokenType::LEFT_PAREN),
                ')' => self.push_simple_token(&mut tokens, TokenType::RIGHT_PAREN),
                '{' => self.push_simple_token(&mut tokens, TokenType::LEFT_BRACE),
                '}' => self.push_simple_token(&mut tokens, TokenType::RIGHT_BRACE),
                '.' => self.push_simple_token(&mut tokens, TokenType::DOT),
                ',' => self.push_simple_token(&mut tokens, TokenType::COMMA),
                '*' => self.push_simple_token(&mut tokens, TokenType::STAR),
                '+' => self.push_simple_token(&mut tokens, TokenType::PLUS),
                '-' => self.push_simple_token(&mut tokens, TokenType::MINUS),
                ';' => self.push_simple_token(&mut tokens, TokenType::SEMICOLON),
                '=' => self.scan_eq(&mut tokens),
                '!' => self.scan_bang(&mut tokens),
                '<' => self.scan_lesser(&mut tokens),
                '>' => self.scan_greater(&mut tokens),
                '/' => self.scan_slash(&mut tokens),
                '"' => self.scan_string(&mut tokens),
                c if c.is_ascii_digit() => self.scan_number(&mut tokens),
                c if c.is_ascii_alphabetic() || c == '_' => self.scan_identifier(&mut tokens),
                '\n' => self.line += 1,
                ' ' | '\r' | '\t' => {}
                ch => self.report_error(&format!("Unexpected character: {}", ch)),
            }
        }

        tokens.push(Token::new(&TokenType::EOF, self.line));
        tokens
    }

    fn push_simple_token(&mut self, tokens: &mut Vec<Token>, ty: TokenType) {
        tokens.push(Token::new(&ty, self.line));
    }

    fn push_with_lexeme(&mut self, tokens: &mut Vec<Token>, ty: TokenType, lexeme: String) {
        tokens.push(Token::new_with_lexeme(&ty, lexeme, self.line));
    }

    fn scan_string(&mut self, tokens: &mut Vec<Token>) {
        let start = self.current;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.report_error("Unterminated string.");
            return;
        }
        self.advance();
        let lexeme = self.source[start - 1..self.current]
            .iter()
            .collect::<String>();

        self.push_with_lexeme(tokens, TokenType::STRING, lexeme);
    }

    fn scan_number(&mut self, tokens: &mut Vec<Token>) {
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

        let lexeme = self.source[start..self.current].iter().collect::<String>();

        self.push_with_lexeme(tokens, TokenType::NUMBER, lexeme);
    }

    fn scan_lesser(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            self.push_simple_token(tokens, TokenType::LESS_EQUAL);
        } else {
            self.push_simple_token(tokens, TokenType::LESS);
        }
    }

    fn scan_greater(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            self.push_simple_token(tokens, TokenType::GREATER_EQUAL);
        } else {
            self.push_simple_token(tokens, TokenType::GREATER);
        }
    }

    fn scan_slash(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
        } else {
            self.push_simple_token(tokens, TokenType::SLASH);
        }
    }

    fn scan_bang(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            self.push_simple_token(tokens, TokenType::BANG_EQUAL);
        } else {
            self.push_simple_token(tokens, TokenType::BANG);
        }
    }

    fn scan_eq(&mut self, tokens: &mut Vec<Token>) {
        if self.match_next('=') {
            self.push_simple_token(tokens, TokenType::EQUAL_EQUAL);
        } else {
            self.push_simple_token(tokens, TokenType::EQUAL);
        }
    }

    fn scan_identifier(&mut self, tokens: &mut Vec<Token>) {
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
            self.push_simple_token(tokens, TokenType::KEYWORD(keyword));
        } else {
            self.push_with_lexeme(tokens, TokenType::IDENTIFIER, lexeme);
        };
    }

    fn report_error(&mut self, message: &str) {
        self.reporter.report(self.line, message.to_string());
    }
}

impl Process for Scanner {
    type Input = String;
    type Output = Vec<Token>;

    fn run(&mut self, input: Self::Input) -> Self::Output {
        self.source = input.chars().collect();
        let tokens = self.scan_tokens();
        self.reporter.print();
        tokens
    }

    fn had_error(&self) -> bool {
        self.reporter.had_error()
    }
}
