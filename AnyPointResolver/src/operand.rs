use std;
use std::rc::Rc;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use super::arithmetic_operation::ArithmeticOperation;

#[derive(Clone, Debug)]
pub(crate) enum Operand {
    Number(i32),
    Operation(Box<ArithmeticOperation>),
}

impl Operand {
    /// Converts the operand to an arithmetic expression.
    ///
    /// # Returns
    ///
    /// A string representation of the expression.
    pub fn to_expression(&self) -> String {
        match self {
            Operand::Number(number) => number.to_string(),
            Operand::Operation(operation) => operation.to_expression(),
        }
    }

    /// Retrieves the result of the operand.
    ///
    /// # Returns
    ///
    /// The result of the operand as an `i32`.
    ///
    /// # Remarks
    ///
    /// Returns the result of this [`Operand`].
    /// If [`Operand`] is [`Operand::Number`] returns the number,
    /// else return the [Operand::Operation]'s result.
    ///
    /// # Example
    ///
    /// ```
    /// use AnyPointResolver::operand::Operand;
    ///
    /// let operand = Operand::new(5);
    /// let result = operand.get_result();
    /// assert_eq!(result, 5);
    /// ```
    pub fn get_result(&self) -> i32 {
        match self {
            Operand::Number(number) => *number,
            Operand::Operation(operation) => operation.get_result(),
        }
    }
}

impl Add for &Operand {
    type Output = Operand;

    fn add(self, rhs: Self) -> Self::Output {
        Operand::Operation(Box::new(ArithmeticOperation::Addition(
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            self.get_result() + rhs.get_result(),
        )))
    }
}

impl Add for Operand {
    type Output = Operand;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Sub for &Operand {
    type Output = Operand;

    fn sub(self, rhs: Self) -> Self::Output {
        Operand::Operation(Box::new(ArithmeticOperation::Subtraction(
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            self.get_result() - rhs.get_result(),
        )))
    }
}

impl Sub for Operand {
    type Output = Operand;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Mul for &Operand {
    type Output = Operand;

    fn mul(self, rhs: Self) -> Self::Output {
        Operand::Operation(Box::new(ArithmeticOperation::Multiplication(
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            self.get_result() * rhs.get_result(),
        )))
    }
}

impl Mul for Operand {
    type Output = Operand;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Div for &Operand {
    type Output = Operand;

    fn div(self, rhs: Self) -> Self::Output {
        Operand::Operation(Box::new(ArithmeticOperation::Division(
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            self.get_result() / rhs.get_result(),
        )))
    }
}

impl Div for Operand {
    type Output = Operand;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl Rem for &Operand {
    type Output = Operand;

    fn rem(self, rhs: Self) -> Self::Output {
        Operand::Operation(Box::new(ArithmeticOperation::Modulo(
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            self.get_result() % rhs.get_result(),
        )))
    }
}

impl Rem for Operand {
    type Output = Operand;

    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        self.get_result() == other.get_result()
    }
}

impl PartialEq<i32> for Operand {
    fn eq(&self, other: &i32) -> bool {
        self.get_result() == *other
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
