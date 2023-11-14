use std::iter;

fn main() {
    println!("Hello, world!");
}

struct TwoNumberResults {
    sum: Option<u32>,
    difference: Option<u32>,
    product: Option<u32>,
    quotient: Option<u32>,
    reverse_difference: Option<u32>,
    reverse_quotient: Option<u32>,
}

impl TwoNumberResults {
    fn new() -> TwoNumberResults {
        TwoNumberResults {
            sum: None,
            difference: None,
            product: None,
            quotient: None,
            reverse_difference: None,
            reverse_quotient: None,
        }
    }

    fn calculate(&mut self, left: u32, right: u32) {
        self.sum = Some(left + right);
        self.difference = Some(left - right);
        self.product = Some(left * right);
        self.quotient = if left % right == 0 {
            Some(left / right)
        } else {
            None
        };
        self.reverse_difference = Some(right - left);
        self.reverse_quotient = if right % left == 0 {
            Some(right / left)
        } else {
            None
        };
    }
}

struct Numbers {
    numbers: Vec<u32>,
}

impl Numbers {
    fn new(numbers: Vec<u32>) -> Numbers {
        Numbers { numbers }
    }
}

impl iter::Iterator for Numbers {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        if self.numbers.len() > 1 {
            let left = self.numbers.remove(0);
            for right in &self.numbers {
                result = Some((left, *right));
            }
        }
        result
    }
}
