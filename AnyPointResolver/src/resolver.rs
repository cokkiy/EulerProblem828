use std::rc::Rc;

use crate::{operand::Operand, results::Results};

pub fn fork_calculation(
    expected_result: i32,
    numbers: &Vec<Rc<Operand>>,
    results_steps: &mut Vec<Operand>,
) {
    for i in 0..numbers.len() - 1 {
        let left = &numbers[i];
        for (pos, right) in numbers[(i + 1)..].iter().enumerate() {
            let remain_numbers: Vec<Rc<Operand>> = numbers
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != i && *index != (i + 1 + pos))
                .map(|(_, number)| number.clone())
                .collect();

            let results: Results = Results::new(left.clone(), right.clone());
            if let Some(sum) = results.sum {
                if expected_result == sum {
                    results_steps.push(sum);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(sum));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(difference) = results.difference {
                if expected_result == difference {
                    results_steps.push(difference);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(difference));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(product) = results.product {
                if expected_result == product {
                    results_steps.push(product);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(product));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(quotient) = results.quotient {
                if expected_result == quotient {
                    results_steps.push(quotient);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(quotient));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(reverse_difference) = results.reverse_difference {
                if expected_result == reverse_difference {
                    results_steps.push(reverse_difference);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(reverse_difference));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
            if let Some(reverse_quotient) = results.reverse_quotient {
                if expected_result == reverse_quotient {
                    results_steps.push(reverse_quotient);
                } else {
                    let mut remain_numbers = remain_numbers.clone();
                    remain_numbers.insert(0, Rc::new(reverse_quotient));
                    fork_calculation(expected_result, &remain_numbers, results_steps);
                }
            }
        }
    }
}
