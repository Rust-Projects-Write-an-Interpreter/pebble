use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::token::{Token, TokenError, TokenResult, TokenType};

pub struct Tokenizer<'a> {
    pub iter: Peekable<Chars<'a>>,
    pub current_character: Option<char>,
    pub errors: bool,

    // A map of single-character lexemes.
    pub sc_lexemes: HashMap<String, TokenType>,
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
    // and return a token result.
    fn tokenize(&self) -> Option<TokenResult> {
        // Extract the current character from
        // the Option and return if it is None.
        let c = self.current_character?;

        // Look for the current character in
        // the map of single-character lexemes.
        // Return None if no character can be found.
        let ttype = self.sc_lexemes.get(&String::from(c))?.clone();

        Some(Ok(Token {
            ttype,
            lexeme: String::from(c),
            literal: String::from("null"),
        }))
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        // Advance the current token and
        // return if we reached the end
        // of the input file.
        let current = self.advance()?;

        // Run the tokenizer on the current character
        // and return the result if not None.
        if let Some(t) = self.tokenize() {
            return Some(t);
        }

        // If we reach this point we couldn't
        // match the current character with
        // any lexeme, so this is an error.
        self.errors = true;
        Some(Err(TokenError {
            message: format!("Unexpected character: {}", current),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::build_lexemes_map;

    #[test]
    // Test that the iterator can be
    // correctly advanced to the next
    // character.
    fn test_advance() {
        let mut t = Tokenizer {
            iter: "some string".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        // This advances the iterator to 'o'
        let character = t.advance();

        assert_eq!(t.current_character, Some('o'));
        assert_eq!(character, Some('o'));
    }

    #[test]
    // Test that the method tokenize behaves
    // correctly when the tokeniser is not
    // initialised and there is no current
    // character.
    fn test_tokenize_no_current_character() {
        let t = Tokenizer {
            iter: "some string".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: HashMap::new(),
        };

        assert_eq!(t.tokenize(), None);
    }

    #[test]
    // Test that the method tokenize behaves
    // correctly when the tokeniser is
    // initialised and there is a current
    // character.
    fn test_tokenize_valid_character() {
        let mut t = Tokenizer {
            iter: ",".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: build_lexemes_map(&[(",", TokenType::Comma)]),
        };

        // This advances the iterator to the initial character
        t.advance();

        // Run the tokenizer on the initial character
        let token = t.tokenize();

        assert_eq!(
            token,
            Some(Ok(Token {
                ttype: TokenType::Comma,
                lexeme: String::from(","),
                literal: String::from("null"),
            }))
        );
    }

    #[test]
    // Test that the method tokenize behaves
    // correctly when the tokeniser is
    // initialised and there is a current
    // character, but the character is
    // not a valid lexeme.
    fn test_tokenize_invalid_character() {
        let mut t = Tokenizer {
            iter: "@".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: build_lexemes_map(&[(",", TokenType::Comma)]),
        };

        // This advances the iterator to the initial character
        t.advance();

        // Run the tokenizer on the initial character
        let token = t.tokenize();

        assert_eq!(token, None);
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;
    use crate::token::build_lexemes_map;

    #[test]
    // Test the iterator end-to-end on
    // a string of valid characters.
    fn test_valid_characters() {
        const LEXEMES: &[(&str, TokenType)] = &[
            (",", TokenType::Comma),
            ("(", TokenType::LParen),
            ("+", TokenType::Plus),
        ];

        let t = Tokenizer {
            iter: ",(+".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: build_lexemes_map(LEXEMES),
        };

        let result: Vec<TokenResult> = t.collect();

        assert_eq!(
            result,
            [
                Ok(Token {
                    ttype: TokenType::Comma,
                    lexeme: String::from(","),
                    literal: String::from("null")
                }),
                Ok(Token {
                    ttype: TokenType::LParen,
                    lexeme: String::from("("),
                    literal: String::from("null")
                }),
                Ok(Token {
                    ttype: TokenType::Plus,
                    lexeme: String::from("+"),
                    literal: String::from("null")
                }),
            ]
        );
    }

    #[test]
    // Test the iterator end-to-end on
    // a string that contains an invalid character.
    fn test_invalid_characters() {
        const LEXEMES: &[(&str, TokenType)] = &[
            (",", TokenType::Comma),
            ("(", TokenType::LParen),
            ("+", TokenType::Plus),
        ];

        let mut t = Tokenizer {
            iter: ",@+".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: build_lexemes_map(LEXEMES),
        };

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [
                Ok(Token {
                    ttype: TokenType::Comma,
                    lexeme: String::from(","),
                    literal: String::from("null")
                }),
                Err(TokenError {
                    message: String::from("Unexpected character: @")
                }),
                Ok(Token {
                    ttype: TokenType::Plus,
                    lexeme: String::from("+"),
                    literal: String::from("null")
                }),
            ]
        );
        assert!(t.errors);
    }

    #[test]
    // Test the iterator end-to-end on
    // an empty string.
    fn test_empty_input() {
        let t = Tokenizer {
            iter: "".chars().peekable(),
            current_character: None,
            errors: false,
            sc_lexemes: build_lexemes_map(&[]),
        };

        let result: Vec<TokenResult> = t.collect();

        assert_eq!(result, []);
    }
}
