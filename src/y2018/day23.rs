//! Day 23: Experimental Emergency Teleportation
//!
//! <https://adventofcode.com/2018/day/23>

use crate::SimpleError;
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    const ORIGIN: Self = Self { x: 0, y: 0, z: 0 };

    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Nanobot {
    position: Point,
    radius: i64,
}

impl Nanobot {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let (position, radius) = line
            .split_once(", ")
            .ok_or_else(|| SimpleError::new(format!("line has no ', ': {line}")))?;

        let position = &position[5..position.len() - 1];
        let split: Vec<_> = position.split(',').collect();
        if split.len() != 3 {
            return Err(SimpleError::new(format!(
                "position does not contain exactly 3 coordinates: {line}"
            )));
        }

        let x = split[0].parse()?;
        let y = split[1].parse()?;
        let z = split[2].parse()?;

        let radius = radius[2..].parse()?;

        Ok(Self {
            position: Point::new(x, y, z),
            radius,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn contains(self, n: i64) -> bool {
        n >= self.start && n < self.end
    }

    fn split(self) -> Vec<Self> {
        if self.end == self.start + 1 {
            vec![self]
        } else {
            let mid = (self.start + self.end) / 2;
            vec![Self::new(self.start, mid), Self::new(mid, self.end)]
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: Range,
    y: Range,
    z: Range,
}

impl Cube {
    fn new(x: Range, y: Range, z: Range) -> Self {
        Self { x, y, z }
    }

    fn is_single_point(&self) -> bool {
        self.x.end == self.x.start + 1
            && self.y.end == self.y.start + 1
            && self.z.end == self.z.start + 1
    }

    fn min_distance_to(&self, p: Point) -> i64 {
        let x_distance = if self.x.contains(p.x) {
            0
        } else {
            cmp::min((p.x - self.x.start).abs(), (p.x - self.x.end + 1).abs())
        };

        let y_distance = if self.y.contains(p.y) {
            0
        } else {
            cmp::min((p.y - self.y.start).abs(), (p.y - self.y.end + 1).abs())
        };

        let z_distance = if self.z.contains(p.z) {
            0
        } else {
            cmp::min((p.z - self.z.start).abs(), (p.z - self.z.end + 1).abs())
        };

        x_distance + y_distance + z_distance
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct HeapEntry {
    cube: Cube,
    intersecting_nanobots: usize,
}

impl HeapEntry {
    fn from_cube(cube: Cube, nanobots: &[Nanobot]) -> Self {
        let mut intersecting_nanobots = 0;
        for &nanobot in nanobots {
            if cube.min_distance_to(nanobot.position) <= nanobot.radius {
                intersecting_nanobots += 1;
            }
        }

        Self {
            cube,
            intersecting_nanobots,
        }
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by intersecting nanobots asc then distance to origin desc, which will get
        // reversed in the max heap
        self.intersecting_nanobots
            .cmp(&other.intersecting_nanobots)
            .then(
                self.cube
                    .min_distance_to(Point::ORIGIN)
                    .cmp(&other.cube.min_distance_to(Point::ORIGIN))
                    .reverse(),
            )
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let nanobots = parse_input(input)?;

    let strongest_nanobot = nanobots
        .iter()
        .copied()
        .max_by_key(|&nanobot| nanobot.radius)
        .unwrap();

    let mut in_range_count = 0;
    for &nanobot in &nanobots {
        if strongest_nanobot.position.distance_to(nanobot.position) <= strongest_nanobot.radius {
            in_range_count += 1;
        }
    }

    Ok(in_range_count)
}

fn solve_part_2(input: &str) -> Result<i64, SimpleError> {
    let nanobots = parse_input(input)?;

    let (min_x, max_x, min_y, max_y, min_z, max_z) = nanobots.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(min_x, max_x, min_y, max_y, min_z, max_z), &nanobot| {
            let p = nanobot.position;
            (
                cmp::min(min_x, p.x),
                cmp::max(max_x, p.x),
                cmp::min(min_y, p.y),
                cmp::max(max_y, p.y),
                cmp::min(min_z, p.z),
                cmp::max(max_z, p.z),
            )
        },
    );

    let x_range = Range::new(min_x, max_x + 1);
    let y_range = Range::new(min_y, max_y + 1);
    let z_range = Range::new(min_z, max_z + 1);

    let cube = Cube::new(x_range, y_range, z_range);

    let best_point = find_optimal_position(cube, &nanobots);

    Ok(best_point.x.abs() + best_point.y.abs() + best_point.z.abs())
}

fn find_optimal_position(cube: Cube, nanobots: &[Nanobot]) -> Point {
    let mut heap = BinaryHeap::new();
    heap.push(HeapEntry::from_cube(cube, nanobots));

    let mut best_so_far = 0;
    let mut best_point_so_far = Point::ORIGIN;

    while let Some(HeapEntry {
        cube,
        intersecting_nanobots,
    }) = heap.pop()
    {
        if cube.is_single_point() {
            let p = Point::new(cube.x.start, cube.y.start, cube.z.start);
            if intersecting_nanobots > best_so_far
                || (intersecting_nanobots == best_so_far
                    && p.distance_to(Point::ORIGIN) < best_point_so_far.distance_to(Point::ORIGIN))
            {
                best_so_far = intersecting_nanobots;
                best_point_so_far = p;
            }

            continue;
        }

        if intersecting_nanobots < best_so_far {
            continue;
        }

        for xs in cube.x.split() {
            for ys in cube.y.split() {
                for zs in cube.z.split() {
                    let new_cube = Cube::new(xs, ys, zs);
                    heap.push(HeapEntry::from_cube(new_cube, nanobots));
                }
            }
        }
    }

    best_point_so_far
}

fn parse_input(input: &str) -> Result<Vec<Nanobot>, SimpleError> {
    if input.lines().count() == 0 {
        return Err(SimpleError::new(String::from("input has no lines")));
    }

    input.lines().map(Nanobot::from_line).collect()
}

pub fn solve(input: &str) -> Result<(usize, i64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample23.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample23-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(7), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(36), solve_part_2(SAMPLE_INPUT_2));
    }
}
