//! Day 13: A Maze of Twisty Little Cubicles
//! https://adventofcode.com/2016/day/13

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_distance_to(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn is_wall(&self, favorite_number: i64) -> bool {
        let &Point { x, y } = self;
        (x * x + 3 * x + 2 * x * y + y + y * y + favorite_number).count_ones() % 2 == 1
    }

    fn adjacent_points(&self, favorite_number: i64) -> Vec<Point> {
        let mut points = Vec::new();

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = self.x + dx;
            let y = self.y + dy;
            if x >= 0 && y >= 0 {
                let p = Point::new(x, y);
                if !p.is_wall(favorite_number) {
                    points.push(p);
                }
            }
        }

        points
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AStarHeapEntry {
    p: Point,
    steps: usize,
    best_solution_lower_bound: i64,
}

impl AStarHeapEntry {
    fn new(p: Point, steps: usize, target: &Point) -> Self {
        let distance_to_target = p.manhattan_distance_to(target);
        Self {
            p,
            steps,
            best_solution_lower_bound: (steps as i64) + distance_to_target,
        }
    }
}

impl PartialOrd<Self> for AStarHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.best_solution_lower_bound.cmp(&other.best_solution_lower_bound).reverse()
    }
}

fn solve_part_1(input: &str, target_x: i64, target_y: i64) -> Result<usize, SimpleError> {
    let favorite_number: i64 = crate::read_single_line(input)?.parse()?;

    let target = Point::new(target_x, target_y);

    let start = Point::new(1, 1);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut heap = BinaryHeap::new();
    heap.push(AStarHeapEntry::new(start, 0, &target));

    while !heap.is_empty() {
        let AStarHeapEntry { p, steps, .. } = heap.pop().unwrap();

        for adjacent_point in p.adjacent_points(favorite_number) {
            if adjacent_point == target {
                return Ok(steps + 1);
            }

            if !visited.contains(&adjacent_point) {
                visited.insert(adjacent_point);
                heap.push(AStarHeapEntry::new(adjacent_point, steps + 1, &target));
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let favorite_number: i64 = crate::read_single_line(input)?.parse()?;

    let start = Point::new(1, 1);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (p, steps) = queue.pop_front().unwrap();

        if steps == 50 {
            continue;
        }

        for adjacent_point in p.adjacent_points(favorite_number) {
            if !visited.contains(&adjacent_point) {
                visited.insert(adjacent_point);
                queue.push_back((adjacent_point, steps + 1));
            }
        }
    }

    Ok(visited.len())
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 31, 39)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(11), solve_part_1("10", 7, 4));
    }
}