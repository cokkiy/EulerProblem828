fn main() {
    println!("Hello, world!");
    fork_calculation(&vec![1, 2, 3, 4]);
}

fn fork_calculation(numbers: &Vec<i32>) {
    for i in 0..numbers.len() - 1 {
        let left = numbers[i];
        for right in &numbers[(i + 1)..] {
            let results = TwoNumberResults::new(left, *right);
            if let Some(sum) = results.sum {
                println!("{} + {} = {}", left, right, sum);
            }
            if let Some(difference) = results.difference {
                println!("{} - {} = {}", left, right, difference);
            }
            if let Some(product) = results.product {
                println!("{} * {} = {}", left, right, product);
            }
            if let Some(quotient) = results.quotient {
                println!("{} / {} = {}", left, right, quotient);
            }
            if let Some(reverse_difference) = results.reverse_difference {
                println!("{} - {} = {}", right, left, reverse_difference);
            }
            if let Some(reverse_quotient) = results.reverse_quotient {
                println!("{} / {} = {}", right, left, reverse_quotient);
            }
        }
    }
}

#[derive(Debug)]
struct TwoNumberResults {
    left: i32,
    right: i32,
    sum: Option<i32>,
    difference: Option<i32>,
    product: Option<i32>,
    quotient: Option<i32>,
    reverse_difference: Option<i32>,
    reverse_quotient: Option<i32>,
}

impl TwoNumberResults {
    fn new(left: i32, right: i32) -> TwoNumberResults {
        TwoNumberResults {
            left,
            right,
            sum: Some(left + right),
            difference: Some(left - right),
            product: Some(left * right),
            quotient: if left % right == 0 {
                Some(left / right)
            } else {
                None
            },
            reverse_difference: Some(right - left),
            reverse_quotient: if right % left == 0 {
                Some(right / left)
            } else {
                None
            },
        }
    }
}
