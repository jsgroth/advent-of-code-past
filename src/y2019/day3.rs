//! Day 3: Crossed Wires
//! https://adventofcode.com/2019/day/3

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, SimpleError> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            _ => Err(SimpleError::new(format!("invalid direction char: {c}")))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PathPart {
    direction: Direction,
    distance: i64,
}

fn solve_part_1(input: &str) -> Result<i64, SimpleError> {
    let (first_path, second_path) = parse_input(input)?;

    let first_touched_points: HashSet<_> = build_touched_points(&first_path).keys().copied().collect();
    let second_touched_points: HashSet<_> = build_touched_points(&second_path).keys().copied().collect();

    let min_by_distance = first_touched_points.intersection(&second_touched_points)
        .copied()
        .min_by_key(|&point| point.x.abs() + point.y.abs())
        .unwrap();

    Ok(min_by_distance.x.abs() + min_by_distance.y.abs())
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let (first_path, second_path) = parse_input(input)?;

    let first_touched_points = build_touched_points(&first_path);
    let second_touched_points = build_touched_points(&second_path);

    let intersection: Vec<_> = first_touched_points.keys().copied()
        .filter(|&point| second_touched_points.contains_key(&point))
        .collect();

    let mut min_steps = usize::MAX;
    for &point in &intersection {
        let steps = first_touched_points.get(&point).copied().unwrap() + second_touched_points.get(&point).copied().unwrap();
        min_steps = cmp::min(min_steps, steps);
    }

    Ok(min_steps)
}

fn build_touched_points(path: &[PathPart]) -> HashMap<Point, usize> {
    let mut position = Point::new(0, 0);
    let mut touched_points = HashMap::new();
    let mut steps = 0;
    for &path_part in path {
        for _ in 0..path_part.distance {
            match path_part.direction {
                Direction::Left => position.x -= 1,
                Direction::Right => position.x += 1,
                Direction::Up => position.y += 1,
                Direction::Down => position.y -= 1,
            }

            steps += 1;

            if !touched_points.contains_key(&position) {
                touched_points.insert(position, steps);
            }
        }
    }

    touched_points
}

pub fn solve(input: &str) -> Result<(i64, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

fn parse_input(input: &str) -> Result<(Vec<PathPart>, Vec<PathPart>), SimpleError> {
    let first_line = crate::read_single_line(input)?;
    let second_line = input.lines().skip(1).next().ok_or(
        SimpleError::new(String::from("input only has one line"))
    )?;

    let first_path = parse_path(first_line)?;
    let second_path = parse_path(second_line)?;

    Ok((first_path, second_path))
}

fn parse_path(line: &str) -> Result<Vec<PathPart>, SimpleError> {
    line.split(',').map(|part| {
        if part.len() < 2 {
            return Err(SimpleError::new(format!("path part is too short: {part}")));
        }

        let direction = Direction::from_char(part.chars().next().unwrap())?;
        let distance = part[1..].parse()?;

        Ok(PathPart { direction, distance })
    })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample3.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample3-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("sample_input/sample3-3.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(6), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(159), solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(Ok(135), solve_part_1(SAMPLE_INPUT_3));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(30), solve_part_2(SAMPLE_INPUT_1));
        assert_eq!(Ok(610), solve_part_2(SAMPLE_INPUT_2));
        assert_eq!(Ok(410), solve_part_2(SAMPLE_INPUT_3));
    }
}