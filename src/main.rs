use std::{collections::HashMap, fmt::Display};

fn main() {
    println!("Start...");
    let mut results_steps: Vec<Vec<ArithmeticOperation>> = Vec::new();
    let steps: Vec<ArithmeticOperation> = Vec::with_capacity(5);
    fork_calculation(58, &vec![2, 7, 3, 4, 5], &steps, &mut results_steps);
    for (index, result) in results_steps.iter().enumerate() {
        let steps = ArithmeticSteps::from(result);
        println!(
            "{}): {} = 58  Score: {}",
            index + 1,
            steps,
            steps.get_score()
        );

        for step in result {
            println!("     {}", step);
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

struct ArithmeticSteps {
    steps: Vec<ArithmeticOperation>,
}

impl ArithmeticSteps {
    fn from(steps: &Vec<ArithmeticOperation>) -> ArithmeticSteps {
        ArithmeticSteps {
            steps: steps.to_vec(),
        }
    }

    fn get_used_numbers(&self) -> Vec<i32> {
        let mut used_numbers: Vec<i32> = Vec::new();
        let mut mid_results: Vec<i32> = Vec::with_capacity(5);

        fn find_used_numbers(
            mid_results: &mut Vec<i32>,
            used_numbers: &mut Vec<i32>,
            left: &i32,
            right: &i32,
        ) {
            if let Some(pos) = mid_results.iter().position(|&x| x == *left) {
                mid_results.remove(pos);
            } else {
                used_numbers.push(*left);
            }

            if let Some(pos) = mid_results.iter().position(|&x| x == *right) {
                mid_results.remove(pos);
            } else {
                used_numbers.push(*right);
            }
        }

        for step in &self.steps {
            match step {
                ArithmeticOperation::Addition(left, right) => {
                    find_used_numbers(&mut mid_results, &mut used_numbers, left, right);
                    mid_results.push(*left + *right);
                }
                ArithmeticOperation::Subtraction(left, right) => {
                    find_used_numbers(&mut mid_results, &mut used_numbers, left, right);
                    mid_results.push(*left - *right);
                }
                ArithmeticOperation::Multiplication(left, right) => {
                    find_used_numbers(&mut mid_results, &mut used_numbers, left, right);
                    mid_results.push(*left * *right);
                }
                ArithmeticOperation::Division(left, right) => {
                    find_used_numbers(&mut mid_results, &mut used_numbers, left, right);
                    mid_results.push(*left / *right);
                }
            }
        }
        used_numbers
    }

    fn get_score(&self) -> i32 {
        let used_numbers = self.get_used_numbers();
        used_numbers.iter().sum()
    }
}

impl Display for ArithmeticSteps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn replace_by_mid_value(
            mid_results: &mut HashMap<i32, String>,
            left: &i32,
            right: &i32,
            mid_value: i32,
            op: &str,
        ) {
            if mid_results.contains_key(left) && mid_results.contains_key(right) {
                let ls = mid_results.remove_entry(left).unwrap().1;
                let rs = mid_results.remove_entry(right).unwrap().1;
                if op == "*" || op == "/" || op == "-" {
                    mid_results.insert(mid_value, format!("({}) {} ({})", ls, op, rs));
                } else {
                    mid_results.insert(mid_value, format!("{} {} {}", ls, op, rs));
                }
            } else if mid_results.contains_key(left) {
                let ls = mid_results.remove_entry(left).unwrap().1;
                if op == "*" || op == "/" {
                    mid_results.insert(mid_value, format!("({}) {} {}", ls, op, right));
                } else {
                    mid_results.insert(mid_value, format!("{} {} {}", ls, op, right));
                }
            } else if mid_results.contains_key(right) {
                let rs = mid_results.remove_entry(right).unwrap().1;
                if op == "*" || op == "/" || op == "-" {
                    mid_results.insert(mid_value, format!("{} {} ({})", left, op, rs));
                } else {
                    mid_results.insert(mid_value, format!("{} {} {}", left, op, rs));
                }
            } else {
                mid_results.insert(mid_value, format!("{} {} {}", left, op, right));
            }
        }

        let mut mid_results: HashMap<i32, String> = HashMap::new();
        for step in &self.steps {
            match step {
                ArithmeticOperation::Addition(left, right) => {
                    let mid_value = *left + *right;
                    replace_by_mid_value(&mut mid_results, left, right, mid_value, "+");
                }
                ArithmeticOperation::Subtraction(left, right) => {
                    let mid_value = *left - *right;
                    replace_by_mid_value(&mut mid_results, left, right, mid_value, "-");
                }
                ArithmeticOperation::Multiplication(left, right) => {
                    let mid_value = *left * *right;
                    replace_by_mid_value(&mut mid_results, left, right, mid_value, "*");
                }
                ArithmeticOperation::Division(left, right) => {
                    let mid_value = *left / *right;
                    replace_by_mid_value(&mut mid_results, left, right, mid_value, "/");
                }
            }
        }
        if mid_results.is_empty() {
            write!(f, "No steps")
        } else {
            let mut steps = String::new();
            for (_, value) in mid_results {
                steps.push_str(&value);
            }
            write!(f, "{}", steps)
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
