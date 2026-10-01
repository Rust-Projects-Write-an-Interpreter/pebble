use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::token::{Token, TokenError, TokenResult, TokenType};

pub struct Tokenizer<'a> {
    pub iter: Peekable<Chars<'a>>,
    pub current_character: Option<char>,
    pub current_line: u16,
    pub errors: bool,

    // A map of single-character lexemes.
    pub sc_lexemes: HashMap<String, TokenType>,

    // A map of multi-character lexemes.
    pub mc_lexemes: HashMap<String, TokenType>,
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

    // Test the current and next characters and
    // try to match one of the multi-character lexemes
    fn multi_character_token(&mut self) -> Option<TokenResult> {
        // If the current character is None return None.
        let c = self.current_character?;

        // If there is nothing after this character just return None.
        let &n = self.iter.peek()?;

        // Build the full token.
        let full = [c, n].iter().collect::<String>();

        // If the full token is not in the multi-character
        // tokens just return None.
        let ttype = self.mc_lexemes.get(&String::from(&full))?.clone();

        // If we found a comment we need to
        // skip the whole line.
        if ttype == TokenType::Comment {
            // Advance the iterator until we
            // reach the end of line
            while let Some(v) = self.advance() {
                if v == '\n' {
                    break;
                }
            }

            // We are skipping this line,
            // increase the line number.
            self.current_line += 1;
        } else {
            // Since we found a match, we must
            // skip the second character.
            self.advance();
        }

        Some(Ok(Token {
            ttype,
            lexeme: full,
            literal: String::from("null"),
        }))
    }

    // Test the current character and
    // try to match one of the single-character lexemes
    fn single_character_token(&self) -> Option<TokenResult> {
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

        // Try to match a multi-character token.
        if let Some(t) = self.multi_character_token() {
            return Some(t);
        }

        // Try to match a single-character token.
        if let Some(t) = self.single_character_token() {
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
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        // This advances the iterator to 'o'
        let character = t.advance();

        assert_eq!(t.current_character, Some('o'));
        assert_eq!(character, Some('o'));
    }

    #[test]
    // Test that the method single_character_token
    // behaves correctly when the tokeniser is not
    // initialised and there is no current
    // character.
    fn test_tokenize_no_current_character() {
        let t = Tokenizer {
            iter: "some string".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        assert_eq!(t.single_character_token(), None);
    }

    #[test]
    // Test that the method single_character_token
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character.
    fn test_tokenize_valid_character() {
        let mut t = Tokenizer {
            iter: ",".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(&[(",", TokenType::Comma)]),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        // Run the tokenizer on the initial character
        let token = t.single_character_token();

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
    // Test that the method single_character_token
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character, but the character is
    // not a valid lexeme.
    fn test_tokenize_invalid_character() {
        let mut t = Tokenizer {
            iter: "@".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(&[(",", TokenType::Comma)]),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        // Run the tokenizer on the initial character
        let token = t.single_character_token();

        assert_eq!(token, None);
    }

    #[test]
    // Test that the method multi_character_token
    // behaves correctly when the tokeniser is not
    // initialised and there is no current
    // character.
    fn test_multi_character_no_current_character() {
        let mut t = Tokenizer {
            iter: "some string".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        assert_eq!(t.multi_character_token(), None);
    }

    #[test]
    // Test that the method multi_character_token
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character.
    fn test_multi_character_valid_character() {
        let mut t = Tokenizer {
            iter: "==".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: build_lexemes_map(&[("==", TokenType::EqualEqual)]),
        };

        // This advances the iterator to the initial character
        t.advance();

        // Run the tokenizer on the initial character
        let token = t.multi_character_token();

        assert_eq!(
            token,
            Some(Ok(Token {
                ttype: TokenType::EqualEqual,
                lexeme: String::from("=="),
                literal: String::from("null"),
            }))
        );
    }

    #[test]
    // Test that the method multi_character_token
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character, but the character is
    // not a valid lexeme.
    fn test_multi_character_invalid_character() {
        let mut t = Tokenizer {
            iter: "@".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: build_lexemes_map(&[("==", TokenType::EqualEqual)]),
        };

        // This advances the iterator to the initial character
        t.advance();

        // Run the tokenizer on the initial character
        let token = t.multi_character_token();

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
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(LEXEMES),
            mc_lexemes: HashMap::new(),
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
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(LEXEMES),
            mc_lexemes: HashMap::new(),
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
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(&[]),
            mc_lexemes: HashMap::new(),
        };

        let result: Vec<TokenResult> = t.collect();

        assert_eq!(result, []);
    }

    #[test]
    // Test the iterator end-to-end on
    // a commented line.
    fn test_comment() {
        const SINGLE_LEXEMES: &[(&str, TokenType)] = &[("/", TokenType::Slash)];

        const MULTI_LEXEMES: &[(&str, TokenType)] = &[("//", TokenType::Comment)];

        let mut t = Tokenizer {
            iter: "// Comment".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(SINGLE_LEXEMES),
            mc_lexemes: build_lexemes_map(MULTI_LEXEMES),
        };

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [Ok(Token {
                ttype: TokenType::Comment,
                lexeme: String::from("//"),
                literal: String::from("null"),
            })]
        );
        assert_eq!(t.current_line, 2);
    }
}
