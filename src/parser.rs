use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
enum Expr {
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
enum UnaryOp {
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

        while precedence < get_precedence(self.get_current_token().expect("Token is empty")) {
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
            Token::Number(number) => unimplemented!(),
            _ => unimplemented!(),
        };
        unimplemented!()
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
                "4*5+5/6",
                Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(4.0)),
                        op: BinaryOp::Mul,
                        right: Box::new(Expr::Number(5.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(5.0)),
                        op: BinaryOp::Div,
                        right: Box::new(Expr::Number(6.0)),
                    }),
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
