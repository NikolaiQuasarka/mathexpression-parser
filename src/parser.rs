use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Plus,
    Neg,
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Operator(operator) => match operator {
            BinaryOp::Add => 1,
            BinaryOp::Sub => 1,
            BinaryOp::Mul => 2,
            BinaryOp::Div => 2,
            BinaryOp::Pow => 3,
        },
        _ => 0,
    }
}

impl Parser {
    fn get_current_token<'a>(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn consume(&mut self) -> &Token {
        let index = self.index;
        self.index += 1;
        let token = self.tokens.get(index).expect("Token is empty");

        token
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expr, ()> {
        let prefix = self.parse_prefix()?;

        let mut expr = prefix;

        while let Some(token) = self.get_current_token() {
            let precedence = get_precedence(token);

            while precedence < precedence {
                let infix = self.parse_infix()?;

                let Token::Operator(operator) = infix.clone() else {
                    return Err(());
                };

                let precedence = get_precedence(infix);

                let right = self.parse_expression(precedence)?;

                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: operator,
                    right: Box::new(right),
                }
            }
        }

        Ok(expr)
    }

    fn parse_infix(&mut self) -> Result<&Token, ()> {
        let token = match self.consume() {
            token @ Token::Operator(_) => token,
            _ => return Err(()),
        };

        Ok(token)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ()> {
        let token = match self.consume() {
            Token::Number(number) => Expr::Number(*number),
            _ => return Err(()),
        };

        Ok(token)
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        self.parse_expression(0)
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::ExpressionTokenizer;

    use super::*;

    #[test]
    fn parse() {
        let strings_and_expectations = [
            (
                "3+2",
                Expr::Binary {
                    left: Box::new(Expr::Number(3.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(2.0)),
                },
            ),
            (
                "7-3+2",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(7.0)),
                        op: BinaryOp::Sub,
                        right: Box::new(Expr::Number(3.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(2.0)),
                },
            ),
            (
                "4*5+6/2",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(4.0)),
                        op: BinaryOp::Mul,
                        right: Box::new(Expr::Number(5.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(6.0)),
                        op: BinaryOp::Div,
                        right: Box::new(Expr::Number(2.0)),
                    }),
                },
            ),
            (
                "10+5-3*2/4",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(10.0)),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Binary {
                            left: Box::new(Expr::Number(5.0)),
                            op: BinaryOp::Sub,
                            right: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(3.0)),
                                    op: BinaryOp::Mul,
                                    right: Box::new(Expr::Number(2.0)),
                                }),
                                op: BinaryOp::Div,
                                right: Box::new(Expr::Number(4.0)),
                            }),
                        }),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                },
            ),
            (
                "8*3+2-5/1+4",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(8.0)),
                                    op: BinaryOp::Mul,
                                    right: Box::new(Expr::Number(3.0)),
                                }),
                                op: BinaryOp::Add,
                                right: Box::new(Expr::Number(2.0)),
                            }),
                            op: BinaryOp::Sub,
                            right: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(5.0)),
                                op: BinaryOp::Div,
                                right: Box::new(Expr::Number(1.0)),
                            }),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Number(4.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                },
            ),
            (
                "2+3*4-5/6+7*8-9",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Number(2.0)),
                                        op: BinaryOp::Add,
                                        right: Box::new(Expr::Binary {
                                            left: Box::new(Expr::Number(3.0)),
                                            op: BinaryOp::Mul,
                                            right: Box::new(Expr::Number(4.0)),
                                        }),
                                    }),
                                    op: BinaryOp::Sub,
                                    right: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Number(5.0)),
                                        op: BinaryOp::Div,
                                        right: Box::new(Expr::Number(6.0)),
                                    }),
                                }),
                                op: BinaryOp::Add,
                                right: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(7.0)),
                                    op: BinaryOp::Mul,
                                    right: Box::new(Expr::Number(8.0)),
                                }),
                            }),
                            op: BinaryOp::Sub,
                            right: Box::new(Expr::Number(9.0)),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Number(0.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                },
            ),
            (
                "1+2*3+4*5+6*7+8*9",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(1.0)),
                                    op: BinaryOp::Add,
                                    right: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Number(2.0)),
                                        op: BinaryOp::Mul,
                                        right: Box::new(Expr::Number(3.0)),
                                    }),
                                }),
                                op: BinaryOp::Add,
                                right: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(4.0)),
                                    op: BinaryOp::Mul,
                                    right: Box::new(Expr::Number(5.0)),
                                }),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(6.0)),
                                op: BinaryOp::Mul,
                                right: Box::new(Expr::Number(7.0)),
                            }),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Binary {
                            left: Box::new(Expr::Number(8.0)),
                            op: BinaryOp::Mul,
                            right: Box::new(Expr::Number(9.0)),
                        }),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                },
            ),
            (
                "10-2*3+4/2-5+6*7-8/4+9",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Binary {
                                            left: Box::new(Expr::Binary {
                                                left: Box::new(Expr::Number(10.0)),
                                                op: BinaryOp::Sub,
                                                right: Box::new(Expr::Binary {
                                                    left: Box::new(Expr::Number(2.0)),
                                                    op: BinaryOp::Mul,
                                                    right: Box::new(Expr::Number(3.0)),
                                                }),
                                            }),
                                            op: BinaryOp::Add,
                                            right: Box::new(Expr::Binary {
                                                left: Box::new(Expr::Number(4.0)),
                                                op: BinaryOp::Div,
                                                right: Box::new(Expr::Number(2.0)),
                                            }),
                                        }),
                                        op: BinaryOp::Sub,
                                        right: Box::new(Expr::Number(5.0)),
                                    }),
                                    op: BinaryOp::Add,
                                    right: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Number(6.0)),
                                        op: BinaryOp::Mul,
                                        right: Box::new(Expr::Number(7.0)),
                                    }),
                                }),
                                op: BinaryOp::Sub,
                                right: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(8.0)),
                                    op: BinaryOp::Div,
                                    right: Box::new(Expr::Number(4.0)),
                                }),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Number(9.0)),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Number(0.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                },
            ),
        ];

        for (string, expect) in strings_and_expectations {
            let tokens = ExpressionTokenizer::from(string.to_string())
                .tokenize_with_check()
                .unwrap();

            let mut parser = Parser::new(tokens);

            assert_eq!(parser.parse().unwrap(), expect)
        }
    }
}
