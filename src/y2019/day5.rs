//! Day 5: Sunny with a Chance of Asteroids
//! https://adventofcode.com/2019/day/5

use std::error::Error;
use crate::SimpleError;
use crate::y2019::intcode;

fn solve_part(input: &str, input_value: i64) -> Result<i64, Box<dyn Error>> {
    let mut program = intcode::parse_program(input)?;

    let mut outputs = Vec::new();
    intcode::execute(
        &mut program,
        || input_value,
        |output| outputs.push(output),
    );

    if outputs.is_empty() {
        return Err(Box::new(SimpleError::new(String::from("intcode program did not output anything"))));
    }

    Ok(outputs.last().copied().unwrap())
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, 1)?;
    let solution2 = solve_part(input, 5)?;

    Ok((solution1, solution2))
}