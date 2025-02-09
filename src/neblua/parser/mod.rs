use super::ast::node::*;
use super::token::{Token, TokenKind};

pub fn parse(input: &Vec<Token>) -> Option<Node> {
    let mut parser = Parser::new(input);
    parser.parse()
}

struct Parser<'a> {
    index: usize,
    input: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<Token>) -> Self {
        Self { index: 0, input }
    }

    pub fn parse(&mut self) -> Option<Node> {
        Some(Node::FunctionCall(self.parse_function_call()?))
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
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::Name(value)) => {
                self.index += 1;
                Some(Name {
                    value: value.clone(),
                })
            }
            _ => None,
        }
    }

    fn consume_literal_string(&mut self) -> Option<LiteralString> {
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::LiteralString(value)) => {
                self.index += 1;
                Some(LiteralString {
                    value: value.clone(),
                })
            }
            _ => None,
        }
    }

    fn consume_begin_paren(&mut self) -> bool {
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::BeginParen) => {
                self.index += 1;
                true
            }
            _ => false,
        }
    }

    fn consume_end_paren(&mut self) -> bool {
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::EndParen) => {
                self.index += 1;
                true
            }
            _ => false,
        }
    }

    fn consume_comma(&mut self) -> bool {
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::Comma) => {
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
                Token::new(0, TokenKind::Name("print".as_bytes().to_vec())),
                Token::new(5, TokenKind::BeginParen),
                Token::new(6, TokenKind::LiteralString("hello".as_bytes().to_vec())),
                Token::new(13, TokenKind::Comma),
                Token::new(14, TokenKind::LiteralString("world".as_bytes().to_vec())),
                Token::new(21, TokenKind::EndParen),
            ];
            let mut parser = Parser::new(&input);

            let actual = parser.parse_function_call();

            let expected = Some(FunctionCall {
                name: Name {
                    value: "print".as_bytes().to_vec(),
                },
                args: Args {
                    exp_list: vec![
                        Exp::LiteralString(LiteralString {
                            value: "hello".as_bytes().to_vec(),
                        }),
                        Exp::LiteralString(LiteralString {
                            value: "world".as_bytes().to_vec(),
                        }),
                    ],
                },
            });
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_args() {
            let input = vec![
                Token::new(0, TokenKind::BeginParen),
                Token::new(1, TokenKind::LiteralString("hello".as_bytes().to_vec())),
                Token::new(8, TokenKind::Comma),
                Token::new(9, TokenKind::LiteralString("world".as_bytes().to_vec())),
                Token::new(16, TokenKind::EndParen),
            ];
            let mut parser = Parser::new(&input);

            let actual = parser.parse_args();

            let expected = Some(Args {
                exp_list: vec![
                    Exp::LiteralString(LiteralString {
                        value: "hello".as_bytes().to_vec(),
                    }),
                    Exp::LiteralString(LiteralString {
                        value: "world".as_bytes().to_vec(),
                    }),
                ],
            });
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_exp() {
            let input = vec![Token::new(
                0,
                TokenKind::LiteralString("hello".as_bytes().to_vec()),
            )];
            let mut parser = Parser::new(&input);

            let actual = parser.parse_exp();

            let expected = Some(Exp::LiteralString(LiteralString {
                value: "hello".as_bytes().to_vec(),
            }));
            assert_eq!(actual, expected);
        }
    }
}
