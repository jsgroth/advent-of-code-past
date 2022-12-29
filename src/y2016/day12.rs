//! Day 12: Leonardo's Monorail
//! https://adventofcode.com/2016/day/12

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    CopyConstant(i64, char),
    CopyRegister(char, char),
    Increment(char),
    Decrement(char),
    JumpNotZero(char, i64),
    Jump(i64),
    Nop,
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["cpy", x, y] => {
                match x.parse::<i64>() {
                    Ok(x) => Ok(Self::CopyConstant(x, as_register_id(*y)?)),
                    Err(_) => Ok(Self::CopyRegister(as_register_id(*x)?, as_register_id(*y)?)),
                }
            },
            ["inc", x] => Ok(Self::Increment(as_register_id(*x)?)),
            ["dec", x] => Ok(Self::Decrement(as_register_id(*x)?)),
            ["jnz", x, y] => {
                match x.parse::<i64>() {
                    Ok(x) => {
                        if x != 0 {
                            Ok(Self::Jump(y.parse()?))
                        } else {
                            Ok(Self::Nop)
                        }
                    }
                    Err(_) => Ok(Self::JumpNotZero(as_register_id(*x)?, y.parse()?)),
                }
            },
            _ => Err(SimpleError::new(format!("invalid line: {line}")))
        }
    }
}

fn as_register_id(s: &str) -> Result<char, SimpleError> {
    match s.chars().next() {
        Some(c) => {
            if !('a'..='d').contains(&c) {
                return Err(SimpleError::new(format!("invalid register id: {s}")));
            }
            Ok(c)
        },
        None => Err(SimpleError::new(String::from("cannot get first char of empty string")))
    }
}

fn solve_part(input: &str, initial_c_value: i64) -> Result<i64, SimpleError> {
    let instructions = parse_input(input)?;

    let mut registers: HashMap<_, i64> =
        [('a', 0), ('b', 0), ('c', initial_c_value), ('d', 0)].into_iter().collect();

    let mut pc = 0;
    while pc < instructions.len() {
        match instructions[pc] {
            Instruction::CopyConstant(x, y) => {
                *registers.get_mut(&y).unwrap() = x;
                pc += 1;
            }
            Instruction::CopyRegister(x, y) => {
                *registers.get_mut(&y).unwrap() = *registers.get(&x).unwrap();
                pc += 1;
            }
            Instruction::Increment(x) => {
                *registers.get_mut(&x).unwrap() += 1;
                pc += 1;
            }
            Instruction::Decrement(x) => {
                *registers.get_mut(&x).unwrap() -= 1;
                pc += 1;
            }
            Instruction::JumpNotZero(x, y) => {
                if *registers.get(&x).unwrap() != 0 {
                    pc = ((pc as i64) + y) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::Jump(y) => {
                pc = ((pc as i64) + y) as usize;
            }
            Instruction::Nop => {
                pc += 1;
            }
        }
    }

    Ok(*registers.get(&'a').unwrap())
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, 0)?;
    let solution2 = solve_part(input, 1)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(42), solve_part(SAMPLE_INPUT, 0));
    }
}