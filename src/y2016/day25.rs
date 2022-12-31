//! Day 25: Clock Signal
//! https://adventofcode.com/2016/day/25

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;
use crate::y2016::assembunny::AssembunnyProgram;

fn solve_part(input: &str) -> Result<i64, SimpleError> {
    let mut program = AssembunnyProgram::from_lines(input)?;

    for a in 1.. {
        let mut registers: HashMap<_, _> =
            [('a', a), ('b', 0), ('c', 0), ('d', 0)].into_iter().collect();

        let pattern = [0, 1].into_iter().cycle().take(20);
        if program.outputs_pattern(&mut registers, pattern) {
            return Ok(a);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

pub fn solve(input: &str) -> Result<(i64, String), Box<dyn Error>> {
    let solution1 = solve_part(input)?;

    Ok((solution1, String::new()))
}