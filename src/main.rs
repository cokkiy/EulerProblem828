use std::fmt::Display;

fn main() {
    println!("Start...");
    let mut results_steps: Vec<Vec<ArithmeticOperation>> = Vec::new();
    let steps: Vec<ArithmeticOperation> = Vec::with_capacity(5);
    fork_calculation(211, &vec![2, 3, 4, 6, 7, 25], &steps, &mut results_steps);
    for (index, result) in results_steps.iter().enumerate() {
        println!("Result {}:", index + 1);
        for step in result {
            println!("{}", step);
        }
    }
}

#[derive(Clone, Copy)]
enum ArithmeticOperation {
    Addition(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32),
}

impl Display for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition(left, right) => {
                write!(f, "{} + {} = {}", left, right, left + right)
            }
            Self::Subtraction(left, right) => {
                write!(f, "{} - {} = {}", left, right, left - right)
            }
            Self::Multiplication(left, right) => {
                write!(f, "{} * {} = {}", left, right, left * right)
            }
            Self::Division(left, right) => {
                write!(f, "{} / {} = {}", left, right, left / right)
            }
        }
    }
}

fn fork_calculation(
    expected_result: i32,
    numbers: &Vec<i32>,
    procedure_steps: &[ArithmeticOperation],
    results_steps: &mut Vec<Vec<ArithmeticOperation>>,
) {
    for i in 0..numbers.len() - 1 {
        let left = &numbers[i];
        for right in &numbers[(i + 1)..] {
            let remain_numbers: Vec<i32> = numbers
                .iter()
                .filter(|&x| x != left && x != right)
                .copied()
                .collect();

            let results: TwoNumberResults = TwoNumberResults::new(*left, *right);
            if let Some(sum) = results.sum {
                let mut procedure_steps = procedure_steps.to_vec();
                procedure_steps.push(ArithmeticOperation::Addition(*left, *right));
                if expected_result == sum {
                    results_steps.push(procedure_steps);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.push(sum);
                    fork_calculation(
                        expected_result,
                        &remain_numbers,
                        &procedure_steps,
                        results_steps,
                    );
                }
            }
            if let Some(difference) = results.difference {
                let mut procedure_steps = procedure_steps.to_vec();
                procedure_steps.push(ArithmeticOperation::Subtraction(*left, *right));
                if expected_result == difference {
                    results_steps.push(procedure_steps);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.push(difference);
                    fork_calculation(
                        expected_result,
                        &remain_numbers,
                        &procedure_steps,
                        results_steps,
                    );
                }
            }
            if let Some(product) = results.product {
                let mut procedure_steps = procedure_steps.to_vec();
                procedure_steps.push(ArithmeticOperation::Multiplication(*left, *right));
                if expected_result == product {
                    results_steps.push(procedure_steps);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.push(product);
                    fork_calculation(
                        expected_result,
                        &remain_numbers,
                        &procedure_steps,
                        results_steps,
                    );
                }
            }
            if let Some(quotient) = results.quotient {
                let mut procedure_steps = procedure_steps.to_vec();
                procedure_steps.push(ArithmeticOperation::Division(*left, *right));
                if expected_result == quotient {
                    results_steps.push(procedure_steps);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.push(quotient);
                    fork_calculation(
                        expected_result,
                        &remain_numbers,
                        &procedure_steps,
                        results_steps,
                    );
                }
            }
            if let Some(reverse_difference) = results.reverse_difference {
                let mut procedure_steps = procedure_steps.to_vec();
                procedure_steps.push(ArithmeticOperation::Subtraction(*right, *left));
                if expected_result == reverse_difference {
                    results_steps.push(procedure_steps);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.push(reverse_difference);
                    fork_calculation(
                        expected_result,
                        &remain_numbers,
                        &procedure_steps,
                        results_steps,
                    );
                }
            }
            if let Some(reverse_quotient) = results.reverse_quotient {
                let mut procedure_steps = procedure_steps.to_vec();
                procedure_steps.push(ArithmeticOperation::Division(*right, *left));
                if expected_result == reverse_quotient {
                    results_steps.push(procedure_steps);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.push(reverse_quotient);
                    fork_calculation(
                        expected_result,
                        &remain_numbers,
                        &procedure_steps,
                        results_steps,
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
struct TwoNumberResults {
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
            sum: Some(left + right),
            difference: Some(left - right),
            product: Some(left * right),
            quotient: if right != 0 && left % right == 0 {
                Some(left / right)
            } else {
                None
            },
            reverse_difference: Some(right - left),
            reverse_quotient: if left != 0 && right % left == 0 {
                Some(right / left)
            } else {
                None
            },
        }
    }
}
