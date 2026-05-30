use logos::Logos;

use crate::parser::BinaryOp;

pub struct ExpressionTokenizer<'a> {
    /// Store expression string
    expression: &'a str,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s")]
pub enum Token {
    #[token(r"(")]
    LeftBracket,
    #[token(r")")]
    RightBracket,
    #[token(",")]
    Delimiter,
    #[regex(r"([-+*/^])", |token| {
        let token = token.slice();

        match token {
            "-"=> BinaryOp::Sub,
            "+"=> BinaryOp::Add,
            "*"=> BinaryOp::Mul,
            "/"=> BinaryOp::Div,
            "^"=> BinaryOp::Pow,
            _=>unreachable!()
        }
    } )]
    Operator(BinaryOp),
    #[regex(r"(\d+(\.\d+)?)", |lex| {
        lex.slice().parse::<f64>().unwrap()
    })]
    Number(f64),
}

impl<'a> ExpressionTokenizer<'a> {
    /// Create new instance
    pub fn from(expression: &'a str) -> Self {
        Self { expression }
    }

    /// Tokenize and return either Ok with valid tokens for parser or an error
    pub fn tokenize(&self) -> Result<Vec<Token>, ()> {
        Token::lexer(self.expression).collect::<Result<Vec<_>, ()>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_test() {
        let strings_and_expects = [(
            "3+3*(83)",
            [
                Token::Number(3.0),
                Token::Operator(BinaryOp::Add),
                Token::Number(3.0),
                Token::Operator(BinaryOp::Mul),
                Token::LeftBracket,
                Token::Number(83.0),
                Token::RightBracket,
            ],
        )];

        for (str, expect) in strings_and_expects {
            let tokenizer = ExpressionTokenizer::from(str);
            assert_eq!(tokenizer.tokenize().unwrap(), expect);
        }
    }
}
