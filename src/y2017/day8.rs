//! Day 8: I Heard You Like Registers
//! https://adventofcode.com/2017/day/8

use crate::SimpleError;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
enum Condition {
    Gt(String, i64),
    Ge(String, i64),
    Lt(String, i64),
    Le(String, i64),
    Eq(String, i64),
    Ne(String, i64),
}

impl Condition {
    fn from_str(s: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = s.split(' ').collect();
        match split.as_slice() {
            [a, ">", b] => Ok(Self::Gt(String::from(*a), b.parse()?)),
            [a, ">=", b] => Ok(Self::Ge(String::from(*a), b.parse()?)),
            [a, "<", b] => Ok(Self::Lt(String::from(*a), b.parse()?)),
            [a, "<=", b] => Ok(Self::Le(String::from(*a), b.parse()?)),
            [a, "==", b] => Ok(Self::Eq(String::from(*a), b.parse()?)),
            [a, "!=", b] => Ok(Self::Ne(String::from(*a), b.parse()?)),
            _ => Err(SimpleError::new(format!("invalid line: {s}"))),
        }
    }

    fn evaluate(&self, registers: &HashMap<String, i64>) -> bool {
        match self {
            Self::Gt(a, b) => registers.get(a).copied().unwrap_or(0) > *b,
            Self::Ge(a, b) => registers.get(a).copied().unwrap_or(0) >= *b,
            Self::Lt(a, b) => registers.get(a).copied().unwrap_or(0) < *b,
            Self::Le(a, b) => registers.get(a).copied().unwrap_or(0) <= *b,
            Self::Eq(a, b) => registers.get(a).copied().unwrap_or(0) == *b,
            Self::Ne(a, b) => registers.get(a).copied().unwrap_or(0) != *b,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    register: String,
    value: i64,
    condition: Condition,
}

fn solve_part(input: &str, find_max_ever: bool) -> Result<i64, SimpleError> {
    let instructions = parse_input(input)?;

    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut max_ever = i64::MIN;
    for instruction in &instructions {
        if instruction.condition.evaluate(&registers) {
            if let Some(register_value) = registers.get_mut(&instruction.register) {
                *register_value += instruction.value;
            } else {
                registers.insert(instruction.register.clone(), instruction.value);
            }
            max_ever = cmp::max(max_ever, *registers.get(&instruction.register).unwrap());
        }
    }

    if find_max_ever {
        return Ok(max_ever);
    }

    let max = registers
        .into_values()
        .max()
        .ok_or_else(|| SimpleError::new(String::from("no registers were set")))?;

    Ok(max)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.splitn(5, ' ').collect();

            let register = String::from(split[0]);

            let mut value: i64 = split[2].parse()?;
            if split[1] == "dec" {
                value = -value;
            }

            let condition = Condition::from_str(split[4])?;

            Ok(Instruction {
                register,
                value,
                condition,
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample8.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(1), solve_part(SAMPLE_INPUT, false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(10), solve_part(SAMPLE_INPUT, true));
    }
}
