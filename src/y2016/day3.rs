//! Day 3: Squares With Three Sides
//!
//! <https://adventofcode.com/2016/day/3>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let triangle_lengths = parse_input(input)?;

    let valid = triangle_lengths
        .into_iter()
        .filter(|&(a, b, c)| is_possible(a, b, c))
        .count();

    Ok(valid)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let triangle_lengths = parse_input(input)?;

    if triangle_lengths.len() % 3 != 0 {
        return Err(SimpleError::new(format!(
            "input length is {}, must be a mutiple of 3",
            triangle_lengths.len()
        )));
    }

    let mut valid = 0;
    for chunk in triangle_lengths.chunks(3) {
        let (a0, b0, c0) = chunk[0];
        let (a1, b1, c1) = chunk[1];
        let (a2, b2, c2) = chunk[2];

        if is_possible(a0, a1, a2) {
            valid += 1;
        }
        if is_possible(b0, b1, b2) {
            valid += 1;
        }
        if is_possible(c0, c1, c2) {
            valid += 1;
        }
    }

    Ok(valid)
}

fn is_possible(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32, u32)>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').filter(|s| !s.is_empty()).collect();
            if split.len() != 3 {
                return Err(SimpleError::new(format!("invalid line: {line}")));
            }

            let a: u32 = split[0].parse()?;
            let b: u32 = split[1].parse()?;
            let c: u32 = split[2].parse()?;

            Ok((a, b, c))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
