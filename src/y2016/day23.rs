//! Day 23: Safe Cracking
//! https://adventofcode.com/2016/day/23

use crate::y2016::assembunny::AssembunnyProgram;
use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;

fn solve_part(input: &str, initial_a_value: i64) -> Result<i64, SimpleError> {
    let mut program = AssembunnyProgram::from_lines(input)?;

    let mut registers: HashMap<_, _> = [('a', initial_a_value), ('b', 0), ('c', 0), ('d', 0)]
        .into_iter()
        .collect();

    program.optimize_multiplies();
    program.execute(&mut registers);

    Ok(*registers.get(&'a').unwrap())
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, 7)?;
    let solution2 = solve_part(input, 12)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample23.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(3), solve_part(SAMPLE_INPUT, 0));
    }
}
