
use super::engine::Register;

pub struct Parser;

impl Parser {
    fn parse(&self, input: &str) -> Vec<Operation> {
        vec![Operation::Binary(NoOp)]
    }
}
