//! Day 23: Coprocessor Conflagration
//! https://adventofcode.com/2017/day/23

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Arg {
    Register(char),
    Constant(i64),
}

impl Arg {
    fn from_str(s: &str) -> Result<Self, SimpleError> {
        match s.parse::<i64>() {
            Ok(n) => Ok(Self::Constant(n)),
            Err(_) => Ok(Self::Register(s.parse()?)),
        }
    }

    fn get_value(&self, registers: &HashMap<char, i64>) -> i64 {
        match *self {
            Self::Register(x) => registers.get(&x).copied().unwrap_or(0),
            Self::Constant(n) => n,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Set(char, Arg),
    Subtract(char, Arg),
    Multiply(char, Arg),
    JumpNotZero(Arg, Arg),
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["set", x, y] => Ok(Self::Set(x.parse()?, Arg::from_str(y)?)),
            ["sub", x, y] => Ok(Self::Subtract(x.parse()?, Arg::from_str(y)?)),
            ["mul", x, y] => Ok(Self::Multiply(x.parse()?, Arg::from_str(y)?)),
            ["jnz", x, y] => Ok(Self::JumpNotZero(Arg::from_str(x)?, Arg::from_str(y)?)),
            _ => Err(SimpleError::new(format!("invalid line: {line}")))
        }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let instructions = parse_input(input)?;

    let mut registers = HashMap::new();
    let mut pc = 0;
    let mut mul_count = 0;
    while pc < instructions.len() {
        match instructions[pc] {
            Instruction::Set(x, y) => {
                registers.insert(x, y.get_value(&registers));
                pc += 1;
            }
            Instruction::Subtract(x, y) => {
                registers.insert(x, registers.get(&x).copied().unwrap_or(0) - y.get_value(&registers));
                pc += 1;
            }
            Instruction::Multiply(x, y) => {
                registers.insert(x, registers.get(&x).copied().unwrap_or(0) * y.get_value(&registers));
                pc += 1;
                mul_count += 1;
            }
            Instruction::JumpNotZero(x, y) => {
                if x.get_value(&registers) != 0 {
                    pc = ((pc as i64) + y.get_value(&registers)) as usize;
                } else {
                    pc += 1;
                }
            }
        }
    }

    Ok(mul_count)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let instructions = parse_input(input)?;

    let mut b = match instructions[0] {
        Instruction::Set('b', Arg::Constant(n)) => n,
        _ => return Err(SimpleError::new(format!("expected 'set b X', got {:?}", instructions[0])))
    };

    b *= match instructions[4] {
        Instruction::Multiply('b', Arg::Constant(n)) => n,
        _ => return Err(SimpleError::new(format!("expected 'mul b X', got {:?}", instructions[4])))
    };

    b -= match instructions[5] {
        Instruction::Subtract('b', Arg::Constant(n)) => n,
        _ => return Err(SimpleError::new(format!("expected 'sub b X', got {:?}", instructions[5])))
    };

    if instructions[6] != Instruction::Set('c', Arg::Register('b')) {
        return Err(SimpleError::new(format!("expected 'set c b', got {:?}", instructions[6])))
    }

    let c = b - match instructions[7] {
        Instruction::Subtract('c', Arg::Constant(n)) => n,
        _ => return Err(SimpleError::new(format!("expected 'sub c X', got {:?}", instructions[7])))
    };

    // I think this is always 17 but just in case
    let step = match instructions[instructions.len() - 2] {
        Instruction::Subtract('b', Arg::Constant(n)) => -n,
        _ => return Err(SimpleError::new(format!("expected 'sub b X', got {:?}", instructions[instructions.len() - 2])))
    };

    let mut not_prime_count = 0;
    for n in (b..=c).step_by(step as usize) {
        if !is_prime(n) {
            not_prime_count += 1;
        }
    }

    Ok(not_prime_count)
}

fn is_prime(n: i64) -> bool {
    let sqrt = (n as f64).sqrt().ceil() as i64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}