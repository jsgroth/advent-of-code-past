use std::collections::HashMap;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum InstructionArg {
    Register(char),
    Constant(i64),
}

#[derive(Debug, Clone, Copy)]
enum AssembunnyInstruction {
    Copy(InstructionArg, InstructionArg),
    Increment(char),
    Decrement(char),
    JumpNotZero(InstructionArg, InstructionArg),
    Toggle(char),
    Out(char),
    Add(char, char),
    MultiplyAdd(char, char, char),
    Nop,
}

impl AssembunnyInstruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
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
            ["tgl", x] => Ok(Self::Toggle(as_register_id(*x)?)),
            ["out", x] => Ok(Self::Out(as_register_id(*x)?)),
            _ => Err(SimpleError::new(format!("invalid line: {line}")))
        }
    }

    fn execute(&self, registers: &mut HashMap<char, i64>, pc: &mut usize, program: &mut Vec<Self>) -> Option<i64> {
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
            Self::Toggle(x) => {
                let value = *registers.get_mut(&x).unwrap() + (*pc as i64);
                if value >= 0 && value < program.len() as i64 {
                    let new_instruction = match program[value as usize] {
                        Self::Copy(x, y) => Self::JumpNotZero(x, y),
                        Self::JumpNotZero(x, y) => Self::Copy(x, y),
                        Self::Increment(x) => Self::Decrement(x),
                        Self::Decrement(x) | Self::Toggle(x) => Self::Increment(x),
                        Self::Out(..) => panic!("cannot toggle out"),
                        Self::Add(..) | Self::MultiplyAdd(..) | Self::Nop => panic!("cannot toggle {:?}, optimizations failed", program[value as usize]),
                    };
                    program[value as usize] = new_instruction;
                }

                *pc += 1;
            }
            Self::Out(x) => {
                *pc += 1;
                return Some(*registers.get(&x).unwrap());
            }
            Self::Add(x, y) => {
                *registers.get_mut(&x).unwrap() += *registers.get(&y).unwrap();
                *pc += 1;
            }
            Self::MultiplyAdd(x, y, z) => {
                *registers.get_mut(&x).unwrap() += *registers.get(&y).unwrap() * *registers.get(&z).unwrap();
                *pc += 1;
            }
            Self::Nop => {
                *pc += 1;
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct AssembunnyProgram {
    instructions: Vec<AssembunnyInstruction>,
}

impl AssembunnyProgram {
    pub fn from_lines(input: &str) -> Result<AssembunnyProgram, SimpleError> {
        let instructions: Result<Vec<_>, _> = input.lines().map(AssembunnyInstruction::from_line).collect();
        Ok(AssembunnyProgram { instructions: instructions? })
    }

    pub fn optimize_multiplies(&mut self) {
        let mut add_instructions = Vec::new();
        for (i, window) in self.instructions.windows(4).enumerate() {
            if let AssembunnyInstruction::Copy(InstructionArg::Register(cp_x), InstructionArg::Register(cp_y)) = window[0] {
                if let AssembunnyInstruction::JumpNotZero(InstructionArg::Register(jnz_x), InstructionArg::Constant(-2)) = window[3] {
                    match (window[1], window[2]) {
                        (AssembunnyInstruction::Increment(inc_x), AssembunnyInstruction::Decrement(dec_x)) |
                        (AssembunnyInstruction::Decrement(dec_x), AssembunnyInstruction::Increment(inc_x)) => {
                            if cp_y == jnz_x && jnz_x == dec_x && inc_x != dec_x {
                                add_instructions.push((i, inc_x, cp_x));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        for &(i, x, y) in &add_instructions {
            self.instructions[i] = AssembunnyInstruction::Add(x, y);
            self.instructions[i + 1] = AssembunnyInstruction::Nop;
            self.instructions[i + 2] = AssembunnyInstruction::Nop;
            self.instructions[i + 3] = AssembunnyInstruction::Nop;
        }

        let mut multiply_instructions = Vec::new();
        for (i, window) in self.instructions.windows(6).enumerate() {
            if let AssembunnyInstruction::Add(add_x, add_y) = window[0] {
                if let AssembunnyInstruction::JumpNotZero(InstructionArg::Register(jnz_x), InstructionArg::Constant(-5)) = window[5] {
                    if let AssembunnyInstruction::Decrement(dec_x) = window[4] {
                        if dec_x == jnz_x && dec_x != add_x && dec_x != add_y {
                            multiply_instructions.push((i, add_x, add_y, dec_x));
                        }
                    }
                }
            }
        }

        for &(i, x, y, z) in &multiply_instructions {
            self.instructions[i] = AssembunnyInstruction::MultiplyAdd(x, y, z);
            self.instructions[i + 4] = AssembunnyInstruction::Nop;
            self.instructions[i + 5] = AssembunnyInstruction::Nop;
        }
    }

    pub fn execute(&mut self, registers: &mut HashMap<char, i64>) {
        let mut pc = 0;
        while pc < self.instructions.len() {
            let instruction = self.instructions[pc];
            instruction.execute(registers, &mut pc, &mut self.instructions);
        }
    }

    pub fn outputs_pattern(&mut self, registers: &mut HashMap<char, i64>, mut pattern: impl Iterator<Item = i64>) -> bool {
        let mut pc = 0;
        while pc < self.instructions.len() {
            let instruction = self.instructions[pc];
            if let Some(value) = instruction.execute(registers, &mut pc, &mut self.instructions) {
                match pattern.next() {
                    Some(pattern_value) => {
                        if value != pattern_value {
                            return false;
                        }
                    }
                    None => {
                        return true;
                    }
                }
            }
        }

        return pattern.next().is_none();
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