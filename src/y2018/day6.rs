//! Day 6: Chronal Coordinates
//!
//! <https://adventofcode.com/2018/day/6>

use crate::SimpleError;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let points = parse_input(input)?;

    let (max_x, max_y) = get_maximums(&points);

    let rows = (max_y + 1) as usize;
    let cols = (max_x + 1) as usize;

    let mut grid = vec![vec![0; cols]; rows];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, value) in row.iter_mut().enumerate() {
            let p = Point::new(x as i32, y as i32);
            let distances: Vec<_> = points
                .iter()
                .map(|other_p| other_p.distance_to(p))
                .collect();

            let min_distance = distances.iter().copied().min().unwrap();
            let min_distance_count = distances.iter().filter(|&&n| n == min_distance).count();
            if min_distance_count > 1 {
                continue;
            }

            let region = 1 + distances.iter().position(|&n| n == min_distance).unwrap();
            *value = region;
        }
    }

    let mut regions_touching_edge = HashSet::new();
    for x in 0..cols {
        if grid[0][x] != 0 {
            regions_touching_edge.insert(grid[0][x]);
        }
        if grid[rows - 1][x] != 0 {
            regions_touching_edge.insert(grid[rows - 1][x]);
        }
    }
    for row in &grid {
        if row[0] != 0 {
            regions_touching_edge.insert(row[0]);
        }
        if row[cols - 1] != 0 {
            regions_touching_edge.insert(row[cols - 1]);
        }
    }

    let mut count_by_region = HashMap::new();
    for row in &grid {
        for &region in row {
            if region != 0 && !regions_touching_edge.contains(&region) {
                if let Some(value) = count_by_region.get_mut(&region) {
                    *value += 1;
                } else {
                    count_by_region.insert(region, 1);
                }
            }
        }
    }

    let max_region_count = count_by_region.into_values().max().unwrap();

    Ok(max_region_count)
}

fn solve_part_2(input: &str, distance_limit: i32) -> Result<usize, SimpleError> {
    let points = parse_input(input)?;

    let (max_x, max_y) = get_maximums(&points);

    let rows = max_y + 1;
    let cols = max_x + 1;

    let mut safe_count = 0;
    for x in 0..cols {
        for y in 0..rows {
            let p = Point::new(x, y);
            let total_distance: i32 = points.iter().map(|other_p| other_p.distance_to(p)).sum();
            if total_distance < distance_limit {
                safe_count += 1;
            }
        }
    }

    Ok(safe_count)
}

fn get_maximums(points: &[Point]) -> (i32, i32) {
    points
        .iter()
        .copied()
        .fold((i32::MIN, i32::MIN), |(max_x, max_y), point| {
            (cmp::max(max_x, point.x), cmp::max(max_y, point.y))
        })
}

fn parse_input(input: &str) -> Result<Vec<Point>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(", ")
                .ok_or_else(|| SimpleError::new(format!("line has no ', ': {line}")))?;

            Ok(Point::new(x.parse()?, y.parse()?))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input, 10000)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample6.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(17), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(16), solve_part_2(SAMPLE_INPUT, 32));
    }
}
