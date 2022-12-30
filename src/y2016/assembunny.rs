use std::collections::HashMap;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
pub enum AssembunnyInstruction {
    CopyConstant(i64, char),
    CopyRegister(char, char),
    Increment(char),
    Decrement(char),
    JumpNotZero(char, i64),
    Jump(i64),
    Nop,
}

impl AssembunnyInstruction {
    pub fn from_line(line: &str) -> Result<Self, SimpleError> {
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

    pub fn execute(&self, registers: &mut HashMap<char, i64>, pc: &mut usize) {
        match *self {
            Self::CopyConstant(x, y) => {
                *registers.get_mut(&y).unwrap() = x;
                *pc += 1;
            }
            AssembunnyInstruction::CopyRegister(x, y) => {
                *registers.get_mut(&y).unwrap() = *registers.get(&x).unwrap();
                *pc += 1;
            }
            AssembunnyInstruction::Increment(x) => {
                *registers.get_mut(&x).unwrap() += 1;
                *pc += 1;
            }
            AssembunnyInstruction::Decrement(x) => {
                *registers.get_mut(&x).unwrap() -= 1;
                *pc += 1;
            }
            AssembunnyInstruction::JumpNotZero(x, y) => {
                if *registers.get(&x).unwrap() != 0 {
                    *pc = ((*pc as i64) + y) as usize;
                } else {
                    *pc += 1;
                }
            }
            AssembunnyInstruction::Jump(y) => {
                *pc = ((*pc as i64) + y) as usize;
            }
            AssembunnyInstruction::Nop => {
                *pc += 1;
            }
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