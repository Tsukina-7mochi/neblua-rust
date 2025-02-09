pub mod error;

use super::token::{Token, TokenKind};
use error::Error;

/** Tokenizes the given input string into a sequence of tokens. */
pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokenizer = Tokenizer::new(input);
    let mut result = vec![];

    loop {
        match tokenizer.consume() {
            Some(token) => result.push(token?),
            None => break,
        }
    }

    Ok(result)
}

struct Tokenizer<'a> {
    index: usize,
    input: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            index: 0,
            input: input.as_bytes(),
        }
    }

    pub fn consume(&mut self) -> Option<Result<Token, Error>> {
        self.skip_whitespace();
        if self.is_empty() {
            return None;
        }

        let result = None
            .or_else(|| self.consume_char())
            .or_else(|| self.consume_literal_string())
            .or_else(|| self.consume_name().map(Ok))
            .unwrap_or_else(|| {
                let (line, column) = self.get_line_and_column_of(self.index);
                Err(Error::new_unexpected_character(
                    line,
                    column,
                    self.input[self.index] as char,
                ))
            });
        Some(result)
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.input.len()
    }

    fn get_line_and_column_of(&self, index: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for i in 0..index {
            if self.input[i] == b'\n' {
                line += 1;
                column = 1;
            } else if self.input[i] & 0b11000000 != 0b10000000 {
                // skip UTF-8 continuation byte
                column += 1;
            }
        }
        (line, column)
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() {
            if !self.input[self.index].is_ascii_whitespace() {
                break;
            }

            self.index += 1;
        }
    }

    fn consume_char(&mut self) -> Option<Result<Token, Error>> {
        let head_char = self.input.get(self.index)?;
        let token = match *head_char {
            b'(' => Token::new(self.index, TokenKind::BeginParen),
            b')' => Token::new(self.index, TokenKind::EndParen),
            b',' => Token::new(self.index, TokenKind::Comma),
            _ => return None,
        };

        self.index += 1;

        Some(Ok(token))
    }

    fn consume_literal_string(&mut self) -> Option<Result<Token, Error>> {
        let token_index = self.index;

        if !self.input.get(self.index).is_some_and(|x| *x == b'"') {
            return None;
        }

        let mut value = Vec::<u8>::new();
        loop {
            self.index += 1;

            let next_char = match self.input.get(self.index) {
                Some(x) => x,
                None => {
                    let (line, column) = self.get_line_and_column_of(self.index);
                    return Some(Err(Error::new_unexpected_end_of_input(line, column)));
                }
            };
            if *next_char == b'"' {
                self.index += 1;
                break;
            } else if *next_char == b'\\' {
                // escape sequence
                value.extend_from_slice(&self.input[self.index..self.index + 2]);
                self.index += 1;
            } else if *next_char & 0b11110000 == 0b11110000 {
                // 4-byte UTF-8 character
                value.extend_from_slice(&self.input[self.index..self.index + 4]);
                self.index += 3;
            } else if *next_char & 0b11100000 == 0b11100000 {
                // 3-byte UTF-8 character
                value.extend_from_slice(&self.input[self.index..self.index + 3]);
                self.index += 2;
            } else if *next_char & 0b11000000 == 0b11000000 {
                // 2-byte UTF-8 character
                value.extend_from_slice(&self.input[self.index..self.index + 2]);
                self.index += 1;
            } else {
                value.push(*next_char);
            }
        }

        Some(Ok(Token::new(token_index, TokenKind::LiteralString(value))))
    }

    fn consume_name(&mut self) -> Option<Token> {
        let token_index = self.index;
        let mut value = Vec::<u8>::new();

        match self.input.get(self.index) {
            Some(x) if x.is_ascii_alphabetic() => {
                value.push(*x);
            }
            Some(b'_') => {
                value.push(b'_');
            }
            _ => return None,
        }

        loop {
            self.index += 1;

            match self.input.get(self.index) {
                Some(x) if x.is_ascii_alphanumeric() => {
                    value.push(*x);
                }
                Some(b'_') => {
                    value.push(b'_');
                }
                _ => break,
            }
        }

        Some(Token::new(token_index, TokenKind::Name(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tokenize {
        use super::*;

        #[test]
        fn empty_string() {
            let tokens = tokenize(" ");
            assert_eq!(tokens, Ok(vec![]));
        }

        #[test]
        fn print_hello() {
            let tokens = tokenize("print(\"hello\")");
            assert_eq!(
                tokens,
                Ok(vec![
                    Token::new(0, TokenKind::Name("print".as_bytes().to_vec())),
                    Token::new(5, TokenKind::BeginParen),
                    Token::new(6, TokenKind::LiteralString("hello".as_bytes().to_vec())),
                    Token::new(13, TokenKind::EndParen),
                ])
            );
        }
    }

    #[cfg(test)]
    mod tokenizer {
        use super::*;

        #[test]
        fn consume_begin_paren() {
            let mut tokenizer = Tokenizer::new("(");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(0, TokenKind::BeginParen)))
            );
        }

        #[test]
        fn consume_end_paren() {
            let mut tokenizer = Tokenizer::new(")");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(0, TokenKind::EndParen)))
            );
        }

        #[test]
        fn consume_comma() {
            let mut tokenizer = Tokenizer::new(",");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(0, TokenKind::Comma)))
            );
        }

        #[test]
        fn consume_literal_string() {
            let mut tokenizer = Tokenizer::new("\"hello\"");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(
                    0,
                    TokenKind::LiteralString("hello".as_bytes().to_vec())
                )))
            );
        }

        #[test]
        fn consume_literal_string_includes_escape_sequence() {
            let mut tokenizer = Tokenizer::new("\"hello\\\"world\"");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(
                    0,
                    TokenKind::LiteralString("hello\\\"world".as_bytes().to_vec())
                )))
            );
        }

        #[test]
        fn consume_literal_string_contains_utf8_2_byte_char() {
            let mut tokenizer = Tokenizer::new("\"α\"");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(
                    0,
                    TokenKind::LiteralString("α".as_bytes().to_vec())
                )))
            );
        }

        #[test]
        fn consume_literal_string_contains_utf8_3_byte_char() {
            let mut tokenizer = Tokenizer::new("\"あ\"");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(
                    0,
                    TokenKind::LiteralString("あ".as_bytes().to_vec())
                )))
            );
        }

        #[test]
        fn consume_literal_string_contains_utf8_4_byte_char() {
            let mut tokenizer = Tokenizer::new("\"💖\"");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(
                    0,
                    TokenKind::LiteralString("💖".as_bytes().to_vec())
                )))
            );
        }

        #[test]
        fn consume_name() {
            let mut tokenizer = Tokenizer::new("_foo_123");
            assert_eq!(
                tokenizer.consume(),
                Some(Ok(Token::new(
                    0,
                    TokenKind::Name("_foo_123".as_bytes().to_vec())
                )))
            );
        }
    }
}
