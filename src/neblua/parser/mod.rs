use super::token::*;

pub fn parse(input: &Vec<TerminalToken>) -> Option<Token> {
    let mut parser = Parser::new(input);
    parser.parse()
}

struct Parser<'a> {
    index: usize,
    input: &'a Vec<TerminalToken>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<TerminalToken>) -> Self {
        Self { index: 0, input }
    }

    pub fn parse(&mut self) -> Option<Token> {
        Some(Token::FunctionCall(self.parse_function_call()?))
    }

    fn parse_function_call(&mut self) -> Option<FunctionCall> {
        let name = self.consume_name()?;
        let args = self.parse_args().expect("expected args");

        Some(FunctionCall { name, args })
    }

    fn parse_args(&mut self) -> Option<Args> {
        if !self.consume_begin_paren() {
            return None;
        }

        let mut exp_list = vec![self.parse_exp().expect("expected exp")];

        loop {
            if !self.consume_comma() {
                break;
            }

            let exp = self.parse_exp().expect("expected exp");
            exp_list.push(exp);
        }

        if !self.consume_end_paren() {
            panic!("expected ')'");
        }

        Some(Args { exp_list })
    }

    fn parse_exp(&mut self) -> Option<Exp> {
        match self.consume_literal_string() {
            Some(literal_string) => Some(Exp::LiteralString(literal_string)),
            None => None,
        }
    }

    fn consume_name(&mut self) -> Option<Name> {
        match self.input.get(self.index) {
            Some(TerminalToken::Name(name)) => {
                self.index += 1;
                Some(name.clone())
            }
            _ => None,
        }
    }

    fn consume_literal_string(&mut self) -> Option<LiteralString> {
        match self.input.get(self.index) {
            Some(TerminalToken::LiteralString(value)) => {
                self.index += 1;
                Some(value.clone())
            }
            _ => None,
        }
    }

    fn consume_begin_paren(&mut self) -> bool {
        match self.input.get(self.index) {
            Some(TerminalToken::BeginParen) => {
                self.index += 1;
                true
            }
            _ => false,
        }
    }

    fn consume_end_paren(&mut self) -> bool {
        match self.input.get(self.index) {
            Some(TerminalToken::EndParen) => {
                self.index += 1;
                true
            }
            _ => false,
        }
    }

    fn consume_comma(&mut self) -> bool {
        match self.input.get(self.index) {
            Some(TerminalToken::Comma) => {
                self.index += 1;
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod parser {
        use super::*;

        #[test]
        fn parse_function_call() {
            let input = vec![
                TerminalToken::Name(Name::new("print".as_bytes().to_vec())),
                TerminalToken::BeginParen,
                TerminalToken::LiteralString(LiteralString::new("hello".as_bytes().to_vec())),
                TerminalToken::Comma,
                TerminalToken::LiteralString(LiteralString::new("world".as_bytes().to_vec())),
                TerminalToken::EndParen,
            ];
            let mut parser = Parser::new(&input);

            let actual = parser.parse_function_call();

            let expected = Some(FunctionCall {
                name: Name::new("print".as_bytes().to_vec()),
                args: Args {
                    exp_list: vec![
                        Exp::LiteralString(LiteralString::new("hello".as_bytes().to_vec())),
                        Exp::LiteralString(LiteralString::new("world".as_bytes().to_vec())),
                    ],
                },
            });
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_args() {
            let input = vec![
                TerminalToken::BeginParen,
                TerminalToken::LiteralString(LiteralString::new("hello".as_bytes().to_vec())),
                TerminalToken::Comma,
                TerminalToken::LiteralString(LiteralString::new("world".as_bytes().to_vec())),
                TerminalToken::EndParen,
            ];
            let mut parser = Parser::new(&input);

            let actual = parser.parse_args();

            let expected = Some(Args {
                exp_list: vec![
                    Exp::LiteralString(LiteralString::new("hello".as_bytes().to_vec())),
                    Exp::LiteralString(LiteralString::new("world".as_bytes().to_vec())),
                ],
            });
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_exp() {
            let input = vec![TerminalToken::LiteralString(LiteralString::new(
                "hello".as_bytes().to_vec(),
            ))];
            let mut parser = Parser::new(&input);

            let actual = parser.parse_exp();

            let expected = Some(Exp::LiteralString(LiteralString::new(
                "hello".as_bytes().to_vec(),
            )));
            assert_eq!(actual, expected);
        }
    }
}
