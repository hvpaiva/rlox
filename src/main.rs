use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, WrapErr};

use codecrafters_interpreter as rlox;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
    Parse { filename: PathBuf },
    Evaluate { filename: PathBuf },
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Tokenize { filename } => {
            let mut any_cc_err = false;

            let file_contents = read_file(filename)?;

            for token in rlox::Lexer::new(&file_contents) {
                let token = match token {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{e:?}");
                        if let Some(unrecognized) =
                            e.downcast_ref::<rlox::lexer::errors::SingleTokenError>()
                        {
                            any_cc_err = true;
                            eprintln!(
                                "[line {}] Error: Unexpected character: {}",
                                unrecognized.line(),
                                unrecognized.token
                            );
                        } else if let Some(unterminated) =
                            e.downcast_ref::<rlox::lexer::errors::StringTerminationError>()
                        {
                            any_cc_err = true;
                            eprintln!("[line {}] Error: Unterminated string.", unterminated.line(),);
                        }
                        continue;
                    }
                };
                println!("{token}");
            }
            println!("EOF  null");

            if any_cc_err {
                std::process::exit(65);
            }
        }
        Command::Parse { filename: _ } => todo!(),
        Command::Evaluate { filename: _ } => todo!(),
    }

    Ok(())
}

fn read_file(filename: PathBuf) -> Result<String, miette::Error> {
    let file_contents = fs::read_to_string(&filename)
        .into_diagnostic()
        .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

    Ok(file_contents)
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
