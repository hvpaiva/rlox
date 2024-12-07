use std::env;
use std::fs;
use std::process::exit;

use scanner::Scanner;

mod scanner;
mod token;

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

            let mut scanner = Scanner::new(&file_contents);
            let tokens = scanner.scan_tokens();

            let mut result = 0;
            if scanner.had_error() {
                result = 65;
            }

            for token in &tokens {
                println!("{}", token);
            }

            exit(result);
        }
        _ => eprintln!("Unknown command: {command}"),
    }
}
