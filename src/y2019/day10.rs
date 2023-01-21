//! Day 10: Monitoring Station
//! https://adventofcode.com/2019/day/10

use std::cmp;
use std::cmp::Ordering;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: Point) -> i32 {
        (other.y - self.y).abs() + (other.x - self.x).abs()
    }

    fn angle_to(&self, other: Point) -> f64 {
        let dy = (other.y - self.y) as f64;
        let dx = (other.x - self.x) as f64;

        dy.atan2(dx)
    }
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let grid = parse_input(input)?;

    let mut max_seen = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            if !b {
                continue;
            }

            max_seen = cmp::max(max_seen, compute_visible_asteroids(&grid, i, j));
        }
    }

    Ok(max_seen)
}

fn solve_part_2(input: &str, n: usize) -> Result<usize, SimpleError> {
    let grid = parse_input(input)?;

    let mut max_seen = 0;
    let mut max_seen_i = 0;
    let mut max_seen_j = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            if !b {
                continue;
            }

            let visible_count = compute_visible_asteroids(&grid, i, j);
            if visible_count > max_seen {
                max_seen = visible_count;
                max_seen_i = i;
                max_seen_j = j;
            }
        }
    }

    let (destroyed_i, destroyed_j) = find_nth_destroyed(&grid, max_seen_i, max_seen_j, n)?;

    Ok(destroyed_j * 100 + destroyed_i)
}

fn compute_visible_asteroids(grid: &[Vec<bool>], i: usize, j: usize) -> u32 {
    let mut visible_count = 0;
    for (other_i, row) in grid.iter().enumerate() {
        for (other_j, &b) in row.iter().enumerate() {
            if !b || (other_i == i && other_j == j) {
                continue;
            }

            let di = other_i as i32 - i as i32;
            let dj = other_j as i32 - j as i32;
            let d_gcd = gcd(di.abs(), dj.abs());

            let di = di / d_gcd;
            let dj = dj / d_gcd;

            let mut ii = i as i32 + di;
            let mut jj = j as i32 + dj;
            let mut visible = true;
            while ii != other_i as i32 || jj != other_j as i32 {
                if grid[ii as usize][jj as usize] {
                    visible = false;
                    break;
                }

                ii += di;
                jj += dj;
            }

            if visible {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn gcd(a: i32, b: i32) -> i32 {
    if a > b {
        return gcd(b, a);
    }

    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

fn find_nth_destroyed(grid: &[Vec<bool>], i: usize, j: usize, n: usize) -> Result<(usize, usize), SimpleError> {
    let total_asteroids: usize = grid.iter().map(|row| {
        row.iter().filter(|&&b| b).count()
    }).sum();
    if total_asteroids < n + 1 {
        return Err(SimpleError::new(format!("input has {total_asteroids} asteroids, expected at least {}", n + 1)));
    }

    // Flip i/y value sign so that the math works out properly
    let y = -(i as i32);
    let x = j as i32;
    let laser_point = Point::new(x, y);

    let mut asteroid_positions: Vec<_> = grid.iter().enumerate()
        .flat_map(|(asteroid_i, row)| {
            row.iter().enumerate()
                .filter_map(move |(asteroid_j, &b)| {
                    if b && (asteroid_i != i || asteroid_j != j) {
                        let asteroid_y = -(asteroid_i as i32);
                        let asteroid_x = asteroid_j as i32;
                        Some(Point::new(asteroid_x, asteroid_y))
                    } else {
                        None
                    }
                })
        })
        .collect();

    asteroid_positions.sort_by(|&a, &b| {
        let a_angle = laser_point.angle_to(a);
        let b_angle = laser_point.angle_to(b);

        let angle_cmp = if (a_angle - b_angle).abs() < 1e-9 {
            Ordering::Equal
        } else {
            // Reverse so ordering is clockwise
            a_angle.total_cmp(&b_angle).reverse()
        };

        // Ensure that closer points come before farther points
        angle_cmp.then(
            laser_point.distance_to(a).cmp(&laser_point.distance_to(b))
        )
    });

    // First position where angle is pi/2 or less
    let first_position = asteroid_positions.iter().position(|&point| {
        laser_point.angle_to(point) - std::f64::consts::FRAC_PI_2 < 1e-9
    }).unwrap_or(0);
    
    let mut destroyed = vec![false; asteroid_positions.len()];

    let mut position = first_position;
    let mut last_destroyed_angle = f64::MAX;
    for _ in 0..n {
        // Advance until we reach a non-destroyed asteroid that is not at the same angle as the
        // last destroyed asteroid
        while destroyed[position] || (last_destroyed_angle - laser_point.angle_to(asteroid_positions[position])).abs() < 1e-9 {
            position = (position + 1) % asteroid_positions.len();
        }

        destroyed[position] = true;
        last_destroyed_angle = laser_point.angle_to(asteroid_positions[position]);
    }

    Ok((-asteroid_positions[position].y as usize, asteroid_positions[position].x as usize))
}

fn parse_input(input: &str) -> Result<Vec<Vec<bool>>, SimpleError> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(SimpleError::new(format!("invalid input char: {c}")))
            }
        })
            .collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input, 200)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample10.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample10-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("sample_input/sample10-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("sample_input/sample10-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("sample_input/sample10-5.txt");
    const SAMPLE_INPUT_6: &str = include_str!("sample_input/sample10-6.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(8), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(33), solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(Ok(35), solve_part_1(SAMPLE_INPUT_3));
        assert_eq!(Ok(41), solve_part_1(SAMPLE_INPUT_4));
        assert_eq!(Ok(210), solve_part_1(SAMPLE_INPUT_5));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(1501), solve_part_2(SAMPLE_INPUT_6, 9));
        assert_eq!(Ok(802), solve_part_2(SAMPLE_INPUT_5, 200));
    }
}