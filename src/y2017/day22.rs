//! Day 22: Sporifica Virus
//! https://adventofcode.com/2017/day/22

use crate::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::mem;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn next_state(&self) -> Self {
        match self {
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let mut points: HashSet<_> = parse_input(input)?.into_iter().collect();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 1;
    let mut infections = 0;
    for _ in 0..10000 {
        let p = Point::new(x, y);
        if points.contains(&p) {
            points.remove(&p);

            dx = -dx;
            mem::swap(&mut dx, &mut dy);
        } else {
            points.insert(p);
            infections += 1;

            dy = -dy;
            mem::swap(&mut dx, &mut dy);
        }

        x += dx;
        y += dy;
    }

    Ok(infections)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let mut point_states: HashMap<_, _> = parse_input(input)?
        .into_iter()
        .map(|point| (point, NodeState::Infected))
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 1;
    let mut infections = 0;
    for _ in 0..10_000_000 {
        let p = Point::new(x, y);
        let node_state = point_states.get(&p).copied().unwrap_or(NodeState::Clean);

        match node_state {
            NodeState::Clean => {
                dy = -dy;
                mem::swap(&mut dx, &mut dy);
            }
            NodeState::Infected => {
                dx = -dx;
                mem::swap(&mut dx, &mut dy);
            }
            NodeState::Flagged => {
                dx = -dx;
                dy = -dy;
            }
            NodeState::Weakened => {}
        }

        let next_state = node_state.next_state();
        match next_state {
            NodeState::Clean => {
                point_states.remove(&p);
            }
            NodeState::Infected => {
                infections += 1;
                point_states.insert(p, next_state);
            }
            _ => {
                point_states.insert(p, next_state);
            }
        }

        x += dx;
        y += dy;
    }

    Ok(infections)
}

fn parse_input(input: &str) -> Result<Vec<Point>, SimpleError> {
    if input.lines().next().is_none() {
        return Err(SimpleError::new(String::from("input is empty")));
    }

    let side_len = input.lines().next().unwrap().len() as i32;

    let mut points = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point::new(
                    (j as i32) - side_len / 2,
                    side_len / 2 - (i as i32),
                ));
            }
        }
    }

    Ok(points)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample22.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(5587), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    #[ignore] // Takes too long without release optimizations
    fn test_sample_input_part_2() {
        assert_eq!(Ok(2511944), solve_part_2(SAMPLE_INPUT));
    }
}
