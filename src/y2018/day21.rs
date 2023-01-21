//! Day 21: Chronal Conversion
//!
//! <https://adventofcode.com/2018/day/21>

use crate::y2018::chronodevice::{ChronoInstruction, ChronoOperation};
use crate::SimpleError;
use std::collections::HashSet;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let (ip, program) = parse_input(input)?;

    let target_register = determine_target_register(&program)?;

    let mut registers = [0; 6];
    let mut pc = 0;
    while pc < program.len() {
        if pc == program.len() - 3 {
            return Ok(registers[target_register]);
        }

        let instruction = program[pc];

        registers[ip] = pc as u64;
        registers[instruction.c] = instruction
            .op
            .execute(&registers, instruction.a, instruction.b);
        pc = registers[ip] as usize;

        pc += 1;
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let (ip, program) = parse_input(input)?;

    let target_register = determine_target_register(&program)?;

    let mut registers = [0; 6];
    let mut pc = 0;
    let mut seen_values = HashSet::new();
    let mut last_seen_value = 0;
    while pc < program.len() {
        if pc == program.len() - 3 {
            if !seen_values.insert(registers[target_register]) {
                return Ok(last_seen_value);
            }
            last_seen_value = registers[target_register];
        }

        let instruction = program[pc];

        registers[ip] = pc as u64;
        registers[instruction.c] = instruction
            .op
            .execute(&registers, instruction.a, instruction.b);
        pc = registers[ip] as usize;

        pc += 1;
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<(usize, Vec<ChronoInstruction>), SimpleError> {
    let first_line = crate::read_single_line(input)?;
    let ip: usize = first_line[4..].parse()?;

    let program: Vec<_> = input
        .lines()
        .skip(1)
        .map(ChronoInstruction::from_line)
        .collect::<Result<_, _>>()?;

    Ok((ip, program))
}

fn determine_target_register(program: &[ChronoInstruction]) -> Result<usize, SimpleError> {
    let check_instruction = program[program.len() - 3];
    if check_instruction.op != ChronoOperation::EqualRegisterRegister {
        return Err(SimpleError::new(format!(
            "expected third-to-last instruction to be eqrr: {check_instruction:?}"
        )));
    }

    let target_register = match (check_instruction.a, check_instruction.b) {
        (0, b) => b as usize,
        (a, 0) => a as usize,
        _ => {
            return Err(SimpleError::new(format!(
                "expected either a or b to be 0 in instruction: {check_instruction:?}"
            )))
        }
    };

    Ok(target_register)
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
