//! Day 1: Not Quite Lisp
//!
//! <https://adventofcode.com/2015/day/1>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let increments = parse_input(input)?;
    let floor = increments
        .into_iter()
        .reduce(|a, b| a + b)
        .expect("input should not be empty");

    Ok(floor)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let increments = parse_input(input)?;

    let mut floor = 0;
    for (i, step) in increments.into_iter().enumerate() {
        floor += step;
        if floor < 0 {
            return Ok(i + 1);
        }
    }

    Err(SimpleError::new(String::from(
        "no solution found for part 2",
    )))
}

fn parse_input(input: &str) -> Result<Vec<i32>, SimpleError> {
    let line = crate::read_single_line(input)?;

    line.chars()
        .map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => Err(SimpleError::new(format!("unexpected char: {c}"))),
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(i32, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(0), solve_part_1("(())"));
        assert_eq!(Ok(0), solve_part_1("()()"));
        assert_eq!(Ok(3), solve_part_1("((("));
        assert_eq!(Ok(3), solve_part_1("(()(()("));
        assert_eq!(Ok(-1), solve_part_1("())"));
        assert_eq!(Ok(-3), solve_part_1(")))"));
        assert_eq!(Ok(-3), solve_part_1(")())())"));
    }

    #[test]
    fn test_invalid_input() {
        assert!(solve_part_1("").is_err());
        assert!(solve_part_1("asdf").is_err());
        assert!(solve_part_1("()() ()()").is_err());

        assert!(solve_part_2("()()()()()()").is_err());
        assert!(solve_part_2("(((((((").is_err());
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(1), solve_part_2(")"));
        assert_eq!(Ok(5), solve_part_2("()())"));
    }
}
