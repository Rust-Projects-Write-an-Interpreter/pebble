use std::env;
use std::fs;
use std::process;

use crate::tokenizer::Tokenizer;

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
                iter: file_contents.chars(),
                current_character: None,
                errors: false,
            };

            // Iterate on the tokenizer.
            for _item in &mut t {
                continue;
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
