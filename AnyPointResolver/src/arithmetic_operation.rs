use super::operand::Operand;
use std;
use std::{fmt::Display, rc::Rc};

#[derive(Clone, Debug)]
pub enum ArithmeticOperation {
    Addition(Rc<Operand>, Rc<Operand>, i32),
    Subtraction(Rc<Operand>, Rc<Operand>, i32),
    Multiplication(Rc<Operand>, Rc<Operand>, i32),
    Division(Rc<Operand>, Rc<Operand>, i32),
    Modulo(Rc<Operand>, Rc<Operand>, i32),
}

impl ArithmeticOperation {
    pub(crate) fn to_tuple(&self) -> (Rc<Operand>, Rc<Operand>, i32) {
        match self {
            Self::Addition(left, right, result)
            | Self::Subtraction(left, right, result)
            | Self::Multiplication(left, right, result)
            | Self::Division(left, right, result)
            | Self::Modulo(left, right, result) => (left.clone(), right.clone(), *result),
        }
    }

    fn format_operand_expression(operand: &Rc<Operand>, pre_is_division: bool) -> String {
        match operand.as_ref() {
            Operand::Operation(operation) => {
                let operation_str = match operation.as_ref() {
                    ArithmeticOperation::Addition(_, _, _)
                    | ArithmeticOperation::Subtraction(_, _, _) => {
                        format!("({})", operation.to_expression())
                    }
                    ArithmeticOperation::Division(_, _, _) if pre_is_division => {
                        format!("({})", operation.to_expression())
                    }
                    _ => operation.to_expression(),
                };
                operation_str
            }
            _ => operand.to_expression(),
        }
    }

    pub(crate) fn to_expression(&self) -> String {
        match self {
            Self::Addition(left, right, _) => {
                format!("{} + {}", left.to_expression(), right.to_expression())
            }
            Self::Subtraction(left, right, _) => {
                let operation_str = Self::format_operand_expression(right, false);
                format!("{} - {}", left.to_expression(), operation_str)
            }
            Self::Multiplication(left, right, _) => {
                let l_operation_str = Self::format_operand_expression(left, false);
                let r_operation_str = Self::format_operand_expression(right, false);
                format!("{} * {}", l_operation_str, r_operation_str)
            }
            Self::Division(left, right, _) => {
                let l_operation_str = Self::format_operand_expression(left, false);
                let r_operation_str = Self::format_operand_expression(right, true);
                format!("{} / {}", l_operation_str, r_operation_str)
            }
            Self::Modulo(left, right, _) => {
                let l_operation_str = Self::format_operand_expression(left, false);
                let r_operation_str = Self::format_operand_expression(right, true);
                format!("{} % {}", l_operation_str, r_operation_str)
            }
        }
    }

    pub(crate) fn get_result(&self) -> i32 {
        self.to_tuple().2
    }
}

impl Display for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition(left, right, result) => {
                write!(
                    f,
                    "{} + {} = {}",
                    left.to_expression(),
                    right.to_expression(),
                    result
                )
            }
            Self::Subtraction(left, right, result) => {
                let operation_str = Self::format_operand_expression(right, false);
                write!(
                    f,
                    "{} - {} = {}",
                    left.to_expression(),
                    operation_str,
                    result
                )
            }
            Self::Multiplication(left, right, result) => {
                let l_operation_str = Self::format_operand_expression(left, false);
                let r_operation_str = Self::format_operand_expression(right, false);
                write!(f, "{} * {} = {}", l_operation_str, r_operation_str, result)
            }
            Self::Division(left, right, result) => {
                let l_operation_str = Self::format_operand_expression(left, false);
                let r_operation_str = Self::format_operand_expression(right, true);
                write!(f, "{} / {} = {}", l_operation_str, r_operation_str, result)
            }

            Self::Modulo(left, right, result) => {
                let l_operation_str = Self::format_operand_expression(left, false);
                let r_operation_str = Self::format_operand_expression(right, true);
                write!(f, "{} % {} = {}", l_operation_str, r_operation_str, result)
            }
        }
    }
}
