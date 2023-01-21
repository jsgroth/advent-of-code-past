//! Day 12: The N-Body Problem
//! https://adventofcode.com/2019/day/12

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::iter;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

impl Coords {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.z.signum())
    }

    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Moon {
    position: Coords,
    velocity: Coords,
}

impl Moon {
    fn new(position: Coords) -> Self {
        Self {
            position,
            velocity: Coords::new(0, 0, 0),
        }
    }
}

fn solve_part_1(input: &str, steps: usize) -> Result<i64, SimpleError> {
    let mut moons = parse_input(input)?;

    for _ in 0..steps {
        // Apply gravity
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }

                let gravity_adjustment = (moons[j].position - moons[i].position).signum();
                moons[i].velocity += gravity_adjustment;
            }
        }

        // Apply velocity
        for moon in &mut moons {
            moon.position += moon.velocity;
        }
    }

    Ok(total_energy(&moons))
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let mut moons = parse_input(input)?;

    let mut previous_x_states: HashMap<_, _> = iter::once((x_state(&moons), 0)).collect();
    let mut previous_y_states: HashMap<_, _> = iter::once((y_state(&moons), 0)).collect();
    let mut previous_z_states: HashMap<_, _> = iter::once((z_state(&moons), 0)).collect();

    let mut x_cycle_end = 0_u64;
    let mut y_cycle_end = 0_u64;
    let mut z_cycle_end = 0_u64;

    for step in 1.. {
        // Apply gravity
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }

                let gravity_adjustment = (moons[j].position - moons[i].position).signum();
                moons[i].velocity += gravity_adjustment;
            }
        }

        // Apply velocity
        for moon in &mut moons {
            moon.position += moon.velocity;
        }

        if x_cycle_end == 0 {
            let current_x_state = x_state(&moons);
            if let Some(&prev) = previous_x_states.get(&current_x_state) {
                if prev != 0 {
                    return Err(SimpleError::new(format!(
                        "expected x cycle to start at 0, starts at {prev}"
                    )));
                }
                x_cycle_end = step;
            } else {
                previous_x_states.insert(current_x_state, step);
            }
        }

        if y_cycle_end == 0 {
            let current_y_state = y_state(&moons);
            if let Some(&prev) = previous_y_states.get(&current_y_state) {
                if prev != 0 {
                    return Err(SimpleError::new(format!(
                        "expected y cycle to start at 0, starts at {prev}"
                    )));
                }
                y_cycle_end = step;
            } else {
                previous_y_states.insert(current_y_state, step);
            }
        }

        if z_cycle_end == 0 {
            let current_z_state = z_state(&moons);
            if let Some(&prev) = previous_z_states.get(&current_z_state) {
                if prev != 0 {
                    return Err(SimpleError::new(format!(
                        "expected z cycle to start at 0, starts at {prev}"
                    )));
                }
                z_cycle_end = step;
            } else {
                previous_z_states.insert(current_z_state, step);
            }
        }

        if x_cycle_end > 0 && y_cycle_end > 0 && z_cycle_end > 0 {
            return Ok(lcm(x_cycle_end, lcm(y_cycle_end, z_cycle_end)));
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn x_state(moons: &[Moon]) -> Vec<(i64, i64)> {
    moons
        .iter()
        .map(|&moon| (moon.position.x, moon.velocity.x))
        .collect()
}

fn y_state(moons: &[Moon]) -> Vec<(i64, i64)> {
    moons
        .iter()
        .map(|&moon| (moon.position.y, moon.velocity.y))
        .collect()
}

fn z_state(moons: &[Moon]) -> Vec<(i64, i64)> {
    moons
        .iter()
        .map(|&moon| (moon.position.z, moon.velocity.z))
        .collect()
}

fn lcm(a: u64, b: u64) -> u64 {
    if a < b {
        a / gcd(a, b) * b
    } else {
        b / gcd(a, b) * a
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
        return gcd(b, a);
    }

    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

fn total_energy(moons: &[Moon]) -> i64 {
    moons
        .iter()
        .map(|&moon| moon.position.energy() * moon.velocity.energy())
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Moon>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line[1..line.len() - 1].split(", ").collect();
            if split.len() != 3 {
                return Err(SimpleError::new(format!("invalid line format: {line}")));
            }

            let x = split[0][2..].parse()?;
            let y = split[1][2..].parse()?;
            let z = split[2][2..].parse()?;

            Ok(Moon::new(Coords::new(x, y, z)))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(i64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 1000)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample12.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample12-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(179), solve_part_1(SAMPLE_INPUT_1, 10));
        assert_eq!(Ok(1940), solve_part_1(SAMPLE_INPUT_2, 100));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(2772), solve_part_2(SAMPLE_INPUT_1));
        assert_eq!(Ok(4686774924), solve_part_2(SAMPLE_INPUT_2));
    }
}
