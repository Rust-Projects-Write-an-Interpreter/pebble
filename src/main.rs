use std::fs;
use std::path::PathBuf;
use std::process;

use crate::token::{Token, TokenError, TokenResult};
use crate::tokenizer::tokenize;

use clap::{Parser as CParser, Subcommand as CSubcommand};

mod token;
mod tokenizer;

#[derive(CSubcommand)]
enum Commands {
    Parse { filename: PathBuf },
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
    ok: impl Fn(Token),
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

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Parse { filename } => {
            // Process the source and
            // extract Tokens or errors.
            tokenize_source(&filename);
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
