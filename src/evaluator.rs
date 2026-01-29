use crate::parser::{AstNodeType, BinaryOperator};

pub fn evaluate(node: &AstNodeType) -> i64 {
    match node {
        AstNodeType::NumericLiteral { value } =>{
            println!("Numericliteral: {}", value);
            *value
        },

        AstNodeType::BinaryExpression { left, operator, right } => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);

            match operator {
                BinaryOperator::Add => left_val + right_val,
                BinaryOperator::Subtract => left_val - right_val,
                BinaryOperator::Multiply => left_val * right_val,
                BinaryOperator::Divide => {
                    if right_val == 0 {
                        panic!("division by zero");
                    }
                    left_val / right_val
                }
                BinaryOperator::Modulo => {
                    if right_val == 0 {
                        panic!("modulo by zero");
                    }
                    left_val % right_val
                }}
        }

        AstNodeType::Program { body } => {
            let mut result = 0;
            for expr in body {
                result = evaluate(expr);
            }
            result
        }

        _ => panic!("cannot evaluate {:?}", node),
    }
}