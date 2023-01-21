//! Day 12: Leonardo's Monorail
//! https://adventofcode.com/2016/day/12

use crate::y2016::assembunny::AssembunnyProgram;
use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;

fn solve_part(input: &str, initial_c_value: i64) -> Result<i64, SimpleError> {
    let mut program = AssembunnyProgram::from_lines(input)?;

    let mut registers: HashMap<_, i64> = [('a', 0), ('b', 0), ('c', initial_c_value), ('d', 0)]
        .into_iter()
        .collect();

    program.execute(&mut registers);

    Ok(*registers.get(&'a').unwrap())
}

pub fn solve(input: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let solution1 = solve_part(input, 0)?;
    let solution2 = solve_part(input, 1)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(42), solve_part(SAMPLE_INPUT, 0));
    }
}
