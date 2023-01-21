//! Day 6: Probably a Fire Hazard
//! https://adventofcode.com/2015/day/6

use std::cmp::Ordering;
use std::error::Error;
use std::ops::Not;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ordering => ordering,
        }
    }
}

#[derive(Debug)]
enum ActionType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Action {
    action_type: ActionType,
    p1: Point,
    p2: Point,
}

impl Action {
    fn new(action_type: ActionType, p1: Point, p2: Point) -> Self {
        Self { action_type, p1, p2 }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let actions = parse_input(input)?;

    let mut grid = vec![vec![false; 1000]; 1000];

    for action in &actions {
        for row in &mut grid[action.p1.x..=action.p2.x] {
            for value in &mut row[action.p1.y..=action.p2.y] {
                *value = match action.action_type {
                    ActionType::TurnOn => true,
                    ActionType::TurnOff => false,
                    ActionType::Toggle => value.not(),
                }
            }
        }
    }

    let on = grid.iter().map(|row| {
        row.iter().filter(|b| **b).count()
    })
        .sum();

    Ok(on)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let actions = parse_input(input)?;

    let mut grid = vec![vec![0_u32; 1000]; 1000];

    for action in &actions {
        for row in &mut grid[action.p1.x..=action.p2.x] {
            for value in &mut row[action.p1.y..=action.p2.y] {
                match action.action_type {
                    ActionType::TurnOn => *value += 1,
                    ActionType::TurnOff => *value = value.saturating_sub(1),
                    ActionType::Toggle => *value += 2,
                }
            }
        }
    }

    let total_brightness = grid.into_iter().map(|row| {
        row.into_iter().sum::<u32>()
    })
        .sum();

    Ok(total_brightness)
}

fn parse_input(input: &str) -> Result<Vec<Action>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(' ').collect();

        match split.as_slice() {
            ["turn", "on", p1, "through", p2] => Ok(Action::new(
                ActionType::TurnOn, parse_point(p1)?, parse_point(p2)?
            )),
            ["turn", "off", p1, "through", p2] => Ok(Action::new(
                ActionType::TurnOff, parse_point(p1)?, parse_point(p2)?
            )),
            ["toggle", p1, "through", p2] => Ok(Action::new(
                ActionType::Toggle, parse_point(p1)?, parse_point(p2)?
            )),
            _ => Err(SimpleError::new(format!("unrecognized action: {line}")))
        }
    })
        .collect()
}

fn parse_point(s: &str) -> Result<Point, SimpleError> {
    let (x, y) = match s.split_once(',') {
        Some((x, y)) => (x, y),
        None => return Err(SimpleError::new(format!("point string does not have a comma: {s}")))
    };

    Ok(Point::new(x.parse()?, y.parse()?))
}

pub fn solve(input: &str) -> Result<(usize, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        assert!(solve_part_1("asdf").is_err());
        assert!(solve_part_1("turn on 1 through 5").is_err());
        assert!(solve_part_1("toggle 1,1 2,2 3,3").is_err());
    }
}