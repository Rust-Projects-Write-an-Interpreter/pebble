use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    pub iter: Peekable<Chars<'a>>,
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
    fn tokenize(&mut self) -> Option<()> {
        // Extract the current character from
        // the Option and return if it is None.
        let c = self.current_character?;

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
        };

        Some(())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        // Advance the current token and
        // return if we reached the end
        // of the input file.
        self.advance()?;

        // Run the tokenizer on the
        // current character.
        self.tokenize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that the iterator can be
    // correctly advanced to the next
    // character.
    fn test_advance() {
        let mut t = Tokenizer {
            iter: "some string".chars().peekable(),
            current_character: None,
            errors: false,
        };

        // This advances the iterator to the initial character
        t.advance();

        // This advances the iterator to 'o'
        let character = t.advance();

        assert_eq!(t.current_character, Some('o'));
        assert_eq!(character, Some('o'));
    }
}
