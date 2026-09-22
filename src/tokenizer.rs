use std::str::Chars;

pub struct Tokenizer<'a> {
    pub iter: Chars<'a>,
    pub current_character: Option<char>,
    pub errors: bool,
}

impl<'a> Tokenizer<'a> {
    // Advance the iterator and
    // return the current character.
    fn advance(&mut self) -> Option<char> {
        // Advance the iterator and store
        // the current character for future
        // retrieval.
        self.current_character = self.iter.next();

        // Return the current character.
        self.current_character
    }

    // Process each character in the iterator
    // and print a token in the form
    // `TYPE LEXEME null` if the token is valid or
    // an error message if not.
    fn tokenize(&mut self, c: char) {
        match c {
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
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.next()?;

        self.tokenize(c);
        Some(())
    }
}
