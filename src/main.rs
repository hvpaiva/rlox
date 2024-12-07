use std::env;
use std::fmt::Display;
use std::fs;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
}

struct Token {
    ty: TokenType,
    lexer: String,
    literal: Option<String>,
}

impl Token {
    fn parse_char(lexer: char) -> Option<Self> {
        match lexer {
            '(' => Some(Token {
                ty: TokenType::LEFT_PAREN,
                lexer: lexer.to_string(),
                literal: None,
            }),
            ')' => Some(Token {
                ty: TokenType::RIGHT_PAREN,
                lexer: lexer.to_string(),
                literal: None,
            }),
            '{' => Some(Token {
                ty: TokenType::LEFT_BRACE,
                lexer: lexer.to_string(),
                literal: None,
            }),
            '}' => Some(Token {
                ty: TokenType::RIGHT_BRACE,
                lexer: lexer.to_string(),
                literal: None,
            }),
            _ => None,
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
    contents.chars().map(Token::parse_char).for_each(|t| {
        if let Some(t) = t {
            println!("{t}");
        };
    });
    println!("EOF  null");
}
