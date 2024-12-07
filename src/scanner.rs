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
                '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, "(")),
                ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, ")")),
                '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, "{")),
                '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, "}")),
                '.' => tokens.push(Token::new(TokenType::DOT, ".")),
                ',' => tokens.push(Token::new(TokenType::COMMA, ",")),
                '*' => tokens.push(Token::new(TokenType::STAR, "*")),
                '+' => tokens.push(Token::new(TokenType::PLUS, "+")),
                '-' => tokens.push(Token::new(TokenType::MINUS, "-")),
                ';' => tokens.push(Token::new(TokenType::SEMICOLON, ";")),

                '=' => {
                    if self.match_next('=') {
                        tokens.push(Token::new(TokenType::EQUAL_EQUAL, "=="));
                    } else {
                        tokens.push(Token::new(TokenType::EQUAL, "="));
                    }
                }

                '!' => {
                    if self.match_next('=') {
                        tokens.push(Token::new(TokenType::BANG_EQUAL, "!="));
                    } else {
                        tokens.push(Token::new(TokenType::BANG, "!"));
                    }
                }

                '<' => {
                    if self.match_next('=') {
                        tokens.push(Token::new(TokenType::LESS_EQUAL, "<="));
                    } else {
                        tokens.push(Token::new(TokenType::LESS, "<"));
                    }
                }

                '>' => {
                    if self.match_next('=') {
                        tokens.push(Token::new(TokenType::GREATER_EQUAL, ">="));
                    } else {
                        tokens.push(Token::new(TokenType::GREATER, ">"));
                    }
                }

                '/' => {
                    if self.match_next('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        tokens.push(Token::new(TokenType::SLASH, "/"));
                    }
                }

                '"' => {
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
                        continue;
                    }

                    self.advance();
                    let literal = self.source[start..self.current - 1]
                        .iter()
                        .collect::<String>();

                    tokens.push(Token::new_with_literal(TokenType::STRING, literal));
                }

                c if c.is_ascii_digit() => {
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

                    tokens.push(Token::new_with_literal(TokenType::NUMBER, literal));
                }

                '\n' => {
                    self.line += 1;
                }

                ' ' | '\r' | '\t' => {
                    // ignorar espaços em branco
                }

                ch => {
                    print_err(self.line, &format!("Unexpected character: {}", ch));
                    self.had_error = true;
                }
            }
        }

        tokens.push(Token::new(TokenType::EOF, ""));
        tokens
    }
}

fn print_err(line: usize, message: &str) {
    eprintln!("[line {}] Error: {}", line, message);
}
