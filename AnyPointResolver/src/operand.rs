use std;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use super::arithmetic_operation::ArithmeticOperation;

#[derive(Clone)]
pub(crate) enum Operand {
    Number(i32),
    Operation(Box<ArithmeticOperation>),
}

impl Add for &Operand {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Number(left), Operand::Number(right)) => left + right,
            (Operand::Number(left), Operand::Operation(operation)) => operation.get_result() + left,
            (Operand::Operation(operation), Operand::Number(right)) => {
                operation.get_result() + right
            }
            (Operand::Operation(left), Operand::Operation(right)) => {
                left.get_result() + right.get_result()
            }
        }
    }
}

impl Sub for &Operand {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Number(left), Operand::Number(right)) => left - right,
            (Operand::Number(left), Operand::Operation(operation)) => left - operation.get_result(),
            (Operand::Operation(operation), Operand::Number(right)) => {
                operation.get_result() - right
            }
            (Operand::Operation(left), Operand::Operation(right)) => {
                left.get_result() - right.get_result()
            }
        }
    }
}

impl Mul for &Operand {
    type Output = i32;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Number(left), Operand::Number(right)) => left * right,
            (Operand::Number(left), Operand::Operation(operation)) => left * operation.get_result(),
            (Operand::Operation(operation), Operand::Number(right)) => {
                operation.get_result() * right
            }
            (Operand::Operation(left), Operand::Operation(right)) => {
                left.get_result() * right.get_result()
            }
        }
    }
}

impl Div for &Operand {
    type Output = i32;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Number(left), Operand::Number(right)) => left / right,
            (Operand::Number(left), Operand::Operation(operation)) => left / operation.get_result(),
            (Operand::Operation(operation), Operand::Number(right)) => {
                operation.get_result() / right
            }
            (Operand::Operation(left), Operand::Operation(right)) => {
                left.get_result() / right.get_result()
            }
        }
    }
}

impl Rem for &Operand {
    type Output = i32;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Number(left), Operand::Number(right)) => left % right,
            (Operand::Number(left), Operand::Operation(operation)) => left % operation.get_result(),
            (Operand::Operation(operation), Operand::Number(right)) => {
                operation.get_result() % right
            }
            (Operand::Operation(left), Operand::Operation(right)) => {
                left.get_result() % right.get_result()
            }
        }
    }
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Number(left), Self::Operation(operation)) => *left == operation.get_result(),
            (Self::Operation(operation), Self::Number(right)) => operation.get_result() == *right,
            (Self::Operation(left), Self::Operation(right)) => {
                left.get_result() == right.get_result()
            }
        }
    }
}

impl PartialEq<i32> for Operand {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Number(number) => number == other,
            Self::Operation(operation) => operation.get_result() == *other,
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => {
                write!(f, "{}", number)
            }
            Self::Operation(operation) => {
                write!(f, "{}", operation)
            }
        }
    }
}
