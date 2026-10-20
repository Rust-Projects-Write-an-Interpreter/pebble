use clap::{Parser as CParser, Subcommand as CSubcommand};

#[derive(CSubcommand)]
enum Commands {}

#[derive(CParser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {}
