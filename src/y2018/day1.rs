//! Day 1: Chronal Calibration
//! https://adventofcode.com/2018/day/1

use std::collections::HashSet;
use std::error::Error;
use std::iter;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    Ok(parse_input(input)?.into_iter().sum())
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let frequencies = parse_input(input)?;

    let mut current_freq = 0;
    let mut seen_freqs: HashSet<_> = iter::once(0).collect();
    for n in frequencies.iter().copied().cycle() {
        current_freq += n;
        if !seen_freqs.insert(current_freq) {
            return Ok(current_freq);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<Vec<i32>, SimpleError> {
    input.lines().map(|line| {
        match line.chars().next() {
            Some('+') => Ok(line[1..].parse::<i32>()?),
            Some('-') => Ok(line.parse::<i32>()?),
            _ => Err(SimpleError::new(format!("invalid line: {line}")))
        }
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}