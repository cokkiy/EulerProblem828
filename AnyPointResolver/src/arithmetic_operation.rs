use super::operand::Operand;
use std;
use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub(crate) enum ArithmeticOperation {
    Addition(Rc<Operand>, Rc<Operand>, i32),
    Subtraction(Rc<Operand>, Rc<Operand>, i32),
    Multiplication(Rc<Operand>, Rc<Operand>, i32),
    Division(Rc<Operand>, Rc<Operand>, i32),
}

impl ArithmeticOperation {
    pub(crate) fn to_tuple(&self) -> (Rc<Operand>, Rc<Operand>, i32) {
        match self {
            Self::Addition(left, right, result)
            | Self::Subtraction(left, right, result)
            | Self::Multiplication(left, right, result)
            | Self::Division(left, right, result) => (left.clone(), right.clone(), *result),
        }
    }

    pub(crate) fn get_result(&self) -> i32 {
        self.to_tuple().2
    }

    /// Returns `true` if the arithmetic operation is [`Addition`].
    ///
    /// [`Addition`]: ArithmeticOperation::Addition
    #[must_use]
    pub(crate) fn is_addition(&self) -> bool {
        matches!(self, Self::Addition(..))
    }

    /// Returns `true` if the arithmetic operation is [`Subtraction`].
    ///
    /// [`Subtraction`]: ArithmeticOperation::Subtraction
    #[must_use]
    pub(crate) fn is_subtraction(&self) -> bool {
        matches!(self, Self::Subtraction(..))
    }

    /// Returns `true` if the arithmetic operation is [`Multiplication`].
    ///
    /// [`Multiplication`]: ArithmeticOperation::Multiplication
    #[must_use]
    pub(crate) fn is_multiplication(&self) -> bool {
        matches!(self, Self::Multiplication(..))
    }

    /// Returns `true` if the arithmetic operation is [`Division`].
    ///
    /// [`Division`]: ArithmeticOperation::Division
    #[must_use]
    pub(crate) fn is_division(&self) -> bool {
        matches!(self, Self::Division(..))
    }
}

impl Display for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition(left, right, result) => {
                write!(f, "{} + {} = {}", left, right, result)
            }
            Self::Subtraction(left, right, result) => {
                let operation_str = format_operand(right);
                write!(f, "{} - {} = {}", left, operation_str, result)
            }
            Self::Multiplication(left, right, result) => {
                let l_operation_str = format_operand(left);
                let r_operation_str = format_operand(right);
                write!(f, "{} * {} = {}", l_operation_str, r_operation_str, result)
            }
            Self::Division(left, right, result) => {
                let l_operation_str = format_operand(left);
                let r_operation_str = format_operand(right);
                write!(f, "{} / {} = {}", l_operation_str, r_operation_str, result)
            }
        }
    }
}

pub(crate) fn format_operand(operand: &Rc<Operand>) -> String {
    match operand.as_ref() {
        Operand::Operation(operation) => {
            let operation_str = match operation.as_ref() {
                ArithmeticOperation::Addition(_, _, _)
                | ArithmeticOperation::Subtraction(_, _, _) => format!("({})", operation),
                _ => format!("{}", operation),
            };
            operation_str
        }
        _ => format!("{}", operand),
    }
}
