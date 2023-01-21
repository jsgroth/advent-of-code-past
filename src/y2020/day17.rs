//! Day 17: Conway Cubes
//! https://adventofcode.com/2020/day/17

use crate::SimpleError;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;

trait Point: Eq + Hash + Copy {
    fn from_2d(x: i32, y: i32) -> Self;

    fn minimum_coordinates<I: Iterator<Item = Self>>(iter: I) -> Self;

    fn maximum_coordinates<I: Iterator<Item = Self>>(iter: I) -> Self;

    // Iterator over all points in the range of [min - 1, max + 1] over each coordinate
    fn points_including_range(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>>;

    fn adjacent_points(&self) -> Box<dyn Iterator<Item = Self>>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Point for Point3D {
    fn from_2d(x: i32, y: i32) -> Self {
        Self::new(x, y, 0)
    }

    fn minimum_coordinates<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (min_x, min_y, min_z) = iter.fold(
            (i32::MAX, i32::MAX, i32::MAX),
            |(min_x, min_y, min_z), point| {
                (
                    cmp::min(min_x, point.x),
                    cmp::min(min_y, point.y),
                    cmp::min(min_z, point.z),
                )
            },
        );
        Self::new(min_x, min_y, min_z)
    }

    fn maximum_coordinates<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (max_x, max_y, max_z) = iter.fold(
            (i32::MIN, i32::MIN, i32::MIN),
            |(max_x, max_y, max_z), point| {
                (
                    cmp::max(max_x, point.x),
                    cmp::max(max_y, point.y),
                    cmp::max(max_z, point.z),
                )
            },
        );
        Self::new(max_x, max_y, max_z)
    }

    fn points_including_range(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        let iter = ((min.x - 1)..=(max.x + 1)).flat_map(move |x| {
            ((min.y - 1)..=(max.y + 1))
                .flat_map(move |y| ((min.z - 1)..=(max.z + 1)).map(move |z| Self::new(x, y, z)))
        });
        Box::new(iter)
    }

    fn adjacent_points(&self) -> Box<dyn Iterator<Item = Self>> {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let iter = (-1..=1)
            .flat_map(|dx| (-1..=1).flat_map(move |dy| (-1..=1).map(move |dz| (dx, dy, dz))))
            .filter(|&(dx, dy, dz)| dx != 0 || dy != 0 || dz != 0)
            .map(move |(dx, dy, dz)| Self::new(x + dx, y + dy, z + dz));
        Box::new(iter)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point4D {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }
}

impl Point for Point4D {
    fn from_2d(x: i32, y: i32) -> Self {
        Self::new(x, y, 0, 0)
    }

    fn minimum_coordinates<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (min_x, min_y, min_z, min_w) = iter.fold(
            (i32::MAX, i32::MAX, i32::MAX, i32::MAX),
            |(min_x, min_y, min_z, min_w), point| {
                (
                    cmp::min(min_x, point.x),
                    cmp::min(min_y, point.y),
                    cmp::min(min_z, point.z),
                    cmp::min(min_w, point.w),
                )
            },
        );
        Self::new(min_x, min_y, min_z, min_w)
    }

    fn maximum_coordinates<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (max_x, max_y, max_z, max_w) = iter.fold(
            (i32::MIN, i32::MIN, i32::MIN, i32::MIN),
            |(max_x, max_y, max_z, max_w), point| {
                (
                    cmp::max(max_x, point.x),
                    cmp::max(max_y, point.y),
                    cmp::max(max_z, point.z),
                    cmp::max(max_w, point.w),
                )
            },
        );
        Self::new(max_x, max_y, max_z, max_w)
    }

    fn points_including_range(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        let iter = ((min.x - 1)..=(max.x + 1)).flat_map(move |x| {
            ((min.y - 1)..=(max.y + 1)).flat_map(move |y| {
                ((min.z - 1)..=(max.z + 1)).flat_map(move |z| {
                    ((min.w - 1)..=(max.w + 1)).map(move |w| Self::new(x, y, z, w))
                })
            })
        });
        Box::new(iter)
    }

    fn adjacent_points(&self) -> Box<dyn Iterator<Item = Self>> {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;

        let iter = (-1..=1)
            .flat_map(|dx| {
                (-1..=1).flat_map(move |dy| {
                    (-1..=1).flat_map(move |dz| (-1..=1).map(move |dw| (dx, dy, dz, dw)))
                })
            })
            .filter(|&(dx, dy, dz, dw)| dx != 0 || dy != 0 || dz != 0 || dw != 0)
            .map(move |(dx, dy, dz, dw)| Self::new(x + dx, y + dy, z + dz, w + dw));
        Box::new(iter)
    }
}

fn solve_part<P: Point>(input: &str) -> Result<usize, SimpleError> {
    let mut active_points = parse_input(input)?;

    for _ in 0..6 {
        let minimums = P::minimum_coordinates(active_points.iter().copied());
        let maximums = P::maximum_coordinates(active_points.iter().copied());

        let mut next_active_points = HashSet::new();

        for point in P::points_including_range(minimums, maximums) {
            let neighbor_count = point
                .adjacent_points()
                .filter(|&adj_point| active_points.contains(&adj_point))
                .count();

            if neighbor_count == 3 || (active_points.contains(&point) && neighbor_count == 2) {
                next_active_points.insert(point);
            }
        }

        active_points = next_active_points;
    }

    Ok(active_points.len())
}

fn parse_input<P: Point>(input: &str) -> Result<HashSet<P>, SimpleError> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| match c {
                    '#' => Some(Ok(P::from_2d(j as i32, i as i32))),
                    '.' => None,
                    _ => Some(Err(SimpleError::new(format!("invalid char: {c}")))),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part::<Point3D>(input)?;
    let solution2 = solve_part::<Point4D>(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample17.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(112), solve_part::<Point3D>(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(848), solve_part::<Point4D>(SAMPLE_INPUT));
    }
}
