use std::env;
use std::fs;
use std::process::exit;

use scanner::Scanner;

mod formatter;
mod parser;
mod report;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_default();

    match command.as_str() {
        "tokenize" => {
            let mut scanner = Scanner::new();
            let tokens = scanner.run(file_contents);

            for token in &tokens {
                println!("{}", token);
            }

            exit(scanner.exit_code());
        }
        "parse" => {
            let mut parser = parser::Parser::new();
            let mut scanner = Scanner::new();

            let tokens = scanner.run(file_contents);

            if let Some(ast) = parser.run(tokens) {
                println!("{}", ast);
            }

            exit(parser.exit_code());
        }
        _ => eprintln!("Unknown command: {command}"),
    }
}

pub trait Process {
    type Input;
    type Output;

    fn run(&mut self, input: Self::Input) -> Self::Output;

    fn had_error(&self) -> bool;

    fn exit_code(&self) -> i32 {
        if self.had_error() {
            65
        } else {
            0
        }
    }
}
