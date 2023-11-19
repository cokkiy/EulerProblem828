use super::operand::Operand;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct TwoNumberResults {
    pub(crate) sum: Option<i32>,
    pub(crate) difference: Option<i32>,
    pub(crate) product: Option<i32>,
    pub(crate) quotient: Option<i32>,
    pub(crate) reverse_difference: Option<i32>,
    pub(crate) reverse_quotient: Option<i32>,
}

impl TwoNumberResults {
    pub(crate) fn new(left: Rc<Operand>, right: Rc<Operand>) -> TwoNumberResults {
        TwoNumberResults {
            sum: Some(left.as_ref() + right.as_ref()),
            difference: Some(left.as_ref() - right.as_ref()),
            product: Some(left.as_ref() * right.as_ref()),
            quotient: if *right != 0 && left.as_ref() % right.as_ref() == 0 {
                Some(left.as_ref() / right.as_ref())
            } else {
                None
            },
            reverse_difference: Some(right.as_ref() - left.as_ref()),
            reverse_quotient: if *left != 0 && right.as_ref() % left.as_ref() == 0 {
                Some(right.as_ref() / left.as_ref())
            } else {
                None
            },
        }
    }
}