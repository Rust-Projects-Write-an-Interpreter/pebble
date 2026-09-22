use std::env;
use std::fs;
use std::str::Chars;

// Process each character in the iterator
// `chars` and print a token in the form
// `TYPE LEXEME null` if the token is valid or
// `UNDEFINED TOKEN` if not.
fn tokenize(chars: Chars) {
    chars.for_each(|c| match c {
        '(' => println!("LEFT_PAREN ( null"),
        ')' => println!("RIGHT_PAREN ) null"),
        '{' => println!("LEFT_BRACE {{ null"),
        '}' => println!("RIGHT_BRACE }} null"),
        _ => println!("UNDEFINED TOKEN"),
    });
}

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

            // Call the tokenizer of the whole
            // content of the input file.
            tokenize(file_contents.chars());
            println!("EOF  null");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
