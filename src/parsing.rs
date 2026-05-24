pub struct ExpressionTokenizer {
    expression: String,
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

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn create_regex() {
        let regex_string = ExpressionTokenizer::create_regex_string();
        let regex = Regex::new(&regex_string);

        dbg!(&regex);

        assert!(regex.is_ok())
    }
}
