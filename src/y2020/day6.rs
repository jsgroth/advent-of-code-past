//! Day 6: Custom Customs
//! https://adventofcode.com/2020/day/6

use crate::SimpleError;
use std::collections::HashSet;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    let total_answered = lines
        .split(|s| s.is_empty())
        .map(|group| {
            group
                .iter()
                .flat_map(|line| line.chars())
                .collect::<HashSet<_>>()
                .len() as u32
        })
        .sum();

    Ok(total_answered)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    let total_all_answered = lines
        .split(|s| s.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
                .unwrap()
                .len() as u32
        })
        .sum();

    Ok(total_all_answered)
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
