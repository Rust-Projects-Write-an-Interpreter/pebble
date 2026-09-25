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
    pub fn advance(&mut self) -> Option<char> {
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

        // We read the current character and transformed
        // it into a token, so we need to make sure
        // the iterator is ready to parse the next character.
        self.advance();

        Some(Ok(Token {
            ttype,
            lexeme: full,
            literal: String::from("null"),
        }))
    }

    // Test the current character and
    // try to match one of the single-character lexemes
    fn single_character_token(&mut self) -> Option<TokenResult> {
        // Extract the current character from
        // the Option and return if it is None.
        let c = self.current_character?;

        // Look for the current character in
        // the map of single-character lexemes.
        // Return None if no character can be found.
        let ttype = self.sc_lexemes.get(&String::from(c))?.clone();

        // If the token we read is a newline we need to
        // update the current line.
        if ttype == TokenType::Newline {
            self.current_line += 1;
        }

        // We read the current character and transformed
        // it into a token, so we need to make sure
        // the iterator is ready to parse the next character.
        self.advance();

        Some(Ok(Token {
            ttype,
            lexeme: String::from(c),
            literal: String::from("null"),
        }))
    }

    // Test the current characters and
    // try to match the beginning of a string.
    // Returns an error if the string is unterminated.
    fn string(&mut self) -> Option<TokenResult> {
        // If the current character is None return None.
        let c = self.current_character?;

        // If this is not double quotes this is not the beginning of a string.
        if c != '"' {
            return None;
        }

        let mut s = String::from(c);

        loop {
            // We already know the current character
            // is part of the string, so let's start
            // from the next one.
            self.advance();

            match self.current_character {
                Some(v) => {
                    s.push(v);

                    if v == '"' {
                        // We just captured the last character of the
                        // string (the closing quotes) so we make sure
                        // the iterator is ready to parse the next character.
                        self.advance();

                        // Return the token using the full string
                        // with quotes as lexemes (this is what we processed)
                        // and the string without quotes as literal value.
                        return Some(Ok(Token {
                            ttype: TokenType::String,
                            lexeme: s.clone(),
                            literal: s.trim_matches('"').to_owned(),
                        }));
                    }
                }
                None => {
                    // No need to advance. If we reach this point
                    // there is nothing left to read in the file
                    // and the string hasn't been terminated.
                    // Goodbye.
                    self.errors = true;
                    return Some(Err(TokenError {
                        message: String::from("Unterminated string."),
                        line: self.current_line,
                    }));
                }
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current character is None return None.
        let current = self.current_character?;

        // Try to match a string.
        if let Some(t) = self.string() {
            return Some(t);
        }

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

        // We detected an error, so we advance
        // the iterator to continue processing
        // the input.
        self.advance();

        Some(Err(TokenError {
            message: format!("Unexpected character: {}", current),
            line: self.current_line,
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
        let mut t = Tokenizer {
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

    #[test]
    // Test that the method string
    // behaves correctly when the tokeniser is not
    // initialised and there is no current
    // character.
    fn test_string_no_current_character() {
        let mut t = Tokenizer {
            iter: "characters".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        assert_eq!(t.string(), None);
    }

    #[test]
    // Test that the method string
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character, but the character is
    // not the opening double quotes.
    fn test_string_no_double_quotes() {
        let mut t = Tokenizer {
            iter: "characters".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(t.string(), None);
    }

    #[test]
    // Test that the method string
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // a string with both opening and
    // closing double quotes.
    fn test_string_valid_string() {
        let mut t = Tokenizer {
            iter: "\"some string\"".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.string(),
            Some(Ok(Token {
                ttype: TokenType::String,
                lexeme: String::from("\"some string\""),
                literal: String::from("some string"),
            }))
        );
    }

    #[test]
    // Test that the method string
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // a string with only the
    // opening double quotes.
    fn test_string_unterminated() {
        let mut t = Tokenizer {
            iter: "\"some string".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.string(),
            Some(Err(TokenError {
                message: String::from("Unterminated string."),
                line: 1
            }))
        );
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

        let mut t = Tokenizer {
            iter: ",(+".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(LEXEMES),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

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

        // This advances the iterator to the initial character
        t.advance();

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
                    message: String::from("Unexpected character: @"),
                    line: 1,
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
        let mut t = Tokenizer {
            iter: "".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(&[]),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

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

        // This advances the iterator to the initial character
        t.advance();

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

    #[test]
    // Test the iterator end-to-end on
    // a single new line character.
    fn test_newline() {
        const SINGLE_LEXEMES: &[(&str, TokenType)] = &[("\n", TokenType::Newline)];

        let mut t = Tokenizer {
            iter: "\n".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: build_lexemes_map(SINGLE_LEXEMES),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [Ok(Token {
                ttype: TokenType::Newline,
                lexeme: String::from("\n"),
                literal: String::from("null"),
            })]
        );
        assert_eq!(t.current_line, 2);
    }

    #[test]
    // Test the iterator end-to-end on
    // a well-defined string.
    fn test_string() {
        let mut t = Tokenizer {
            iter: "\"a string\"".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [Ok(Token {
                ttype: TokenType::String,
                lexeme: String::from("\"a string\""),
                literal: String::from("a string"),
            })]
        );
    }
}
