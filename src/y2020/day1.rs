//! Day 1: Report Repair
//!
//! <https://adventofcode.com/2020/day/1>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<_, _>>()?;

    for (i, &n) in numbers.iter().enumerate() {
        for &m in &numbers[i + 1..] {
            if n + m == 2020 {
                return Ok(n * m);
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<_, _>>()?;

    for (i, &n) in numbers.iter().enumerate() {
        for (j, &m) in numbers.iter().enumerate().skip(i + 1) {
            for &p in &numbers[j + 1..] {
                if n + m + p == 2020 {
                    return Ok(n * m * p);
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
