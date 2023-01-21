//! Day 23: Experimental Emergency Teleportation
//! https://adventofcode.com/2018/day/23

use std::cmp;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
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
        let (position, radius) = line.split_once(", ").ok_or_else(
            || SimpleError::new(format!("line has no ', ': {line}"))
        )?;

        let position = &position[5..position.len() - 1];
        let split: Vec<_> = position.split(',').collect();
        if split.len() != 3 {
            return Err(SimpleError::new(format!("position does not contain exactly 3 coordinates: {line}")));
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

    fn contains(&self, n: i64) -> bool {
        n >= self.start && n < self.end
    }

    fn split(&self) -> Vec<Self> {
        if self.end == self.start + 1 {
            vec![*self]
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

    fn distance_to(&self, p: Point) -> i64 {
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

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let nanobots = parse_input(input)?;

    let strongest_nanobot = nanobots.iter().copied()
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
        }
    );

    let x_range = Range::new(min_x, max_x + 1);
    let y_range = Range::new(min_y, max_y + 1);
    let z_range = Range::new(min_z, max_z + 1);

    let cube = Cube::new(x_range, y_range, z_range);

    let (best_point, _) = find_optimal_position(cube, &nanobots, &mut 0);

    Ok(best_point.x.abs() + best_point.y.abs() + best_point.z.abs())
}

fn find_optimal_position(cube: Cube, nanobots: &[Nanobot], best_so_far: &mut usize) -> (Point, usize) {
    if cube.x.end == cube.x.start + 1 && cube.y.end == cube.y.start + 1 && cube.z.end == cube.z.start + 1 {
        let mut nanobot_overlap_count = 0;
        let p = Point::new(cube.x.start, cube.y.start, cube.z.start);
        for &nanobot in nanobots {
            if p.distance_to(nanobot.position) <= nanobot.radius {
                nanobot_overlap_count += 1;
            }
        }

        *best_so_far = cmp::max(*best_so_far, nanobot_overlap_count);
        return (Point::new(cube.x.start, cube.y.start, cube.z.start), nanobot_overlap_count);
    }

    let x_split = cube.x.split();
    let y_split = cube.y.split();
    let z_split = cube.z.split();
    let mut cubes_to_search = Vec::new();
    for &xs in &x_split {
        for &ys in &y_split {
            for &zs in &z_split {
                let new_cube = Cube::new(xs, ys, zs);
                let mut overlap_count = 0;
                for &nanobot in nanobots {
                    if new_cube.distance_to(nanobot.position) <= nanobot.radius {
                        overlap_count += 1;
                    }
                }

                if overlap_count > *best_so_far {
                    cubes_to_search.push(new_cube);
                }
            }
        }
    }

    if cubes_to_search.is_empty() {
        return (Point::new(0, 0, 0), 0);
    }

    cubes_to_search.sort_by_key(|&cube| cube.distance_to(Point::new(0, 0, 0)));

    let mut best_overlap_count = 0;
    let mut best_point = Point::new(0, 0, 0);
    for &cube in &cubes_to_search {
        let (p, overlap_count) = find_optimal_position(cube, nanobots, best_so_far);
        if overlap_count > best_overlap_count {
            best_overlap_count = overlap_count;
            best_point = p;
        } else if overlap_count == best_overlap_count && p.x.abs() + p.y.abs() + p.z.abs() < best_point.x.abs() + best_point.y.abs() + best_point.z.abs()  {
            best_point = p;
        }
    }

    (best_point, best_overlap_count)
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