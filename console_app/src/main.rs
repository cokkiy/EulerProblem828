use std::rc::Rc;

use apr::{resolve, Operand};

fn main() {
    println!("Start...");
    let mut results_steps = vec![];
    resolve(
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
        println!("Result {}: {}", index + 1, result);
    }
}
