use std::str::Chars;

pub struct Tokenizer<'a> {
    pub iter: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    // Process each character in the iterator
    // and print a token in the form
    // `TYPE LEXEME null` if the token is valid or
    // `UNDEFINED TOKEN` if not.
    pub fn tokenize(&self) {
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
            _ => println!("UNDEFINED TOKEN"),
        });
    }
}
