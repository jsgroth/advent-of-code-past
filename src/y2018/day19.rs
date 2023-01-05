//! Day 19: Go With The Flow
//! https://adventofcode.com/2018/day/19

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    Ok(0)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    Ok(0)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample19.txt");

    #[test]
    fn test_sample_input_part_1() {

    }
}