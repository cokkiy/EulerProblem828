#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        arithmetic_operation::ArithmeticOperation, operand::Operand,
        two_number_results::TwoNumberResults,
    };

    #[test]
    fn two_number_results() {
        let left = Rc::new(Operand::Number(1));
        let right = Rc::new(Operand::Number(2));
        let results = TwoNumberResults::new(left.clone(), right.clone());
        assert_eq!(results.sum, Some(3));
        assert_eq!(results.difference, Some(-1));
        assert_eq!(results.product, Some(2));
        assert_eq!(results.quotient, None);
        assert_eq!(results.reverse_difference, Some(1));
        assert_eq!(results.reverse_quotient, Some(2));
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
        let results = TwoNumberResults::new(left.clone(), right.clone());
        assert_eq!(results.sum, Some(11));
        assert_eq!(results.difference, Some(-5));
        assert_eq!(results.product, Some(24));
        assert_eq!(results.quotient, None);
        assert_eq!(results.reverse_difference, Some(5));
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
