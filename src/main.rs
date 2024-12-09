use std::env;
use std::fs;
use std::panic;
use std::process;

use evaluator::Evaluator;
use parser::Parser;
use scanner::Scanner;

mod evaluator;
mod formatter;
mod parser;
mod report;
mod scanner;

fn main() {
    setup_panic_hook();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <filename>", args[0]);
        eprintln!("Commands: tokenize, parse, evaluate");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_default();

    match command.as_str() {
        "tokenize" => tokenize(file_contents),
        "parse" => parse(file_contents),
        "evaluate" => evaluate(file_contents),
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}

fn tokenize(contents: String) {
    let mut scanner = Scanner::new();
    let tokens = scanner.run(contents);

    for token in &tokens {
        println!("{}", token);
    }

    process::exit(scanner.exit_code());
}

fn parse(contents: String) {
    let mut scanner = Scanner::new();
    let tokens = scanner.run(contents);

    if scanner.had_error() {
        process::exit(scanner.exit_code());
    }

    let mut parser = Parser::new();
    if let Some(ast) = parser.run(tokens) {
        println!("{}", ast);
    }

    process::exit(parser.exit_code());
}

fn evaluate(contents: String) {
    let mut scanner = Scanner::new();
    let tokens = scanner.run(contents);
    if scanner.had_error() {
        process::exit(scanner.exit_code());
    }

    let mut parser = Parser::new();
    if let Some(ast) = parser.run(tokens) {
        let evaluator = Evaluator::new();
        let result = evaluator.evaluate(&ast);
        println!("{}", result);
    }

    process::exit(parser.exit_code());
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

fn setup_panic_hook() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        default_hook(panic_info);
        process::exit(70);
    }));
}
