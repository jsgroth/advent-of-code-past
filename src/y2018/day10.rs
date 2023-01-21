//! Day 10: The Stars Align
//!
//! <https://adventofcode.com/2018/day/10>

use crate::SimpleError;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    position: Coords,
    velocity: Coords,
}

impl Point {
    fn tick(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

fn solve_both_parts(input: &str) -> Result<(String, usize), SimpleError> {
    let mut points = parse_input(input)?;

    for i in 0.. {
        let (min_x, max_x, min_y, max_y) = points.iter().fold(
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
            |(min_x, max_x, min_y, max_y), point| {
                (
                    cmp::min(min_x, point.position.x),
                    cmp::max(max_x, point.position.x),
                    cmp::min(min_y, point.position.y),
                    cmp::max(max_y, point.position.y),
                )
            },
        );

        if max_x - min_x <= 70 && max_y - min_y <= 10 {
            let positions: HashSet<_> = points.iter().map(|point| point.position).collect();

            let mut word_in_the_stars = String::new();
            for y in min_y..=max_y {
                if y != min_y {
                    word_in_the_stars.push('\n');
                }

                for x in min_x..=max_x {
                    if positions.contains(&Coords::new(x, y)) {
                        word_in_the_stars.push('#');
                    } else {
                        word_in_the_stars.push(' ');
                    }
                }
            }

            return Ok((word_in_the_stars, i));
        }

        for point in &mut points {
            point.tick();
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<Vec<Point>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let position_end_index = line
                .chars()
                .position(|c| c == '>')
                .ok_or_else(|| SimpleError::new(format!("line has no '>': {line}")))?;

            let position = &line["position=".len()..position_end_index + 1];
            let velocity = &line[(position_end_index + 2 + "velocity=".len())..];

            let position = parse_coords(position)?;
            let velocity = parse_coords(velocity)?;

            Ok(Point { position, velocity })
        })
        .collect()
}

fn parse_coords(s: &str) -> Result<Coords, SimpleError> {
    let stripped: String = s[1..s.len() - 1].chars().filter(|&c| c != ' ').collect();
    let (x, y) = stripped
        .split_once(',')
        .ok_or_else(|| SimpleError::new(format!("invalid coords string: {s}")))?;

    Ok(Coords::new(x.parse()?, y.parse()?))
}

pub fn solve(input: &str) -> Result<(String, usize), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}
