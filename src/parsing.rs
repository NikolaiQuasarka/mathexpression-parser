use regex::Regex;

pub struct ExpressionTokenizer {
    /// Store expression string
    expression: String,
    /// Store regex. Needed for lifetime solving
    regex: Regex,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Delimiter,
    Operator(OperatorType),
    Number(f64),
    Other,
}

#[derive(Debug, PartialEq)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl ExpressionTokenizer {
    // Create new instance
    pub fn from(expression: String) -> Self {
        let regex = Regex::new(&Self::create_regex_string()).unwrap();
        Self { expression, regex }
    }

    //Create valid regex string
    fn create_regex_string() -> String {
        let regex_arr = [
            r"(?P<left_bracket>\()",
            r"(?P<right_bracket>\))",
            r"(?P<delimiter>,)",
            r"(?P<operator>[-+*/^])",
            r"(?P<number>\d+(\.\d+)?)",
            r"(?P<unknown>\S)",
        ];

        let regex = regex_arr.join("|");

        regex
    }

    /// Split string from struct to tokens
    fn tokenize(&self) -> impl Iterator<Item = Token> {
        self.regex
            .find_iter(&self.expression)
            .map(|matched_str| Token::from(matched_str.as_str()))
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
                "+" => OperatorType::Add,
                "-" => OperatorType::Sub,
                "/" => OperatorType::Div,
                "*" => OperatorType::Mul,
                "^" => OperatorType::Pow,
                _ => unreachable!(),
            }),

            token @ _ => match token.parse::<f64>() {
                Ok(number) => Self::Number(number),
                Err(_) => Self::Other,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_regex() {
        let regex_string = ExpressionTokenizer::create_regex_string();
        let regex = Regex::new(&regex_string);

        dbg!(&regex);

        assert!(regex.is_ok())
    }

    #[test]
    fn tokenize_test() {
        let strings_and_expects = [(
            "3+3*(83)",
            [
                Token::Number(3.0),
                Token::Operator(OperatorType::Add),
                Token::Number(3.0),
                Token::Operator(OperatorType::Mul),
                Token::LeftBracket,
                Token::Number(83.0),
                Token::RightBracket,
            ],
        )];

        for (string, expect) in strings_and_expects {
            let tokenizer = ExpressionTokenizer::from(string.to_string());
            assert_eq!(tokenizer.tokenize().collect::<Vec<_>>(), expect);
        }
    }

    #[test]
    fn token_from() {
        let strings_and_expected_tokens = [
            ("+", Token::Operator(OperatorType::Add)),
            ("+", Token::Operator(OperatorType::Add)),
            ("-", Token::Operator(OperatorType::Sub)),
            ("/", Token::Operator(OperatorType::Div)),
            ("*", Token::Operator(OperatorType::Mul)),
            ("^", Token::Operator(OperatorType::Pow)),
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

        for string in invalid_chars {
            assert_eq!(Token::from(string), Token::Other)
        }
    }
}
