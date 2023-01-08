//! Day 4: Secure Container
//! https://adventofcode.com/2019/day/4

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let (start, end) = parse_input(input)?;

    let mut valid_count = 0;
    for i in start..=end {
        let digits = to_digits(i);

        if !digits.windows(2).any(|window| window[0] == window[1]) {
            continue;
        }

        if digits.windows(2).any(|window| window[0] > window[1]) {
            continue;
        }

        valid_count += 1;
    }

    Ok(valid_count)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let (start, end) = parse_input(input)?;

    let mut valid_count = 0;
    for i in start..=end {
        let digits = to_digits(i);

        if digits.windows(2).any(|window| window[0] > window[1]) {
            continue;
        }

        if !has_valid_adjacent_pair(&digits) {
            continue;
        }

        valid_count += 1;
    }

    Ok(valid_count)
}

fn to_digits(n: u32) -> Vec<u32> {
    n.to_string().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn has_valid_adjacent_pair(digits: &[u32]) -> bool {
    for i in 0..digits.len() - 1 {
        if digits[i] != digits[i + 1] {
            continue;
        }

        if (i > 0 && digits[i - 1] == digits[i]) || (i < digits.len() - 2 && digits[i + 2] == digits[i]) {
            continue;
        }

        return true;
    }

    false
}

fn parse_input(input: &str) -> Result<(u32, u32), SimpleError> {
    let line = crate::read_single_line(input)?;

    let (start, end) = line.split_once('-').ok_or(
        SimpleError::new(format!("invalid range string: {line}"))
    )?;

    let start = start.parse()?;
    let end = end.parse()?;

    Ok((start, end))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}