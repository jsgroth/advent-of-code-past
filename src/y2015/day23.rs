//! Day 23: Opening the Turing Lock
//!
//! <https://adventofcode.com/2015/day/23>

use crate::SimpleError;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(i32),
    JumpIfEven(char, i32),
    JumpIfOne(char, i32),
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let (operator, operands) = line.split_once(' ').ok_or_else(|| {
            SimpleError::new(format!("invalid line format, missing space: {line}"))
        })?;

        if operands.is_empty() {
            return Err(SimpleError::new(format!(
                "invalid line format, missing operands: {line}"
            )));
        }

        let instruction = match operator {
            "hlf" => Self::Half(operands.chars().next().unwrap()),
            "tpl" => Self::Triple(operands.chars().next().unwrap()),
            "inc" => Self::Increment(operands.chars().next().unwrap()),
            "jmp" => Self::Jump(operands.parse()?),
            "jie" => {
                let (register, offset) = parse_jump_if_operands(operands)?;
                Self::JumpIfEven(register, offset)
            }
            "jio" => {
                let (register, offset) = parse_jump_if_operands(operands)?;
                Self::JumpIfOne(register, offset)
            }
            _ => return Err(SimpleError::new(format!("invalid operator: {line}"))),
        };

        Ok(instruction)
    }
}

fn solve_part(input: &str, initial_a_value: u64) -> Result<u64, SimpleError> {
    let instructions = parse_input(input)?;

    let mut a = initial_a_value;
    let mut b = 0;

    let mut pc = 0;
    while pc < instructions.len() {
        match instructions[pc] {
            Instruction::Half(r) => {
                if r == 'a' {
                    a /= 2;
                } else {
                    b /= 2;
                }
                pc += 1;
            }
            Instruction::Triple(r) => {
                if r == 'a' {
                    a *= 3;
                } else {
                    b *= 3;
                }
                pc += 1;
            }
            Instruction::Increment(r) => {
                if r == 'a' {
                    a += 1;
                } else {
                    b += 1;
                }
                pc += 1;
            }
            Instruction::Jump(offset) => {
                pc = ((pc as i32) + offset) as usize;
            }
            Instruction::JumpIfEven(r, offset) => {
                let test_value = if r == 'a' { a } else { b };
                if test_value % 2 == 0 {
                    pc = ((pc as i32) + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                let test_value = if r == 'a' { a } else { b };
                if test_value == 1 {
                    pc = ((pc as i32) + offset) as usize;
                } else {
                    pc += 1;
                }
            }
        }
    }

    Ok(b)
}

fn parse_jump_if_operands(operands: &str) -> Result<(char, i32), SimpleError> {
    let (register, offset) = operands.split_once(", ").ok_or_else(|| {
        SimpleError::new(format!(
            "invalid line format for jie, missing comma: {operands}"
        ))
    })?;

    if register.is_empty() {
        return Err(SimpleError::new(format!(
            "invalid line format for jie, missing register: {operands}"
        )));
    }

    Ok((register.chars().next().unwrap(), offset.parse()?))
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part(input, 0)?;
    let solution2 = solve_part(input, 1)?;

    Ok((solution1, solution2))
}
