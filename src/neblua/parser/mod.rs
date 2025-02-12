pub mod error;

use crate::neblua::ast::{node::*, Ast};
use crate::neblua::token::{Token, TokenKind};
use error::{Error, ErrorCause};

pub fn parse(input: &Vec<Token>) -> Result<Ast, Error> {
    let mut parser = Parser::new(input);
    let node = parser.parse()?;
    Ok(Ast { root: node })
}

struct Parser<'a> {
    index: usize,
    input: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<Token>) -> Self {
        Self { index: 0, input }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Node>, Error> {
        let function_call = match self.parse_function_call() {
            Some(Ok(function_call)) => function_call,
            Some(Err(err)) => return Err(err),
            None => {
                return Err(Error {
                    index: self.index,
                    cause: ErrorCause::UnexpectedToken(self.input[self.index].clone()),
                })
            }
        };
        if !self.consume_eof() {
            return Err(Error {
                index: self.index,
                cause: ErrorCause::UnexpectedToken(self.input[self.index].clone()),
            });
        }
        Ok(Box::new(function_call))
    }

    fn parse_function_call(&mut self) -> Option<Result<FunctionCall, Error>> {
        let name = self.consume_name()?;
        let args = match self.parse_args() {
            Some(Ok(args)) => args,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(Error {
                    index: self.index,
                    cause: ErrorCause::UnexpectedToken(self.input[self.index].clone()),
                }))
            }
        };

        Some(Ok(FunctionCall { name, args }))
    }

    fn parse_args(&mut self) -> Option<Result<Args, Error>> {
        if !self.consume_begin_paren() {
            return None;
        }

        let first_exp = match self.parse_exp() {
            Some(exp) => exp,
            None => {
                return Some(Err(Error {
                    index: self.index,
                    cause: ErrorCause::UnexpectedToken(self.input[self.index].clone()),
                }))
            }
        };
        let mut exp_list = vec![first_exp];

        loop {
            if !self.consume_comma() {
                break;
            }

            let exp = match self.parse_exp() {
                Some(exp) => exp,
                None => {
                    return Some(Err(Error {
                        index: self.index,
                        cause: ErrorCause::UnexpectedToken(self.input[self.index].clone()),
                    }))
                }
            };
            exp_list.push(exp);
        }

        if !self.consume_end_paren() {
            return Some(Err(Error {
                index: self.index,
                cause: ErrorCause::UnexpectedToken(self.input[self.index].clone()),
            }));
        }

        Some(Ok(Args { exp_list }))
    }

    fn parse_exp(&mut self) -> Option<Exp> {
        None.or_else(|| self.consume_nil().then(|| Exp::Nil))
            .or_else(|| self.consume_literal_string().map(Exp::LiteralString))
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

    fn consume_nil(&mut self) -> bool {
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::Nil) => {
                self.index += 1;
                true
            }
            _ => false,
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

    fn consume_eof(&mut self) -> bool {
        match self.input.get(self.index).map(|x| &x.kind) {
            Some(TokenKind::EOF) => {
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

            let expected = Some(Ok(FunctionCall {
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
            }));
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

            let expected = Some(Ok(Args {
                exp_list: vec![
                    Exp::LiteralString(LiteralString {
                        value: "hello".as_bytes().to_vec(),
                    }),
                    Exp::LiteralString(LiteralString {
                        value: "world".as_bytes().to_vec(),
                    }),
                ],
            }));
            assert_eq!(actual, expected);
        }

        #[test]
        fn parse_exp_literal_string() {
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

        #[test]
        fn parse_exp_nil() {
            let input = vec![Token::new(0, TokenKind::Nil)];
            let mut parser = Parser::new(&input);
            let actual = parser.parse_exp();
            let expected = Some(Exp::Nil);
            assert_eq!(actual, expected);
        }
    }
}
