//! Day 18: Duet
//! https://adventofcode.com/2017/day/18

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::iter;
use std::rc::Rc;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
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
            Self::Register(x) => get_register_value(registers, x),
            Self::Constant(n) => n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Send(char),
    Set(char, Arg),
    Add(char, Arg),
    Multiply(char, Arg),
    Modulus(char, Arg),
    Receive(char),
    JumpGreaterZero(Arg, Arg),
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["snd", x] => Ok(Self::Send(x.parse()?)),
            ["set", x, y] => Ok(Self::Set(x.parse()?, Arg::from_str(*y)?)),
            ["add", x, y] => Ok(Self::Add(x.parse()?, Arg::from_str(*y)?)),
            ["mul", x, y] => Ok(Self::Multiply(x.parse()?, Arg::from_str(*y)?)),
            ["mod", x, y] => Ok(Self::Modulus(x.parse()?, Arg::from_str(*y)?)),
            ["rcv", x] => Ok(Self::Receive(x.parse()?)),
            ["jgz", x, y] => Ok(Self::JumpGreaterZero(Arg::from_str(*x)?, Arg::from_str(*y)?)),
            _ => Err(SimpleError::new(format!("invalid line: {line}")))
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    send_queue: Rc<RefCell<VecDeque<i64>>>,
    receive_queue: Rc<RefCell<VecDeque<i64>>>,
    pc: usize,
    total_sent: usize,
}

impl Program {
    fn new(
        program_id: i64,
        instructions: Vec<Instruction>,
        send_queue: Rc<RefCell<VecDeque<i64>>>,
        receive_queue: Rc<RefCell<VecDeque<i64>>>,
    ) -> Self {
        Self {
            instructions,
            registers: iter::once(('p', program_id)).collect(),
            send_queue,
            receive_queue,
            pc: 0,
            total_sent: 0,
        }
    }

    fn find_first_received_value(&mut self) -> Option<i64> {
        while self.pc < self.instructions.len() {
            if let Instruction::Receive(x) = self.instructions[self.pc] {
                if get_register_value(&self.registers, x) != 0 {
                    return self.send_queue.borrow().back().copied();
                }
                continue;
            }

            self.execute_instruction(self.instructions[self.pc]);
        }

        None
    }

    fn execute(&mut self) -> bool {
        let mut executed = false;

        while self.pc < self.instructions.len() {
            if !self.execute_instruction(self.instructions[self.pc]) {
                return executed;
            }

            executed = true;
        }

        executed
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> bool {
        let Program {
            registers,
            send_queue,
            receive_queue,
            pc,
            total_sent,
            ..
        } = self;

        match instruction {
            Instruction::Send(x) => {
                send_queue.borrow_mut().push_back(get_register_value(registers, x));
                *pc += 1;
                *total_sent += 1;
            }
            Instruction::Set(x, y) => {
                registers.insert(x, y.get_value(registers));
                *pc += 1;
            }
            Instruction::Add(x, y) => {
                registers.insert(x, get_register_value(registers, x) + y.get_value(registers));
                *pc += 1;
            }
            Instruction::Multiply(x, y) => {
                registers.insert(x, get_register_value(registers, x) * y.get_value(registers));
                *pc += 1;
            }
            Instruction::Modulus(x, y) => {
                registers.insert(x, get_register_value(registers, x) % y.get_value(registers));
                *pc += 1;
            }
            Instruction::Receive(x) => {
                if receive_queue.borrow().is_empty() {
                    return false;
                }
                registers.insert(x, receive_queue.borrow_mut().pop_front().unwrap());
                *pc += 1;
            }
            Instruction::JumpGreaterZero(x, y) => {
                if x.get_value(registers) > 0 {
                    *pc = ((*pc as i64) + y.get_value(registers)) as usize;
                } else {
                    *pc += 1;
                }
            }
        }

        true
    }
}

fn get_register_value(registers: &HashMap<char, i64>, x: char) -> i64 {
    registers.get(&x).copied().unwrap_or(0)
}

fn solve_part_1(input: &str) -> Result<i64, SimpleError> {
    let instructions = parse_input(input)?;

    let send_queue = Rc::new(RefCell::new(VecDeque::new()));
    let receive_queue = Rc::clone(&send_queue);
    let mut program = Program::new(0, instructions, send_queue, receive_queue);

    program.find_first_received_value().ok_or(
        SimpleError::new(String::from("no solution found"))
    )
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let instructions = parse_input(input)?;

    let queue0 = Rc::new(RefCell::new(VecDeque::new()));
    let queue1 = Rc::new(RefCell::new(VecDeque::new()));

    let mut program0 = Program::new(
        0,
        instructions.clone(),
        Rc::clone(&queue0),
        Rc::clone(&queue1),
    );
    let mut program1 = Program::new(
        1,
        instructions,
        queue1,
        queue0,
    );

    while program0.execute() || program1.execute() {}

    Ok(program1.total_sent)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, SimpleError> {
    input.lines().map(Instruction::from_line).collect()
}

pub fn solve(input: &str) -> Result<(i64, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample18.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(4), solve_part_1(SAMPLE_INPUT));
    }
}