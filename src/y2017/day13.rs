//! Day 13: Packet Scanners
//!
//! <https://adventofcode.com/2017/day/13>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let scanner_layers = parse_input(input)?;

    let total_severity = scanner_layers
        .into_iter()
        .filter_map(|(depth, range)| {
            if depth % (2 * (range - 1)) == 0 {
                Some(depth * range)
            } else {
                None
            }
        })
        .sum();

    Ok(total_severity)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let scanner_layers = parse_input(input)?;

    for i in 1.. {
        if !scanner_layers
            .iter()
            .any(|&(depth, range)| (depth + i) % (2 * (range - 1)) == 0)
        {
            return Ok(i);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<Vec<(usize, usize)>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let (depth, range) = line
                .split_once(": ")
                .ok_or_else(|| SimpleError::new(format!("invalid line, no ': ': {line}")))?;
            Ok((depth.parse()?, range.parse()?))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample13.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(24), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(10), solve_part_2(SAMPLE_INPUT));
    }
}
