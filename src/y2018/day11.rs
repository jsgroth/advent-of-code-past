//! Day 11: Chronal Charge
//! https://adventofcode.com/2018/day/11

use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Part2Solution {
    point: Point,
    size: usize,
}

impl Display for Part2Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.point, self.size)
    }
}

fn solve_part_1(input: &str) -> Result<Point, SimpleError> {
    let serial_number: i32 = crate::read_single_line(input)?.parse()?;

    let power_levels = compute_power_levels(serial_number);

    let mut max_power_level = i32::MIN;
    let mut max_power_level_point = Point::new(0, 0);
    for i in 0..297 {
        for j in 0..297 {
            let mut total_power_level = 0;
            for k in 0..3 {
                for l in 0..3 {
                    total_power_level += power_levels[i + k][j + l];
                }
            }

            if total_power_level > max_power_level {
                max_power_level = total_power_level;
                max_power_level_point = Point::new(j as i32 + 1, i as i32 + 1);
            }
        }
    }

    Ok(max_power_level_point)
}

fn solve_part_2(input: &str) -> Result<Part2Solution, SimpleError> {
    let serial_number: i32 = crate::read_single_line(input)?.parse()?;

    let power_levels = compute_power_levels(serial_number);

    let mut max_power_level = i32::MIN;
    let mut max_power_level_soln = Part2Solution { point: Point::new(0, 0), size: 0 };
    for i in 0..300 {
        for j in 0..300 {
            let power_level = power_levels[i][j];
            if power_level > max_power_level {
                max_power_level = power_level;
                max_power_level_soln = Part2Solution {
                    point: Point::new(j as i32 + 1, i as i32 + 1),
                    size: 1,
                };
            }
        }
    }

    let mut square_power_levels = power_levels.clone();

    for side_len in 2..=300 {
        for i in 0..300 - side_len {
            for j in 0..300 - side_len {
                let mut total_power_level = power_levels[i][j] + square_power_levels[i + 1][j + 1];

                for k in 1..side_len {
                    total_power_level += power_levels[i][j + k];
                    total_power_level += power_levels[i + k][j];
                }

                if total_power_level > max_power_level {
                    max_power_level = total_power_level;
                    max_power_level_soln = Part2Solution {
                        point: Point::new(j as i32 + 1, i as i32 + 1),
                        size: side_len,
                    };
                }

                square_power_levels[i][j] = total_power_level;
            }
        }
    }

    Ok(max_power_level_soln)
}

fn compute_power_levels(serial_number: i32) -> [[i32; 300]; 300] {
    let mut power_levels = [[0; 300]; 300];
    for i in 0..300 {
        for j in 0..300 {
            let x = j + 1;
            let y = i + 1;

            let power_level = ((x + 10) * y + serial_number) * (x + 10);
            let hundreds_digit = (power_level / 100) % 10;
            power_levels[i as usize][j as usize] = hundreds_digit - 5;
        }
    }

    power_levels
}

pub(crate) fn solve(input: &str) -> Result<(Point, Part2Solution), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(Point::new(33, 45)), solve_part_1("18"));
        assert_eq!(Ok(Point::new(21, 61)), solve_part_1("42"));
    }

    #[test]
    #[ignore] // This is fairly fast with release optimizations but takes >10 seconds without
    fn test_sample_input_part_2() {
        assert_eq!(
            Ok(Part2Solution { point: Point::new(90, 269), size: 16 }),
            solve_part_2("18"),
        );
        assert_eq!(
            Ok(Part2Solution { point: Point::new(232, 251), size: 12 }),
            solve_part_2("42"),
        );
    }
}