//! Day 7: Some Assembly Required
//!
//! <https://adventofcode.com/2015/day/7>

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Not;

enum Operation {
    AssignConstant(u16),
    AssignWire(String),
    AndConstant(u16, String),
    AndWires(String, String),
    Or(String, String),
    Not(String),
    LShift(String, u16),
    RShift(String, u16),
}

struct Instruction {
    operation: Operation,
    target: String,
}

impl Instruction {
    fn new(operation: Operation, target: String) -> Self {
        Self { operation, target }
    }
}

struct EvaluationContext<'a> {
    instruction_map: HashMap<&'a str, &'a Instruction>,
    computed_values: HashMap<&'a str, u16>,
}

impl<'a> EvaluationContext<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        let instruction_map: HashMap<_, _> = instructions
            .iter()
            .map(|instruction| (instruction.target.as_str(), instruction))
            .collect();

        Self {
            instruction_map,
            computed_values: HashMap::new(),
        }
    }

    fn evaluate(&mut self, target: &'a str) -> Result<u16, SimpleError> {
        if let Some(value) = self.computed_values.get(target) {
            return Ok(*value);
        }

        let instruction = *self
            .instruction_map
            .get(target)
            .ok_or_else(|| SimpleError::new(format!("no instruction found for '{target}'")))?;

        let op_result = match &instruction.operation {
            Operation::AssignConstant(n) => *n,
            Operation::AssignWire(a) => self.evaluate(a)?,
            Operation::AndConstant(n, b) => *n & self.evaluate(b)?,
            Operation::AndWires(a, b) => self.evaluate(a)? & self.evaluate(b)?,
            Operation::Or(a, b) => self.evaluate(a)? | self.evaluate(b)?,
            Operation::Not(a) => self.evaluate(a)?.not(),
            Operation::LShift(a, n) => self.evaluate(a)? << *n,
            Operation::RShift(a, n) => self.evaluate(a)? >> *n,
        };

        self.computed_values.insert(target, op_result);

        Ok(op_result)
    }

    fn with_value(&mut self, target: &'a str, value: u16) -> &mut Self {
        self.computed_values.insert(target, value);
        self
    }
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();
            let operation = match split.as_slice() {
                [n, "->", _] => match n.parse() {
                    Ok(n) => Operation::AssignConstant(n),
                    Err(_) => Operation::AssignWire(String::from(*n)),
                },
                [a, "AND", b, "->", _] => match a.parse() {
                    Ok(a) => Operation::AndConstant(a, String::from(*b)),
                    Err(_) => Operation::AndWires(String::from(*a), String::from(*b)),
                },
                [a, "OR", b, "->", _] => Operation::Or(String::from(*a), String::from(*b)),
                ["NOT", a, "->", _] => Operation::Not(String::from(*a)),
                [a, "LSHIFT", n, "->", _] => Operation::LShift(String::from(*a), n.parse()?),
                [a, "RSHIFT", n, "->", _] => Operation::RShift(String::from(*a), n.parse()?),
                _ => return Err(SimpleError::new(format!("unrecognized operation: {line}"))),
            };

            let target = split.last().unwrap();

            Ok(Instruction::new(operation, String::from(*target)))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(u16, u16), Box<dyn Error>> {
    let instructions = parse_input(input)?;

    let solution1 = EvaluationContext::new(&instructions).evaluate("a")?;

    let solution2 = EvaluationContext::new(&instructions)
        .with_value("b", solution1)
        .evaluate("a")?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample7.txt");

    fn solve_part_1(input: &str, target_wire: &str) -> Result<u16, SimpleError> {
        let instructions = parse_input(input)?;
        EvaluationContext::new(&instructions).evaluate(target_wire)
    }

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(123), solve_part_1(SAMPLE_INPUT, "x"));
        assert_eq!(Ok(456), solve_part_1(SAMPLE_INPUT, "y"));
        assert_eq!(Ok(72), solve_part_1(SAMPLE_INPUT, "d"));
        assert_eq!(Ok(507), solve_part_1(SAMPLE_INPUT, "e"));
        assert_eq!(Ok(492), solve_part_1(SAMPLE_INPUT, "f"));
        assert_eq!(Ok(114), solve_part_1(SAMPLE_INPUT, "g"));
        assert_eq!(Ok(65412), solve_part_1(SAMPLE_INPUT, "h"));
        assert_eq!(Ok(65079), solve_part_1(SAMPLE_INPUT, "i"));
    }

    #[test]
    fn test_invalid_input() {
        assert!(solve_part_1("a AND b -> c", "c").is_err());
        assert!(solve_part_1("asdf", "a").is_err());
    }
}
