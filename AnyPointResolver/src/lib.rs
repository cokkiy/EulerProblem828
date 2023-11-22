pub mod arithmetic_operation;
pub mod operand;
pub mod resolver;
pub mod results;
mod tests;

pub use arithmetic_operation::ArithmeticOperation;
pub use operand::Operand;
pub use resolver::fork_calculation as resolve;
