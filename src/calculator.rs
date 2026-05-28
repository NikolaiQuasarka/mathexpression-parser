use crate::parser::{BinaryOp, Expr, UnaryOp};

pub fn calculate(expression: Expr) -> f64 {
    let result = match expression {
        Expr::Number(number) => number,
        Expr::Unary { op, right } => {
            let result = calculate(*right);

            if let UnaryOp::Neg = op {
                return -result;
            }

            result
        }
        Expr::Binary { left, op, right } => {
            let left = calculate(*left);
            let right = calculate(*right);

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

        for (_, expr, result) in data {
            assert_eq!(calculate(expr), result);
        }
    }
}
