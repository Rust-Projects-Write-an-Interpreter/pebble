use std::fs;
use std::path::PathBuf;

use crate::token::TokenResult;
use crate::tokenizer::tokenize;

use clap::{Parser as CParser, Subcommand as CSubcommand};

mod token;
mod tokenizer;

#[derive(CSubcommand)]
enum Commands {
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

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Tokenize { filename } => {
            // Process the source and
            // extract Tokens or errors.
            tokenize_source(&filename);
        }
    }
}
