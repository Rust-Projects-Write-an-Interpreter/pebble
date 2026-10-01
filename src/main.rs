use std::env;
use std::fs;
use std::process;

use crate::token::{build_lexemes_map, MULTI_CHARACTER_LEXEMES, SINGLE_CHARACTER_LEXEMES};
use crate::tokenizer::Tokenizer;

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
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // Create a new tokenizer and
            // instantiate it with the content
            // of the input file.
            let mut t = Tokenizer {
                iter: file_contents.chars().peekable(),
                current_character: None,
                errors: false,
                sc_lexemes: build_lexemes_map(SINGLE_CHARACTER_LEXEMES),
                mc_lexemes: build_lexemes_map(MULTI_CHARACTER_LEXEMES),
            };

            // Iterate on the tokenizer
            // and process each result.
            for token in &mut t {
                match token {
                    Ok(token) => {
                        println!("{}", token);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }

            println!("EOF  null");

            if t.errors {
                process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
