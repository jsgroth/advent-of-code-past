//! Day 8: Handheld Halting
//!
//! <https://adventofcode.com/2020/day/8>

use crate::SimpleError;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    NoOp(i64),
    Accumulate(i64),
    Jump(i64),
}

impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(' ').collect();
        match split.as_slice() {
            ["nop", n] => {
                let n = n.parse()?;
                Ok(Self::NoOp(n))
            }
            ["acc", n] => {
                let n = n.parse()?;
                Ok(Self::Accumulate(n))
            }
            ["jmp", n] => {
                let n = n.parse()?;
                Ok(Self::Jump(n))
            }
            _ => Err(SimpleError::new(format!("invalid instruction: {s}"))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ExecutionResult {
    InfiniteLoop(i64),
    Terminated(i64),
}

fn solve_part_1(input: &str) -> Result<i64, SimpleError> {
    let instructions = parse_input(input)?;

    if let ExecutionResult::InfiniteLoop(accumulator) = execute_program(&instructions) {
        Ok(accumulator)
    } else {
        Err(SimpleError::new(String::from("no solution found")))
    }
}

fn solve_part_2(input: &str) -> Result<i64, SimpleError> {
    let instructions = parse_input(input)?;

    for (i, &instruction) in instructions.iter().enumerate() {
        let replacement_instruction = match instruction {
            Instruction::NoOp(n) => Instruction::Jump(n),
            Instruction::Jump(n) => Instruction::NoOp(n),
            _ => {
                continue;
            }
        };

        let mut instructions = instructions.clone();
        instructions[i] = replacement_instruction;

        if let ExecutionResult::Terminated(accumulator) = execute_program(&instructions) {
            return Ok(accumulator);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn execute_program(instructions: &[Instruction]) -> ExecutionResult {
    let mut executed = vec![false; instructions.len()];

    let mut pc = 0;
    let mut accumulator = 0;
    while pc < instructions.len() {
        if executed[pc] {
            return ExecutionResult::InfiniteLoop(accumulator);
        }

        executed[pc] = true;

        match instructions[pc] {
            Instruction::NoOp(_) => {
                pc += 1;
            }
            Instruction::Accumulate(n) => {
                accumulator += n;
                pc += 1;
            }
            Instruction::Jump(n) => {
                pc = (pc as i64 + n) as usize;
            }
        }
    }

    ExecutionResult::Terminated(accumulator)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_str).collect()
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample8.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(5), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(8), solve_part_2(SAMPLE_INPUT));
    }
}
