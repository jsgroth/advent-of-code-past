//! Day 1: No Time for a Taxicab
//! https://adventofcode.com/2016/day/1

use std::collections::HashSet;
use std::error::Error;
use std::mem;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

impl Rotation {
    fn apply(&self, dx: &mut i32, dy: &mut i32) {
        match self {
            Self::Left => {
                *dy = -(*dy);
                mem::swap(dx, dy);
            }
            Self::Right => {
                *dx = -(*dx);
                mem::swap(dx, dy);
            }
        }
    }
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let instructions = parse_input(input)?;

    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 1;
    for &(rotation, distance) in &instructions {
        rotation.apply(&mut dx, &mut dy);

        x += distance * dx;
        y += distance * dy;
    }

    Ok(x.abs() + y.abs())
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let instructions = parse_input(input)?;

    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 1;
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for &(rotation, distance) in &instructions {
        rotation.apply(&mut dx, &mut dy);

        for _ in 0..distance {
            x += dx;
            y += dy;

            if !visited.insert((x, y)) {
                return Ok(x.abs() + y.abs());
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<Vec<(Rotation, i32)>, SimpleError> {
    let line = crate::read_single_line(input)?;

    line.split(", ").map(|word| {
        let direction = match word.chars().next() {
            Some('L') => Rotation::Left,
            Some('R') => Rotation::Right,
            _ => return Err(SimpleError::new(format!("invalid word format: {word}")))
        };
        let distance: i32 = word[1..].parse()?;

        Ok((direction, distance))
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(5), solve_part_1("R2, L3"));
        assert_eq!(Ok(2), solve_part_1("R2, R2, R2"));
        assert_eq!(Ok(12), solve_part_1("R5, L5, R5, R3"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(4), solve_part_2("R8, R4, R4, R8"));
    }
}