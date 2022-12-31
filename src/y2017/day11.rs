//! Day 11: Hex Ed
//! https://adventofcode.com/2017/day/11

use std::cmp;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

impl Direction {
    fn from_str(s: &str) -> Result<Self, SimpleError> {
        match s {
            "n" => Ok(Self::North),
            "ne" => Ok(Self::Northeast),
            "se" => Ok(Self::Southeast),
            "s" => Ok(Self::South),
            "sw" => Ok(Self::Southwest),
            "nw" => Ok(Self::Northwest),
            _ => Err(SimpleError::new(format!("invalid direction string: {s}")))
        }
    }
}

fn solve_both_parts(input: &str) -> Result<(i32, i32), SimpleError> {
    let directions = parse_input(input)?;

    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut max_distance = 0;
    for direction in directions {
        let (dx, dy) = match direction {
            Direction::North => (0, 2),
            Direction::Northeast => (1, 1),
            Direction::Southeast => (1, -1),
            Direction::South => (0, -2),
            Direction::Southwest => (-1, -1),
            Direction::Northwest => (-1, 1),
        };
        x += dx;
        y += dy;

        max_distance = cmp::max(max_distance, distance_from_origin(x, y));
    }

    Ok((distance_from_origin(x, y), max_distance))
}

fn distance_from_origin(x: i32, y: i32) -> i32 {
    let x = x.abs();
    let y = y.abs();
    x + (y - cmp::min(x, y)) / 2
}

fn parse_input(input: &str) -> Result<Vec<Direction>, SimpleError> {
    crate::read_single_line(input)?
        .split(',')
        .map(Direction::from_str)
        .collect()
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok((3, 3)), solve_both_parts("ne,ne,ne"));
        assert_eq!(Ok((0, 2)), solve_both_parts("ne,ne,sw,sw"));
        assert_eq!(Ok((2, 2)), solve_both_parts("ne,ne,s,s"));
        assert_eq!(Ok((3, 3)), solve_both_parts("se,sw,se,sw,sw"));
    }
}