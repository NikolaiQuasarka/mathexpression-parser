use crate::tokenizer::{OperatorType, Token};

fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Operator(operator) => match operator {
            OperatorType::Add => 1,
            OperatorType::Sub => 1,
            OperatorType::Mul => 2,
            OperatorType::Div => 2,
            OperatorType::Pow => 3,
        },
        _ => 0,
    }
}

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

#[derive(Debug, PartialEq)]
enum BinaryOp {
    Add,
    Sub,
    Mull,
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

impl Parser {
    fn get_current_token<'a>(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn parse_expression(&mut self, precedence: u8) {
        self.parsePrefix();

        while precedence < get_precedence(self.get_current_token().expect("Token is empty")) {
            self.parse_infix();
        }
    }

    fn consume(&mut self) -> Token {
        let index = self.index;
        self.index += 1;
        let token = self.tokens.get(index).expect("Token is empty");

        token.clone()
    }

    fn parse_infix(&mut self) {
        let token = self.consume();

        self.parse_expression(get_precedence(&token));

        todo!()
    }

    fn parsePrefix(&self) {
        unimplemented!()
    }

    pub fn parse(&mut self) -> Expr {
        unimplemented!()
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
        let strings_and_expectations = [(
            "3+2",
            Expr::Binary {
                left: Box::new(Expr::Number(3.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            },
        )];

        for (string, expect) in strings_and_expectations {
            let tokens = ExpressionTokenizer::from(string.to_string())
                .tokenize_with_check()
                .unwrap();

            let mut parser = Parser::new(tokens);

            assert_eq!(parser.parse(), expect)
        }
    }
}
