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

pub struct Parser {
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
    fn get_current_token(&self) -> Option<&Token> {
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

        while let Some(token) = self.get_current_token()
            && precedence < get_precedence(token)
        {
            let (token, infix_expr) = self.parse_infix()?;

            let Token::Operator(operator) = token.clone() else {
                return Err(());
            };

            // let precedence = get_precedence(infix);

            // let right = self.parse_expression(precedence)?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(infix_expr),
            }
        }

        Ok(expr)
    }

    fn parse_infix(&mut self) -> Result<(Token, Expr), ()> {
        let token = self.consume().clone();

        let precedence = get_precedence(&token);

        let expr = match token {
            Token::Operator(_) => self.parse_expression(precedence),
            _ => return Err(()),
        }?;

        Ok((token, expr))
    }

    fn parse_prefix(&mut self) -> Result<Expr, ()> {
        let token = match self.get_current_token().ok_or(())? {
            Token::Number(_) => self.parse_number(),
            Token::LeftBracket => self.parse_parenthesized_expression(),
            Token::Operator(_) => self.parse_unary_expression(),
            _ => return Err(()),
        }?;

        Ok(token)
    }

    fn parse_number(&mut self) -> Result<Expr, ()> {
        let Token::Number(number) = self.consume() else {
            return Err(());
        };

        Ok(Expr::Number(*number))
    }

    fn parse_unary_expression(&mut self) -> Result<Expr, ()> {
        match self.consume() {
            Token::Operator(binary_op) => match binary_op {
                BinaryOp::Add => {
                    let token = self.get_current_token().ok_or(())?;
                    let expr = match token {
                        Token::LeftBracket => self.parse_parenthesized_expression(),
                        Token::Number(_) => self.parse_number(),
                        _ => return Err(()),
                    }?;

                    Ok(expr)
                }
                BinaryOp::Sub => {
                    let token = self.get_current_token().ok_or(())?;
                    let expr = match token {
                        Token::LeftBracket => self.parse_parenthesized_expression(),
                        Token::Number(_) => self.parse_number(),
                        _ => return Err(()),
                    }?;

                    Ok(Expr::Unary {
                        op: UnaryOp::Neg,
                        right: Box::new(expr),
                    })
                }
                _ => return Err(()),
            },
            _ => return Err(()),
        }
    }

    fn parse_parenthesized_expression(&mut self) -> Result<Expr, ()> {
        let Token::LeftBracket = self.consume() else {
            return Err(());
        };

        let expression = self.parse_expression(0);

        let Token::RightBracket = self.consume() else {
            return Err(());
        };

        expression
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
    use crate::{tests::expressions_data::create_data_set, tokenizer::ExpressionTokenizer};

    use super::*;

    #[test]
    fn parse() {
        let data = create_data_set();

        for (string, expect, _) in data {
            let tokens = ExpressionTokenizer::from(string.to_string())
                .tokenize_with_check()
                .unwrap();

            let mut parser = Parser::new(tokens);

            assert_eq!(parser.parse().unwrap(), expect)
        }
    }
}
