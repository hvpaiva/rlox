use std::env;
use std::fmt::Display;
use std::fs;
use std::process::exit;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug)]
enum TokenType {
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
}

#[allow(dead_code)]
struct Token {
    ty: TokenType,
    lexer: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(lexer: char, ty: TokenType, line: usize) -> Self {
        Self {
            ty,
            lexer: lexer.to_string(),
            literal: None,
            line,
        }
    }

    fn parse_char(lexer: char, line: usize) -> Result<Self, String> {
        match lexer {
            '(' => Ok(Token::new(lexer, TokenType::LEFT_PAREN, line)),
            ')' => Ok(Token::new(lexer, TokenType::RIGHT_PAREN, line)),
            '{' => Ok(Token::new(lexer, TokenType::LEFT_BRACE, line)),
            '}' => Ok(Token::new(lexer, TokenType::RIGHT_BRACE, line)),
            '.' => Ok(Token::new(lexer, TokenType::DOT, line)),
            ',' => Ok(Token::new(lexer, TokenType::COMMA, line)),
            '*' => Ok(Token::new(lexer, TokenType::STAR, line)),
            '+' => Ok(Token::new(lexer, TokenType::PLUS, line)),
            '-' => Ok(Token::new(lexer, TokenType::MINUS, line)),
            ';' => Ok(Token::new(lexer, TokenType::SEMICOLON, line)),
            ch => Err(format!("[line {line}] Error: Unexpected character: {ch}")),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.ty,
            self.lexer,
            self.literal.clone().unwrap_or("null".to_owned())
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_default();

            scan(file_contents)
        }
        _ => eprintln!("Unknown command: {command}"),
    }
}

fn scan(contents: String) {
    let mut result = 0;
    for (i, line) in contents.lines().enumerate() {
        line.chars()
            .map(|s| Token::parse_char(s, i + 1))
            .for_each(|t| {
                if let Ok(token) = t {
                    println!("{}", token);
                } else if let Err(e) = t {
                    eprintln!("{}", e);
                    result = 65;
                }
            });
    }
    println!("EOF  null");
    exit(result);
}
