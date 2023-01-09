//! Day 2: 1202 Program Alarm
//! https://adventofcode.com/2019/day/2

use std::error::Error;
use crate::SimpleError;
use crate::y2019::intcode;

fn solve_part_1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut program = intcode::parse_program(input)?;

    program[1] = 12;
    program[2] = 2;

    intcode::execute_no_io(&mut program);

    Ok(program[0])
}

fn solve_part_2(input: &str) -> Result<i64, Box<dyn Error>> {
    let program = intcode::parse_program(input)?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            intcode::execute_no_io(&mut program);

            if program[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(Box::new(SimpleError::new(String::from("no solution found"))))
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}