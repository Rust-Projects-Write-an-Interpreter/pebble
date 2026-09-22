use std::env;
use std::fs;

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
            let t = Tokenizer {
                iter: file_contents.chars(),
            };

            // Call the tokenizer.
            t.tokenize();

            println!("EOF  null");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
