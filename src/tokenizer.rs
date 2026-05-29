use logos::Logos;

use crate::parser::BinaryOp;

pub struct ExpressionTokenizer<'a> {
    /// Store expression string
    expression: &'a str,
    // Store regex. Needed for lifetime solving
    // regex: Regex,
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
    // #[logos(error)]
    // Other,
}

impl<'a> ExpressionTokenizer<'a> {
    /// Create valid regex string
    fn create_regex_string() -> String {
        let regex_arr = [
            r"(\()",
            r"(\))",
            r"(,)",
            r"([-+*/^])",
            r"(\d+(\.\d+)?)",
            r"(\S)",
        ];

        let regex = regex_arr.join("|");

        regex
    }

    /// Split string from struct to tokens
    fn tokenize(&self) -> impl Iterator<Item = Result<Token, ()>> {
        Token::lexer(self.expression)
        // self.regex
        //     .find_iter(&self.expression)
        //     .map(|matched_str| Token::from(matched_str.as_str()))
    }

    /// Create new instance
    pub fn from(expression: &'a str) -> Self {
        // let regex = Regex::new(&Self::create_regex_string()).unwrap();
        Self { expression }
    }

    /// Tokenize and return either Ok with valid tokens for parser or an error
    pub fn tokenize_with_check(&self) -> Result<Vec<Token>, ()> {
        self.tokenize().collect::<Result<Vec<_>, ()>>()
    }
}

impl Token {
    /// Create new instance from the str
    fn from(str: &str) -> Self {
        match str {
            "(" => Self::LeftBracket,
            ")" => Self::RightBracket,
            "," => Self::Delimiter,
            operator @ ("+" | "-" | "/" | "*" | "^") => Self::Operator(match operator {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Sub,
                "/" => BinaryOp::Div,
                "*" => BinaryOp::Mul,
                "^" => BinaryOp::Pow,
                _ => unreachable!(),
            }),

            token @ _ => match token.parse::<f64>() {
                Ok(number) => Self::Number(number),
                Err(_) => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn create_regex() {
    //     let regex_string = ExpressionTokenizer::create_regex_string();
    //     let regex = Regex::new(&regex_string);

    //     dbg!(&regex);

    //     assert!(regex.is_ok())
    // }

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
            assert_eq!(
                tokenizer
                    .tokenize()
                    .map(|token| token.unwrap())
                    .collect::<Vec<_>>(),
                expect
            );
        }
    }

    #[test]
    fn token_from() {
        let strings_and_expected_tokens = [
            ("+", Token::Operator(BinaryOp::Add)),
            ("+", Token::Operator(BinaryOp::Add)),
            ("-", Token::Operator(BinaryOp::Sub)),
            ("/", Token::Operator(BinaryOp::Div)),
            ("*", Token::Operator(BinaryOp::Mul)),
            ("^", Token::Operator(BinaryOp::Pow)),
            (",", Token::Delimiter),
            ("(", Token::LeftBracket),
            (")", Token::RightBracket),
        ];

        for (string, token) in strings_and_expected_tokens {
            assert_eq!(Token::from(string), token);
        }

        let string_expected_numbers = [
            ("10", 10.0),
            ("0.0934", 0.0934),
            ("-32432.646", -32432.646),
            ("325.", 325.0),
            (".097996", 0.097996),
        ];

        for (string, token) in string_expected_numbers {
            assert_eq!(Token::from(string), Token::Number(token))
        }

        let invalid_chars = ["f", "324h54", "12.0f"];

        // for string in invalid_chars {
        //     assert_eq!(Token::from(string), Token::Other)
        // }
    }
}
