//! Day 25: Four-Dimensional Adventure
//! https://adventofcode.com/2018/day/25

use std::collections::HashSet;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    t: i64,
}

impl Point4D {
    fn new(x: i64, y: i64, z: i64, t: i64) -> Self {
        Self { x, y, z, t }
    }

    fn distance_to(&self, other: Point4D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() + (self.t - other.t).abs()
    }
}

#[derive(Debug, Clone)]
struct DisjointSet {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        let parents = (0..size).collect();
        let sizes = vec![1; size];
        Self { parents, sizes }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i_root = self.find(i);
        let j_root = self.find(j);
        if i_root != j_root {
            if self.sizes[i_root] > self.sizes[j_root] {
                self.sizes[i_root] += self.sizes[j_root];
                self.parents[j_root] = i_root;
            } else {
                self.sizes[j_root] += self.sizes[i_root];
                self.parents[i_root] = j_root;
            }
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parents[root] {
            let grandparent = self.parents[root];
            self.parents[root] = grandparent;
            root = grandparent;
        }
        root
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let points = parse_input(input)?;

    let mut disjoint_set = DisjointSet::new(points.len());

    for (i, &point) in points.iter().enumerate() {
        for (j, &other_point) in points.iter().enumerate().skip(i + 1) {
            if point.distance_to(other_point) <= 3 {
                disjoint_set.union(i, j);
            }
        }
    }

    let distinct_constellations: HashSet<_> = (0..points.len())
        .map(|i| disjoint_set.find(i))
        .collect();

    Ok(distinct_constellations.len())
}

fn parse_input(input: &str) -> Result<Vec<Point4D>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(',').collect();
        if split.len() != 4 {
            return Err(SimpleError::new(format!("invalid line format: {line}")));
        }

        let x = split[0].parse()?;
        let y = split[1].parse()?;
        let z = split[2].parse()?;
        let t = split[3].parse()?;

        Ok(Point4D::new(x, y, z, t))
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;

    Ok((solution1, String::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample25.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample25-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(2), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(4), solve_part_1(SAMPLE_INPUT_2));
    }
}