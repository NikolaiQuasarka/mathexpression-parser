use regex::Regex;

pub struct ExpressionTokenizer {
    expression: String,
}

#[derive(Debug, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Delimiter,
    Operator(OperatorType),
    Number(f64),
    Other,
}

#[derive(Debug, PartialEq)]
enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl ExpressionTokenizer {
    pub fn from(expression: String) -> Self {
        Self { expression }
    }

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

    pub fn tokenize(self) -> Vec<String> {
        unimplemented!()
    }
}

impl Token {
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
