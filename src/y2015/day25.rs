//! Day 25: Let It Snow
//! https://adventofcode.com/2015/day/25

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let (row, col) = parse_input(input)?;

    let row = row - 1;
    let col = col - 1;

    let mut i = 0;
    let mut j = 0;
    let mut value: u64 = 20151125;

    while i != row || j != col {
        value = (value * 252533) % 33554393;

        if i == 0 {
            i = j + 1;
            j = 0;
        } else {
            i -= 1;
            j += 1;
        }
    }

    Ok(value)
}

fn parse_input(input: &str) -> Result<(usize, usize), SimpleError> {
    let line = match input.lines().next() {
        Some(line) => line,
        None => return Err(SimpleError::new(String::from("input is empty"))),
    };

    let split: Vec<_> = line.split(' ').collect();
    if split.len() < 3 {
        return Err(SimpleError::new(format!("invalid line format: {line}")));
    }

    let row = split[split.len() - 3];
    let row: usize = row[..row.len() - 1].parse()?;

    let col = split[split.len() - 1];
    let col: usize = col[..col.len() - 1].parse()?;

    Ok((row, col))
}

pub fn solve(input: &str) -> Result<(u64, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;

    Ok((solution1, String::new()))
}