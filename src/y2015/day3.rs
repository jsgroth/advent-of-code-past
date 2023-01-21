//! Day 3: Perfectly Spherical Houses in a Vacuum
//! https://adventofcode.com/2015/day/3

use crate::SimpleError;
use std::collections::HashSet;
use std::error::Error;
use std::mem;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    i: i32,
    j: i32,
}

impl Point {
    fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let line = crate::read_single_line(input)?;

    let mut i = 0;
    let mut j = 0;

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point::new(0, 0));

    for c in line.chars() {
        match c {
            '^' => i -= 1,
            'v' => i += 1,
            '<' => j -= 1,
            '>' => j += 1,
            _ => return Err(SimpleError::new(format!("unexpected char: {c}"))),
        }

        visited.insert(Point::new(i, j));
    }

    Ok(visited.len())
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let line = crate::read_single_line(input)?;

    let mut i1 = 0;
    let mut j1 = 0;
    let mut i2 = 0;
    let mut j2 = 0;

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point::new(0, 0));

    for c in line.chars() {
        match c {
            '^' => i1 -= 1,
            'v' => i1 += 1,
            '<' => j1 -= 1,
            '>' => j1 += 1,
            _ => return Err(SimpleError::new(format!("unexpected char: {c}"))),
        }

        visited.insert(Point::new(i1, j1));

        mem::swap(&mut i1, &mut i2);
        mem::swap(&mut j1, &mut j2);
    }

    Ok(visited.len())
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(2), solve_part_1(">"));
        assert_eq!(Ok(4), solve_part_1("^>v<"));
        assert_eq!(Ok(2), solve_part_1("^v^v^v^v^v^v^v^v"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(3), solve_part_2("^v"));
        assert_eq!(Ok(3), solve_part_2("^>v<"));
        assert_eq!(Ok(11), solve_part_2("^v^v^v^v^v"));
    }

    #[test]
    fn test_invalid_input() {
        assert!(solve_part_1("").is_err());
        assert!(solve_part_1("asdfvvvv").is_err());

        assert!(solve_part_2("").is_err());
        assert!(solve_part_2("asdfvvvv").is_err());
    }
}
