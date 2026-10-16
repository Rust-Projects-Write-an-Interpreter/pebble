use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::token::{
    build_lexemes_map, Keyword, Token, TokenError, TokenLiteral, TokenResult, TokenType,
    MULTI_CHARACTER_LEXEMES, SINGLE_CHARACTER_LEXEMES,
};

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
            literal: TokenLiteral::Null,
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
            literal: TokenLiteral::Null,
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
                            literal: TokenLiteral::String(s.trim_matches('"').to_owned()),
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

    // Test the current characters and
    // try to match the beginning of a number,
    // with or without decimals.
    fn number(&mut self) -> Option<TokenResult> {
        // If the current character is None return None.
        let c = self.current_character?;

        // If the current character is not a digit return None.
        if !c.is_ascii_digit() {
            return None;
        }

        // Prepare the string with the initial digit.
        let mut s = c.to_string();

        loop {
            // We already know the current character
            // is part of the number, so let's start
            // from the next one.
            self.advance();

            match self.current_character {
                Some(v) => {
                    // The current character is a digit.
                    // We can store it and move on to
                    // the next character.
                    if v.is_ascii_digit() {
                        s.push(v);
                        continue;
                    }

                    // The current character is a dot.
                    // We should consider it only if the
                    // resulting string doesn't already
                    // contain a dot. Then we should check
                    // if it's followed by other numbers.
                    // If not, we stop here.
                    if v == '.' && !s.contains('.') {
                        match self.iter.peek() {
                            Some(p) if p.is_ascii_digit() => {
                                // The dot is part of the number
                                s.push(v);
                                continue;
                            }
                            _ => {
                                // The character after the dot is
                                // not a digit or there is nothing
                                // else to read. We can stop here.
                                break;
                            }
                        }
                    }

                    // The current character is neither a
                    // digit nor a dot followed by a digit,
                    // so we can stop here.
                    break;
                }
                None => {
                    // There is nothing left to read.
                    // We can stop here.
                    break;
                }
            }
        }

        // This removes multiple zeros after the dot.
        let literal = TokenLiteral::Number(
            s.parse::<f64>()
                .expect("tokenizer produced an invalid number lexeme"),
        );

        // We don't need to advance as the current
        // character is already something that doesn't
        // belong to the number.
        Some(Ok(Token {
            ttype: TokenType::Number,
            lexeme: s.clone(),
            literal,
        }))
    }

    // Test the current characters and
    // try to match an identifier or
    // a reserved word.
    fn identifier(&mut self) -> Option<TokenResult> {
        // If the current character is None return None.
        let c = self.current_character?;

        // If the current character is not a letter
        // or an underscore return None.
        if !c.is_ascii_alphabetic() && c != '_' {
            return None;
        }

        let mut s = c.to_string();

        loop {
            // We already know the current character
            // is part of the identifier, so let's start
            // from the next one.
            self.advance();

            match self.current_character {
                Some(v) => {
                    // After the first character (a letter
                    // or an underscore) we accept more letters,
                    // underscores, or numbers.
                    // If the character matches we can store it and
                    // move on to the next character.
                    if v.is_ascii_alphanumeric() || v == '_' {
                        s.push(v);
                        continue;
                    }

                    // The current character is neither a
                    // letter nor an underscore, so we can stop here.
                    break;
                }
                None => {
                    // There is nothing left to read.
                    // We can stop here.
                    break;
                }
            }
        }

        // Check if this is a reserved word.
        if let Ok(k) = s.parse::<Keyword>() {
            return Some(Ok(Token {
                ttype: TokenType::Reserved(k),
                lexeme: s.clone(),
                literal: TokenLiteral::Null,
            }));
        }

        // We don't need to advance as the current
        // character is already something that doesn't
        // belong to the identifier
        Some(Ok(Token {
            ttype: TokenType::Identifier,
            lexeme: s.clone(),
            literal: TokenLiteral::Null,
        }))
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current character is None return None.
        let current = self.current_character?;

        // Try to match an identifier.
        if let Some(t) = self.identifier() {
            return Some(t);
        }

        // Try to match a number.
        if let Some(t) = self.number() {
            return Some(t);
        }

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

pub fn tokenize(chars: Peekable<Chars>) -> Vec<TokenResult> {
    // Create a new tokenizer and
    // instantiate it with the content
    // of the input file.
    let mut t = Tokenizer {
        iter: chars,
        current_character: None,
        current_line: 1,
        errors: false,
        sc_lexemes: build_lexemes_map(SINGLE_CHARACTER_LEXEMES),
        mc_lexemes: build_lexemes_map(MULTI_CHARACTER_LEXEMES),
    };

    // Initialise the iterator
    t.advance();

    // Filter out comments, whitespaces, and newlines
    let filtered = t.filter(|i| {
        i.as_ref().map_or(true, |token| {
            !matches!(
                token.ttype,
                TokenType::Comment | TokenType::Whitespace | TokenType::Newline
            )
        })
    });

    // Collect the results in a vector
    let mut results: Vec<TokenResult> = filtered.collect();

    // Make sure the last token is the End Of File
    results.push(Ok(Token {
        ttype: TokenType::Eof,
        lexeme: String::from(""),
        literal: TokenLiteral::Null,
    }));

    results
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
                literal: TokenLiteral::Null,
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
                literal: TokenLiteral::Null,
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
                literal: TokenLiteral::String(String::from("some string")),
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

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is not
    // initialised and there is no current
    // character.
    fn test_number_no_current_character() {
        let mut t = Tokenizer {
            iter: "123".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        assert_eq!(t.number(), None);
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character, but the character is
    // not a digit.
    fn test_number_no_digit() {
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

        assert_eq!(t.number(), None);
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character, and the character is
    // a single digit.
    fn test_number_single_digit() {
        let mut t = Tokenizer {
            iter: "4".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.number(),
            Some(Ok(Token {
                ttype: TokenType::Number,
                lexeme: String::from("4"),
                literal: TokenLiteral::Number(4.0),
            }))
        );
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // digits without a dot.
    fn test_number_multiple_digits_without_dot() {
        let mut t = Tokenizer {
            iter: "4242".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.number(),
            Some(Ok(Token {
                ttype: TokenType::Number,
                lexeme: String::from("4242"),
                literal: TokenLiteral::Number(4242.0),
            }))
        );
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // digits with a dot followed
    // by non-digits.
    fn test_number_multiple_digits_with_dot_without_decimals() {
        let mut t = Tokenizer {
            iter: "4242.characters".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.number(),
            Some(Ok(Token {
                ttype: TokenType::Number,
                lexeme: String::from("4242"),
                literal: TokenLiteral::Number(4242.0),
            }))
        );
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // digits with a dot followed
    // by other digits.
    fn test_number_multiple_digits_with_dot_and_decimals() {
        let mut t = Tokenizer {
            iter: "4242.42".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.number(),
            Some(Ok(Token {
                ttype: TokenType::Number,
                lexeme: String::from("4242.42"),
                literal: TokenLiteral::Number(4242.42),
            }))
        );
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // digits with multiple dots.
    fn test_number_multiple_dots() {
        let mut t = Tokenizer {
            iter: "1.2.3".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.number(),
            Some(Ok(Token {
                ttype: TokenType::Number,
                lexeme: String::from("1.2"),
                literal: TokenLiteral::Number(1.2),
            }))
        );
    }

    #[test]
    // Test that the method identifier
    // behaves correctly when the tokeniser is not
    // initialised and there is no current
    // character.
    fn test_identifier_no_current_character() {
        let mut t = Tokenizer {
            iter: "foo".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        assert_eq!(t.identifier(), None);
    }

    #[test]
    // Test that the method identifier
    // behaves correctly when the tokeniser is
    // initialised and there is a current
    // character, but the character is
    // not a letter or an underscore.
    fn test_identifier_no_letter_or_underscore() {
        let mut t = Tokenizer {
            iter: "(".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(t.identifier(), None);
    }

    #[test]
    // Test that the method identifier
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // an identifier that starts with
    // a digit.
    fn test_identifier_cannot_start_with_digit() {
        let mut t = Tokenizer {
            iter: "8foo".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(t.identifier(), None);
    }

    #[test]
    // Test that the method identifier
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // an identifier that is made only
    // of letters.
    fn test_identifier_all_letters() {
        let mut t = Tokenizer {
            iter: "foo".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.identifier(),
            Some(Ok(Token {
                ttype: TokenType::Identifier,
                lexeme: String::from("foo"),
                literal: TokenLiteral::Null,
            }))
        );
    }

    #[test]
    // Test that the method identifier
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // an identifier that starts with
    // an underscore.
    fn test_identifier_start_with_underscore() {
        let mut t = Tokenizer {
            iter: "_foo".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.identifier(),
            Some(Ok(Token {
                ttype: TokenType::Identifier,
                lexeme: String::from("_foo"),
                literal: TokenLiteral::Null,
            }))
        );
    }

    #[test]
    // Test that the method identifier
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // an identifier that starts with a letter
    // and is made of letters and digits.
    fn test_identifier_can_contain_digits() {
        let mut t = Tokenizer {
            iter: "foo8".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.identifier(),
            Some(Ok(Token {
                ttype: TokenType::Identifier,
                lexeme: String::from("foo8"),
                literal: TokenLiteral::Null,
            }))
        );
    }

    #[test]
    // Test that the method number
    // behaves correctly when the tokeniser is
    // initialised, there is a current
    // character, and the code contains
    // an identifier that is a
    // reserved word.
    fn test_identifier_reserved_word() {
        let mut t = Tokenizer {
            iter: "while".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // This advances the iterator to the initial character
        t.advance();

        assert_eq!(
            t.identifier(),
            Some(Ok(Token {
                ttype: TokenType::Reserved(Keyword::While),
                lexeme: Keyword::While.lexeme(),
                literal: TokenLiteral::Null,
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
                    literal: TokenLiteral::Null
                }),
                Ok(Token {
                    ttype: TokenType::LParen,
                    lexeme: String::from("("),
                    literal: TokenLiteral::Null
                }),
                Ok(Token {
                    ttype: TokenType::Plus,
                    lexeme: String::from("+"),
                    literal: TokenLiteral::Null
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
                    literal: TokenLiteral::Null
                }),
                Err(TokenError {
                    message: String::from("Unexpected character: @"),
                    line: 1,
                }),
                Ok(Token {
                    ttype: TokenType::Plus,
                    lexeme: String::from("+"),
                    literal: TokenLiteral::Null
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
                literal: TokenLiteral::Null,
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
                literal: TokenLiteral::Null,
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
                literal: TokenLiteral::String(String::from("a string")),
            })]
        );
    }

    #[test]
    // Test the iterator end-to-end on
    // a number with decimals.
    fn test_number() {
        let mut t = Tokenizer {
            iter: "4242.42".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // Initialise the iterator
        t.advance();

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [Ok(Token {
                ttype: TokenType::Number,
                lexeme: String::from("4242.42"),
                literal: TokenLiteral::Number(4242.42),
            })]
        );
    }

    #[test]
    // Test the iterator end-to-end on
    // a number with multiple dots.
    fn test_number_multiple_dots() {
        let mut t = Tokenizer {
            iter: "1.2.3".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // Initialise the iterator
        t.advance();

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [
                Ok(Token {
                    ttype: TokenType::Number,
                    lexeme: String::from("1.2"),
                    literal: TokenLiteral::Number(1.2),
                }),
                Err(TokenError {
                    message: "Unexpected character: .".to_string(),
                    line: 1
                }),
                Ok(Token {
                    ttype: TokenType::Number,
                    lexeme: String::from("3"),
                    literal: TokenLiteral::Number(3.0),
                }),
            ]
        );
    }

    #[test]
    // Test the iterator end-to-end on
    // an identifier.
    fn test_identifier() {
        let mut t = Tokenizer {
            iter: "foo".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // Initialise the iterator
        t.advance();

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [Ok(Token {
                ttype: TokenType::Identifier,
                lexeme: String::from("foo"),
                literal: TokenLiteral::Null,
            })]
        );
    }

    #[test]
    // Test the iterator end-to-end on
    // a reserved word.
    fn test_reserved_word() {
        let mut t = Tokenizer {
            iter: "while".chars().peekable(),
            current_character: None,
            current_line: 1,
            errors: false,
            sc_lexemes: HashMap::new(),
            mc_lexemes: HashMap::new(),
        };

        // Initialise the iterator
        t.advance();

        let mut result = Vec::<TokenResult>::new();
        for token in &mut t {
            result.push(token);
        }

        assert_eq!(
            result,
            [Ok(Token {
                ttype: TokenType::Reserved(Keyword::While),
                lexeme: Keyword::While.lexeme(),
                literal: TokenLiteral::Null,
            })]
        );
    }
}

#[cfg(test)]
mod tokenize_tests {
    use super::*;

    #[test]
    // Test the function tokenize end-to-end.
    fn test_tokenize() {
        let results: Vec<TokenResult> = tokenize("for(==)".chars().peekable());

        assert_eq!(
            results,
            [
                Ok(Token {
                    ttype: TokenType::Reserved(Keyword::For),
                    lexeme: String::from("for"),
                    literal: TokenLiteral::Null,
                }),
                Ok(Token {
                    ttype: TokenType::LParen,
                    lexeme: String::from("("),
                    literal: TokenLiteral::Null,
                }),
                Ok(Token {
                    ttype: TokenType::EqualEqual,
                    lexeme: String::from("=="),
                    literal: TokenLiteral::Null,
                }),
                Ok(Token {
                    ttype: TokenType::RParen,
                    lexeme: String::from(")"),
                    literal: TokenLiteral::Null,
                }),
                Ok(Token {
                    ttype: TokenType::Eof,
                    lexeme: String::from(""),
                    literal: TokenLiteral::Null,
                }),
            ]
        );
    }
}
