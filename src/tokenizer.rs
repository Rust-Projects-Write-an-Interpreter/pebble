use std::str::Chars;

pub struct Tokenizer<'a> {
    pub iter: Chars<'a>,
    pub errors: bool,
}

impl<'a> Tokenizer<'a> {
    // Process each character in the iterator
    // and print a token in the form
    // `TYPE LEXEME null` if the token is valid or
    // an error message if not.
    pub fn tokenize(&mut self) {
        self.iter.clone().for_each(|c| match c {
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            '+' => println!("PLUS + null"),
            ';' => println!("SEMICOLON ; null"),
            '*' => println!("STAR * null"),
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            other => {
                self.errors = true;
                eprintln!("[line 1] Error: Unexpected character: {}", other);
            }
        });
    }
}
