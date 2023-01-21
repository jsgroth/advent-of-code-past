//! Day 8: Matchsticks
//! https://adventofcode.com/2015/day/8

use std::error::Error;
use std::iter::Peekable;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    input.lines().map(|line| {
        let original_len = line.len();
        let mut removed_chars = 2;

        let mut chars = line[1..original_len - 1].chars().peekable();
        while chars.peek().is_some() {
            let c = chars.next().unwrap();
            if c == '\\' {
                error_if_exhausted(&mut chars, line)?;

                removed_chars += 1;

                let next_c = chars.next().unwrap();
                if next_c == 'x' {
                    error_if_exhausted(&mut chars, line)?;
                    chars.next();
                    error_if_exhausted(&mut chars, line)?;
                    chars.next();

                    removed_chars += 2;
                }
            }
        }

        Ok(removed_chars)
    })
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    input.lines().map(|line| {
        let mut added_chars = 2;

        for c in line.chars() {
            if c == '"' || c == '\\' {
                added_chars += 1;
            }
        }

        added_chars
    })
        .sum()
}

fn error_if_exhausted<I>(iter: &mut Peekable<I>, line: &str) -> Result<(), SimpleError>
where I: Iterator<Item = char>
{
    match iter.peek() {
        Some(_) => Ok(()),
        None => Err(SimpleError::new(format!("invalid line: {line}")))
    }
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input);

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample8.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(12), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(19, solve_part_2(SAMPLE_INPUT));
    }
}