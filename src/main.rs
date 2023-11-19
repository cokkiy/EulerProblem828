use std::rc::Rc;

fn main() {
    println!("Start...");
    let mut results_steps: Vec<Vec<ArithmeticOperation>> = Vec::new();
    fork_calculation(
        58,
        &vec![
            Rc::new(Operand::Number(2)),
            Rc::new(Operand::Number(7)),
            Rc::new(Operand::Number(3)),
            Rc::new(Operand::Number(4)),
            Rc::new(Operand::Number(5)),
        ],
        &mut results_steps,
    );
    for (index, result) in results_steps.iter().enumerate() {
        println!("Result {}:", index + 1);
        for operation in result {
            println!("{}", operation);
        }
    }
}

mod arithmetic_operation;
mod operand;
use arithmetic_operation::ArithmeticOperation;
use operand::Operand;

fn fork_calculation(
    expected_result: i32,
    numbers: &Vec<Rc<Operand>>,
    results_steps: &mut Vec<Vec<ArithmeticOperation>>,
) {
    for i in 0..numbers.len() - 1 {
        let left = &numbers[i];
        for (pos, right) in numbers[(i + 1)..].iter().enumerate() {
            let remain_numbers: Vec<Rc<Operand>> = numbers
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != i && *index != pos)
                .map(|(_, number)| number.clone())
                .collect();

            let results: modname::TwoNumberResults =
                modname::TwoNumberResults::new(left.clone(), right.clone());
            if let Some(sum) = results.sum {
                let operation = ArithmeticOperation::Addition(left.clone(), right.clone(), sum);
                if expected_result == sum {
                    results_steps.push(vec![operation]);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(Operand::Operation(Box::new(operation))));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(difference) = results.difference {
                let operation =
                    ArithmeticOperation::Addition(left.clone(), right.clone(), difference);
                if expected_result == difference {
                    results_steps.push(vec![operation]);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(Operand::Operation(Box::new(operation))));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(product) = results.product {
                let operation = ArithmeticOperation::Addition(left.clone(), right.clone(), product);
                if expected_result == product {
                    results_steps.push(vec![operation]);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(Operand::Operation(Box::new(operation))));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(quotient) = results.quotient {
                let operation =
                    ArithmeticOperation::Addition(left.clone(), right.clone(), quotient);
                if expected_result == quotient {
                    results_steps.push(vec![operation]);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(Operand::Operation(Box::new(operation))));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(reverse_difference) = results.reverse_difference {
                let operation =
                    ArithmeticOperation::Addition(left.clone(), right.clone(), reverse_difference);
                if expected_result == reverse_difference {
                    results_steps.push(vec![operation]);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(Operand::Operation(Box::new(operation))));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(reverse_quotient) = results.reverse_quotient {
                let operation =
                    ArithmeticOperation::Addition(left.clone(), right.clone(), reverse_quotient);
                if expected_result == reverse_quotient {
                    results_steps.push(vec![operation]);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(Operand::Operation(Box::new(operation))));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
        }
    }
}

mod two_number_results;
