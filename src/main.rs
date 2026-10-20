use std::env;
use std::fs;
use std::process;

use crate::parser::Parser;
use crate::token::{Token, TokenResult};
use crate::tokenizer::tokenize;

mod parser;
mod token;
mod tokenizer;

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
            // Open the input file and read the content.
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // Initialise the error flag.
            let mut errors: bool = false;

            // Process the file and extract the
            // token results.
            let results: Vec<TokenResult> = tokenize(file_contents.chars().peekable());

            // Iterate through the results and print
            // them accordingly on the stdout or the
            // stderr. Record if there are errors.
            for result in results {
                match result {
                    Ok(token) => println!("{}", token),
                    Err(e) => {
                        eprintln!("{}", e);
                        errors = true;
                    }
                }
            }

            // In case there were errors
            // exit with a specific code.
            if errors {
                process::exit(65);
            }
        }
        "parse" => {
            // Open the input file and read the content
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut errors: bool = false;

            // Process the file and extract the
            // token results.
            let results: Vec<TokenResult> = tokenize(file_contents.chars().peekable());

            let mut tokens: Vec<Token> = Vec::new();

            // Iterate through the results and store
            // them or print them on the stderr according
            // to their nature.
            for result in results {
                match result {
                    Ok(token) => tokens.push(token),
                    Err(e) => {
                        eprintln!("{}", e);
                        errors = true;
                    }
                }
            }

            // In case there were errors
            // exit with a specific code.
            if errors {
                process::exit(65);
            }

            let mut p = Parser::new(tokens);

            match p.expression() {
                Some(v) => println!("{}", v),
                None => panic!(),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
