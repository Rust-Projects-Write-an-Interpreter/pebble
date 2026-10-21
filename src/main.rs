use std::fs;
use std::path::PathBuf;
use std::process;

use crate::evaluate::evaluate;
use crate::parser::Parser;
use crate::statement::Statement;
use crate::token::{Token, TokenError, TokenResult};
use crate::tokenizer::tokenize;

use clap::{Parser as CParser, Subcommand as CSubcommand};

mod evaluate;
mod expression;
mod parser;
mod statement;
mod token;
mod tokenizer;

#[cfg(test)]
mod test_utils;

#[derive(CSubcommand)]
enum Commands {
    Evaluate { filename: PathBuf },
    Parse { filename: PathBuf },
    Run { filename: PathBuf },
    Tokenize { filename: PathBuf },
}

#[derive(CParser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn tokenize_source(filename: &PathBuf) -> Vec<TokenResult> {
    // Open the input file and read the content.
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename.display());
        String::new()
    });

    // Process the file and extract the token results.
    let results: Vec<TokenResult> = tokenize(file_contents.chars().peekable());

    results
}

fn tokenize_process_results(
    results: Vec<TokenResult>,
    mut ok: impl FnMut(Token),
    err: impl Fn(TokenError),
) {
    // Initialise the error flag.
    let mut errors: bool = false;

    // Iterate through the results and call
    // the provided functions according
    // to their variant.
    // Record if there are errors.
    for result in results {
        match result {
            Ok(token) => ok(token),
            Err(e) => {
                err(e);
                errors = true
            }
        }
    }

    // In case there were errors
    // exit with a specific code.
    if errors {
        process::exit(65);
    }
}

fn parse_source(filename: &PathBuf) -> Parser {
    // Process the source and
    // extract Tokens or errors.
    let results = tokenize_source(filename);

    // A vector of tokens extracted from the results.
    let mut tokens: Vec<Token> = Vec::new();

    tokenize_process_results(
        results,
        |token| tokens.push(token),
        |error| eprintln!("{}", error),
    );

    Parser::new(tokens)
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Evaluate { filename } => {
            // Process the source and
            // extract a parser.
            let mut p = parse_source(&filename);

            let parsed = p.expression().unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(65);
            });

            let result = evaluate(&parsed);

            match result {
                Ok(v) => println!("{}", v),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(70);
                }
            }
        }
        Commands::Parse { filename } => {
            // Process the source and
            // extract a parser.
            let mut p = parse_source(&filename);

            match p.expression() {
                Ok(v) => println!("{}", v),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(65);
                }
            }
        }
        Commands::Run { filename } => {
            // Process the source and
            // extract a parser.
            let mut p = parse_source(&filename);

            let statement = p.statement().unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(65);
            });

            match statement {
                Statement::Print(e) => {
                    let result = evaluate(&e);

                    match result {
                        Ok(v) => println!("{}", v),
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(70);
                        }
                    }
                }
            }
        }
        Commands::Tokenize { filename } => {
            // Process the source and
            // extract Tokens or errors.
            let results = tokenize_source(&filename);

            // Process all results, print
            // tokens on stdout and error
            // on the stderr.
            tokenize_process_results(
                results,
                |token| println!("{}", token),
                |error| eprintln!("{}", error),
            );
        }
    }
}
