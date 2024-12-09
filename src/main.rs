use std::env;
use std::fs;
use std::panic;
use std::process;
use std::process::exit;
use std::sync::Arc;

use scanner::Scanner;

mod evaluator;
mod formatter;
mod parser;
mod report;
mod scanner;

fn main() {
    let default_hook = Arc::new(panic::take_hook());
    let hook = Arc::clone(&default_hook);
    panic::set_hook(Box::new(move |panic_info| {
        hook(panic_info);
        process::exit(70);
    }));

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
            let mut scanner = Scanner::new();
            let tokens = scanner.run(file_contents);

            if scanner.had_error() {
                exit(scanner.exit_code());
            }
            let mut parser = parser::Parser::new();

            if let Some(ast) = parser.run(tokens) {
                println!("{}", ast);
            }

            exit(parser.exit_code());
        }
        "evaluate" => {
            let mut scanner = Scanner::new();
            let tokens = scanner.run(file_contents);
            if scanner.had_error() {
                exit(scanner.exit_code());
            }

            let mut parser = parser::Parser::new();
            let mut evaluator = evaluator::Evaluator::new();
            if let Some(ast) = parser.run(tokens) {
                let result = evaluator.evaluate(ast);

                println!("{}", result);
            }

            if parser.had_error() {
                exit(parser.exit_code());
            }
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
