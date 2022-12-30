//! Day 23: Safe Cracking
//! https://adventofcode.com/2016/day/23

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    Ok(0)
}

fn solve_part_2(_input: &str) -> Result<usize, SimpleError> {
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

    #[test]
    fn test_sample_input_part_1() {}
}