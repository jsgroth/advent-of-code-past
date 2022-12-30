use std::collections::HashMap;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
pub enum InstructionArg {
    Register(char),
    Constant(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum AssembunnyInstruction {
    Copy(InstructionArg, InstructionArg),
    Increment(char),
    Decrement(char),
    JumpNotZero(InstructionArg, InstructionArg),
}

impl AssembunnyInstruction {
    pub fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["cpy", x, y] => {
                Ok(Self::Copy(parse_argument(*x)?, parse_argument(*y)?))
            },
            ["inc", x] => Ok(Self::Increment(as_register_id(*x)?)),
            ["dec", x] => Ok(Self::Decrement(as_register_id(*x)?)),
            ["jnz", x, y] => {
                Ok(Self::JumpNotZero(parse_argument(*x)?, parse_argument(*y)?))
            },
            _ => Err(SimpleError::new(format!("invalid line: {line}")))
        }
    }

    pub fn execute(&self, registers: &mut HashMap<char, i64>, pc: &mut usize) {
        match *self {
            Self::Copy(x, y) => {
                match y {
                    InstructionArg::Register(y) => {
                        *registers.get_mut(&y).unwrap() = read_arg_value(x, registers);
                    }
                    InstructionArg::Constant(_) => {},
                }
                *pc += 1;
            }
            Self::Increment(x) => {
                *registers.get_mut(&x).unwrap() += 1;
                *pc += 1;
            }
            Self::Decrement(x) => {
                *registers.get_mut(&x).unwrap() -= 1;
                *pc += 1;
            }
            Self::JumpNotZero(x, y) => {
                let x = read_arg_value(x, registers);
                let y = read_arg_value(y, registers);
                if x != 0 {
                    *pc = ((*pc as i64) + y) as usize;
                } else {
                    *pc += 1;
                }
            }
        }
    }
}

fn read_arg_value(arg: InstructionArg, registers: &HashMap<char, i64>) -> i64 {
    match arg {
        InstructionArg::Constant(n) => n,
        InstructionArg::Register(r) => *registers.get(&r).unwrap(),
    }
}

fn parse_argument(s: &str) -> Result<InstructionArg, SimpleError> {
    match s.parse::<i64>() {
        Ok(n) => Ok(InstructionArg::Constant(n)),
        Err(_) => Ok(InstructionArg::Register(as_register_id(s)?)),
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