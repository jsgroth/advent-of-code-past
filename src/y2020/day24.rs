//! Day 24: Lobby Layout
//! https://adventofcode.com/2020/day/24

use crate::SimpleError;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HexDirection {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl HexDirection {
    const ALL: [Self; 6] = [
        Self::East,
        Self::Southeast,
        Self::Southwest,
        Self::West,
        Self::Northwest,
        Self::Northeast,
    ];

    fn len_as_str(&self) -> usize {
        match self {
            Self::East | Self::West => 1,
            Self::Southeast | Self::Southwest | Self::Northwest | Self::Northeast => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct HexPoint {
    x: i32,
    y: i32,
}

impl HexPoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<HexDirection> for HexPoint {
    type Output = Self;

    fn add(self, rhs: HexDirection) -> Self::Output {
        match rhs {
            HexDirection::East => Self::new(self.x + 2, self.y),
            HexDirection::Southeast => Self::new(self.x + 1, self.y - 1),
            HexDirection::Southwest => Self::new(self.x - 1, self.y - 1),
            HexDirection::West => Self::new(self.x - 2, self.y),
            HexDirection::Northwest => Self::new(self.x - 1, self.y + 1),
            HexDirection::Northeast => Self::new(self.x + 1, self.y + 1),
        }
    }
}

impl AddAssign<HexDirection> for HexPoint {
    fn add_assign(&mut self, rhs: HexDirection) {
        *self = *self + rhs;
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let black_points = find_black_points(input)?;

    Ok(black_points.len())
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let mut black_points = find_black_points(input)?;

    for _ in 0..100 {
        black_points = simulate_iteration(&black_points);
    }

    Ok(black_points.len())
}

fn simulate_iteration(black_points: &HashSet<HexPoint>) -> HashSet<HexPoint> {
    let mut next_points = HashSet::new();

    let (min_x, max_x, min_y, max_y) = black_points.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), point| {
            (
                cmp::min(min_x, point.x),
                cmp::max(max_x, point.x),
                cmp::min(min_y, point.y),
                cmp::max(max_y, point.y),
            )
        },
    );

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let point = HexPoint::new(x, y);
            let is_black = black_points.contains(&point);

            let neighbor_count = HexDirection::ALL
                .iter()
                .copied()
                .filter(|&direction| black_points.contains(&(point + direction)))
                .count();

            if neighbor_count == 2 || (is_black && neighbor_count == 1) {
                next_points.insert(point);
            }
        }
    }

    next_points
}

fn find_black_points(input: &str) -> Result<HashSet<HexPoint>, SimpleError> {
    let mut black_points = HashSet::new();

    for line in input.lines() {
        let mut position = HexPoint::new(0, 0);

        let mut remaining = line;
        while !remaining.is_empty() {
            let direction = match remaining.chars().next().unwrap() {
                'n' => match &remaining[..2] {
                    "ne" => HexDirection::Northeast,
                    "nw" => HexDirection::Northwest,
                    _ => {
                        return Err(SimpleError::new(format!(
                            "invalid n* sequence in line: {line}"
                        )))
                    }
                },
                's' => match &remaining[..2] {
                    "se" => HexDirection::Southeast,
                    "sw" => HexDirection::Southwest,
                    _ => {
                        return Err(SimpleError::new(format!(
                            "invalid s* sequence in line: {line}"
                        )))
                    }
                },
                'w' => HexDirection::West,
                'e' => HexDirection::East,
                _ => {
                    return Err(SimpleError::new(format!(
                        "invalid direction in line: {line}"
                    )))
                }
            };

            position += direction;

            remaining = &remaining[direction.len_as_str()..];
        }

        if !black_points.insert(position) {
            black_points.remove(&position);
        }
    }

    Ok(black_points)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample24.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(10), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(2208), solve_part_2(SAMPLE_INPUT));
    }
}
