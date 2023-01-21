//! Day 2: Corruption Checksum
//!
//! <https://adventofcode.com/2017/day/2>

use crate::SimpleError;
use std::cmp;
use std::error::Error;
use std::num::ParseIntError;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let spreadsheet = parse_input(input)?;

    let sum = spreadsheet
        .iter()
        .map(|row| {
            let (min, max) = row.iter().fold((u32::MAX, u32::MIN), |(min, max), &value| {
                (cmp::min(min, value), cmp::max(max, value))
            });
            max - min
        })
        .sum();

    Ok(sum)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let spreadsheet = parse_input(input)?;

    let sum = spreadsheet
        .iter()
        .map(|row| {
            for (i, &a) in row.iter().enumerate() {
                for &b in &row[i + 1..] {
                    if a % b == 0 {
                        return Ok(a / b);
                    } else if b % a == 0 {
                        return Ok(b / a);
                    }
                }
            }
            Err(SimpleError::new(format!(
                "row has no evenly divisible pair: {row:?}"
            )))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(sum)
}

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .lines()
        .map(|line| line.split('\t').map(|word| word.parse::<u32>()).collect())
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
