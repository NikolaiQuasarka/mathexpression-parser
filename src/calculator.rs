use crate::{
    parser::{BinaryOp, Expr, Parser, UnaryOp},
    tokenizer::ExpressionTokenizer,
};

pub fn calculate(str: &str) -> Result<f64, ()> {
    let tokens = ExpressionTokenizer::from(str).tokenize()?;

    let expr = Parser::new(tokens).parse()?;

    let result = calculate_expr(expr);

    Ok(result)
}

fn calculate_expr(expression: Expr) -> f64 {
    let result = match expression {
        Expr::Number(number) => number,
        Expr::Unary { op, right } => {
            let result = calculate_expr(*right);

            if let UnaryOp::Neg = op {
                return -result;
            }

            result
        }
        Expr::Binary { left, op, right } => {
            let left = calculate_expr(*left);
            let right = calculate_expr(*right);

            match op {
                BinaryOp::Add => left + right,
                BinaryOp::Sub => left - right,
                BinaryOp::Mul => left * right,
                BinaryOp::Div => left / right,
                BinaryOp::Pow => left.powf(right),
            }
        }
    };

    result
}

#[cfg(test)]
mod tests {
    use crate::tests::expressions_data::create_data_set;

    use super::*;

    #[test]
    fn calculate_test() {
        let data = create_data_set();

        for (str, _, result) in data {
            assert_eq!(calculate(str).unwrap(), result)
        }
    }

    #[test]
    fn calculate_expr_test() {
        let data = create_data_set();

        for (_, expr, result) in data {
            assert_eq!(calculate_expr(expr), result);
        }
    }
}
