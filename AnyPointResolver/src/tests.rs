#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{arithmetic_operation::ArithmeticOperation, operand::Operand, results::Results};

    #[test]
    fn operand_add() {
        let left = Operand::Number(2);
        let right = Operand::Number(4);
        let result = &left + &right;
        assert_eq!(format!("{}", result), "2 + 4 = 6");
    }

    #[test]
    fn operand_operation_add() {
        let left = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        )));
        let right = Operand::Number(2);
        let result = &left + &right;
        assert_eq!(format!("{}", result), "1 + 2 + 2 = 5");
    }

    #[test]
    fn division_after_division() {
        let left = Operand::Number(8);
        let mid = Operand::Number(4);
        let right = Operand::Number(2);
        let result = left.clone() / (&mid / &right);
        assert_eq!(format!("{}", result), "8 / (4 / 2) = 4")
    }

    #[test]
    fn division_after_mul() {
        let left = Operand::Number(8);
        let mid = Operand::Number(4);
        let right = Operand::Number(2);
        let result = left.clone() * (&mid / &right);
        assert_eq!(format!("{}", result), "8 * 4 / 2 = 16")
    }

    #[test]
    fn mul_after_division() {
        let left = Operand::Number(8);
        let mid = Operand::Number(4);
        let right = Operand::Number(2);
        let result = left.clone() / (&mid * &right);
        assert_eq!(format!("{}", result), "8 / 4 * 2 = 1")
    }

    #[test]
    fn mul_after_addition() {
        let left = Operand::Number(8);
        let mid = Operand::Number(4);
        let right = Operand::Number(2);
        let result = left.clone() + (&mid * &right);
        assert_eq!(format!("{}", result), "8 + 4 * 2 = 16")
    }

    #[test]
    fn add_after_mul() {
        let left = Operand::Number(8);
        let mid = Operand::Number(4);
        let right = Operand::Number(2);
        let result = left.clone() * (&mid + &right);
        assert_eq!(format!("{}", result), "8 * (4 + 2) = 48")
    }

    #[test]
    fn add_after_div() {
        let left = Operand::Number(8);
        let mid = Operand::Number(4);
        let right = Operand::Number(2);
        let result = left.clone() / (&mid + &right);
        assert_eq!(format!("{}", result), "8 / (4 + 2) = 1")
    }

    #[test]
    fn operand_sub() {
        let left = Operand::Number(2);
        let right = Operand::Number(4);
        let result = &left - &right;
        assert_eq!(format!("{}", result), "2 - 4 = -2");
    }

    #[test]
    fn operand_direct_sub() {
        let left = Operand::Number(2);
        let right = Operand::Number(4);
        let result = &left - &right - left.clone();
        assert_eq!(format!("{}", result), "2 - 4 - 2 = -4");
        assert_eq!(left, 2);
    }

    #[test]
    fn operand_operation_sub() {
        let left = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        )));
        let right = Operand::Number(2);
        let result = &left - &right;
        assert_eq!(format!("{}", result), "1 + 2 - 2 = 1");
    }

    #[test]
    fn operand_right_operation_sub() {
        let left = Operand::Number(2);
        let right = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        )));
        let result = &left - &right;
        assert_eq!(format!("{}", result), "2 - (1 + 2) = -1");
    }

    #[test]
    fn multi_step_operation() {
        let n1 = Operand::Number(1);
        let n2 = Operand::Number(2);
        let n3 = Operand::Number(3);
        let n4 = Operand::Number(4);
        let result = &(&n4 * &n3) - &(&n2 + &n1);
        assert_eq!(format!("{}", result), "4 * 3 - (2 + 1) = 9");
    }

    #[test]
    fn operation_add_display() {
        let operation = ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        );
        assert_eq!(format!("{}", operation), "1 + 2 = 3");
    }

    #[test]
    fn operation_sub_display() {
        let operation = ArithmeticOperation::Subtraction(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            -1,
        );
        assert_eq!(format!("{}", operation), "1 - 2 = -1");
    }

    #[test]
    fn operation_mul_display() {
        let operation = ArithmeticOperation::Multiplication(
            Rc::new(Operand::Number(3)),
            Rc::new(Operand::Number(2)),
            6,
        );
        assert_eq!(format!("{}", operation), "3 * 2 = 6");
    }

    #[test]
    fn operation_div_display() {
        let operation = ArithmeticOperation::Division(
            Rc::new(Operand::Number(6)),
            Rc::new(Operand::Number(2)),
            3,
        );
        assert_eq!(format!("{}", operation), "6 / 2 = 3");
    }

    #[test]
    fn operand_number_display() {
        let operand = Operand::Number(1);
        assert_eq!(format!("{}", operand), "1");
    }

    #[test]
    fn operand_operation_display() {
        let operand = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        )));
        assert_eq!(format!("{}", operand), "1 + 2 = 3");
    }

    #[test]
    fn operand_number_to_expression() {
        let operand = Operand::Number(10);
        assert_eq!(operand.to_expression(), "10");
    }

    #[test]
    fn operand_operation_to_expression() {
        let operand = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(4)),
            5,
        )));
        assert_eq!(operand.to_expression(), "1 + 4");
    }

    #[test]
    fn operand_inner_left_operation_to_expression() {
        let left = Rc::new(Operand::Operation(Box::new(
            ArithmeticOperation::Multiplication(
                Rc::new(Operand::Number(2)),
                Rc::new(Operand::Number(3)),
                6,
            ),
        )));
        let operand = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            left,
            Rc::new(Operand::Number(4)),
            10,
        )));
        assert_eq!(operand.to_expression(), "2 * 3 + 4");
    }

    #[test]
    fn operand_inner_right_operation_to_expression() {
        let right = Rc::new(Operand::Operation(Box::new(ArithmeticOperation::Division(
            Rc::new(Operand::Number(6)),
            Rc::new(Operand::Number(3)),
            2,
        ))));
        let operand = Operand::Operation(Box::new(ArithmeticOperation::Subtraction(
            Rc::new(Operand::Number(4)),
            right,
            10,
        )));
        assert_eq!(operand.to_expression(), "4 - 6 / 3");
    }

    #[test]
    fn operand_inner_2_operation_to_expression() {
        let left = Rc::new(Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(6)),
            Rc::new(Operand::Number(3)),
            9,
        ))));

        let right = Rc::new(Operand::Operation(Box::new(
            ArithmeticOperation::Multiplication(
                Rc::new(Operand::Number(6)),
                Rc::new(Operand::Number(3)),
                18,
            ),
        )));
        let operand = Operand::Operation(Box::new(ArithmeticOperation::Multiplication(
            left, right, 162,
        )));
        assert_eq!(operand.to_expression(), "(6 + 3) * 6 * 3");
        assert_eq!(format!("{}", operand), "(6 + 3) * 6 * 3 = 162");
    }

    #[test]
    fn operand_add_number_display() {
        let operand = Rc::new(Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        ))));

        let num = Rc::new(Operand::Number(4));

        let result = ArithmeticOperation::Addition(operand, num, 7);
        assert_eq!(format!("{}", result), "1 + 2 + 4 = 7");
    }

    #[test]
    fn two_number_results() {
        let left = Rc::new(Operand::Number(1));
        let right = Rc::new(Operand::Number(2));
        let results = Results::new(left.clone(), right.clone());
        assert_eq!(results.sum, Some(Operand::Number(3)));
        assert_eq!(results.difference.unwrap().get_result(), -1);
        assert_eq!(results.product.unwrap(), 2);
        assert_eq!(results.quotient, None);
        assert_eq!(results.reverse_difference.unwrap().get_result(), 1);
        assert_eq!(results.reverse_quotient.unwrap(), 2);
    }

    #[test]
    fn two_operation_results() {
        let left = Rc::new(Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        ))));

        let right = Rc::new(Operand::Operation(Box::new(
            ArithmeticOperation::Multiplication(
                Rc::new(Operand::Number(2)),
                Rc::new(Operand::Number(4)),
                8,
            ),
        )));
        let results = Results::new(left.clone(), right.clone());
        assert_eq!(results.sum, Some(Operand::Number(11)));
        assert_eq!(results.difference.unwrap().get_result(), -5);
        assert_eq!(results.product.unwrap(), 24);
        assert_eq!(results.quotient, None);
        assert_eq!(results.reverse_difference.unwrap().get_result(), 5);
        assert_eq!(results.reverse_quotient, None);
    }

    #[test]
    fn num_operation_results() {
        let left = Rc::new(Operand::Number(16));

        let right = Rc::new(Operand::Operation(Box::new(
            ArithmeticOperation::Multiplication(
                Rc::new(Operand::Number(2)),
                Rc::new(Operand::Number(4)),
                8,
            ),
        )));
        let results = Results::new(left.clone(), right.clone());
        assert_eq!(results.sum, Some(Operand::Number(24)));
        assert_eq!(results.difference.unwrap().get_result(), 8);
        assert_eq!(results.product.unwrap(), 128);
        assert_eq!(results.quotient.unwrap(), 2);
        assert_eq!(results.reverse_difference.unwrap().get_result(), -8);
        assert_eq!(results.reverse_quotient, None);
    }

    #[test]
    fn operation_num_results() {
        let left = Rc::new(Operand::Operation(Box::new(ArithmeticOperation::Division(
            Rc::new(Operand::Number(8)),
            Rc::new(Operand::Number(2)),
            4,
        ))));

        let right = Rc::new(Operand::Number(3));
        let results = Results::new(left.clone(), right.clone());
        assert_eq!(results.sum.unwrap().get_result(), 7);
        assert_eq!(results.difference.unwrap().get_result(), 1);
        assert_eq!(results.product.unwrap(), 12);
        assert_eq!(results.quotient, None);
        assert_eq!(results.reverse_difference.unwrap().get_result(), -1);
        assert_eq!(results.reverse_quotient, None);
    }

    #[test]
    fn left_operation_right_num_add() {
        let left = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        )));
        let right = Rc::new(Operand::Number(4));
        let result = &left + &right;
        assert_eq!(result, 7);
    }

    #[test]
    fn left_number_right_operation_add() {
        let left = Rc::new(Operand::Number(2));
        let right = Operand::Operation(Box::new(ArithmeticOperation::Multiplication(
            Rc::new(Operand::Number(2)),
            Rc::new(Operand::Number(3)),
            6,
        )));
        let result = left.as_ref() + &right;
        assert_eq!(result, 8);
    }

    #[test]
    fn left_operation_right_operation_add() {
        let left = Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(Operand::Number(1)),
            Rc::new(Operand::Number(2)),
            3,
        )));
        let right = Operand::Operation(Box::new(ArithmeticOperation::Multiplication(
            Rc::new(Operand::Number(2)),
            Rc::new(Operand::Number(3)),
            6,
        )));
        let result = &left + &right;
        assert_eq!(result, 9);
    }

    #[test]
    fn two_number_add() {
        let left = Operand::Number(1);
        let right = Operand::Number(2);
        let result = &left + &right;
        assert_eq!(result, 3);
    }

    #[test]
    fn two_number_sub() {
        let left = Operand::Number(1);
        let right = Operand::Number(2);
        let result = &left - &right;
        assert_eq!(result, -1);
    }

    #[test]
    fn two_number_mul() {
        let left = Operand::Number(3);
        let right = Operand::Number(2);
        let result = &left * &right;
        assert_eq!(result, 6);
    }

    #[test]
    fn two_number_div() {
        let left = Operand::Number(6);
        let right = Operand::Number(2);
        let result = &left / &right;
        assert_eq!(result, 3);
    }
}
