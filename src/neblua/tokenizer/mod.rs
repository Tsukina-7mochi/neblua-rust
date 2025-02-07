use super::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new(input);
    TokenizerIterator { tokenizer }.collect()
}

struct TokenizerIterator<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Iterator for TokenizerIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenizer.consume()
    }
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

    pub fn consume(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.consume_char()
            .or_else(|| self.consume_literal_string())
            .or_else(|| self.consume_name())
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() {
            if !self.input[self.index].is_ascii_whitespace() {
                break;
            }

            self.index += 1;
        }
    }

    fn consume_char(&mut self) -> Option<Token> {
        let head_char = self.input.get(self.index)?;
        let token = match *head_char {
            b'(' => Token::BeginParen,
            b')' => Token::EndParen,
            b',' => Token::Comma,
            _ => return None,
        };

        self.index += 1;

        Some(token)
    }

    fn consume_literal_string(&mut self) -> Option<Token> {
        if !self.input.get(self.index).is_some_and(|x| *x == b'"') {
            return None;
        }

        let mut value = Vec::<u8>::new();
        loop {
            self.index += 1;
            dbg!(&value);
            dbg!(self.index);

            let next_char = self.input.get(self.index).unwrap();
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

        Some(Token::LiteralString(value))
    }

    fn consume_name(&mut self) -> Option<Token> {
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

        Some(Token::Name(value))
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
            assert_eq!(tokens, vec![]);
        }

        #[test]
        fn print_hello() {
            let tokens = tokenize("print(\"hello\")");
            assert_eq!(
                tokens,
                vec![
                    Token::Name("print".as_bytes().to_vec()),
                    Token::BeginParen,
                    Token::LiteralString("hello".as_bytes().to_vec()),
                    Token::EndParen,
                ],
            );
        }
    }

    #[cfg(test)]
    mod tokenizer {
        use super::*;

        #[test]
        fn consume_begin_paren() {
            let mut tokenizer = Tokenizer::new(" ( ");
            assert_eq!(tokenizer.consume(), Some(Token::BeginParen));
        }

        #[test]
        fn consume_end_paren() {
            let mut tokenizer = Tokenizer::new(" ) ");
            assert_eq!(tokenizer.consume(), Some(Token::EndParen));
        }

        #[test]
        fn consume_comma() {
            let mut tokenizer = Tokenizer::new(" , ");
            assert_eq!(tokenizer.consume(), Some(Token::Comma));
        }

        #[test]
        fn consume_literal_string() {
            let mut tokenizer = Tokenizer::new(" \"hello\" ");
            assert_eq!(
                tokenizer.consume(),
                Some(Token::LiteralString("hello".as_bytes().to_vec()))
            );
        }

        #[test]
        fn consume_literal_string_includes_escape_sequence() {
            let mut tokenizer = Tokenizer::new(" \"hello\\\"world\" ");
            assert_eq!(
                tokenizer.consume(),
                Some(Token::LiteralString("hello\\\"world".as_bytes().to_vec()))
            );
        }

        #[test]
        fn consume_literal_string_contains_utf8_2_byte_char() {
            let mut tokenizer = Tokenizer::new(" \"α\" ");
            assert_eq!(
                tokenizer.consume(),
                Some(Token::LiteralString("α".as_bytes().to_vec()))
            );
        }

        #[test]
        fn consume_literal_string_contains_utf8_3_byte_char() {
            let mut tokenizer = Tokenizer::new(" \"あ\" ");
            assert_eq!(
                tokenizer.consume(),
                Some(Token::LiteralString("あ".as_bytes().to_vec()))
            );
        }

        #[test]
        fn consume_literal_string_contains_utf8_4_byte_char() {
            let mut tokenizer = Tokenizer::new(" \"💖\" ");
            assert_eq!(
                tokenizer.consume(),
                Some(Token::LiteralString("💖".as_bytes().to_vec()))
            );
        }

        #[test]
        fn consume_name() {
            let mut tokenizer = Tokenizer::new(" _foo_123 ");
            assert_eq!(
                tokenizer.consume(),
                Some(Token::Name("_foo_123".as_bytes().to_vec()))
            );
        }

        #[test]
        fn consume_name_fails_if_name_starts_with_numeric() {
            let mut tokenizer = Tokenizer::new(" 0_foo ");
            assert_eq!(tokenizer.consume(), None);
        }
    }
}
