#[cfg(test)]
mod integrate_tests {
    use std::rc::Rc;

    use apr::{resolve, Operand};

    #[test]
    fn test_resolve() {
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
        assert_eq!(results_steps.len(), 86);
    }
}
