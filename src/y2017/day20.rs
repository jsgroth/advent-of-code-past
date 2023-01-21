//! Day 20: Particle Swarm
//!
//! <https://adventofcode.com/2017/day/20>

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Add, AddAssign};

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

    fn from_str(s: &str) -> Result<Self, SimpleError> {
        let s = &s[1..s.len() - 1];
        let split: Vec<_> = s.split(',').collect();
        if split.len() != 3 {
            return Err(SimpleError::new(format!("invalid coordinate string: {s}")));
        }

        Ok(Self::new(
            split[0].parse()?,
            split[1].parse()?,
            split[2].parse()?,
        ))
    }

    fn distance_to(&self, other: Coords) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
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

#[derive(Debug, Clone)]
struct Particle {
    position: Coords,
    velocity: Coords,
    acceleration: Coords,
}

impl Particle {
    fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(", ").collect();
        let (p, v, a) = match split.as_slice() {
            [p, v, a] => (p, v, a),
            _ => return Err(SimpleError::new(format!("invalid line format: {line}"))),
        };

        let position = Coords::from_str(&p[2..])?;
        let velocity = Coords::from_str(&v[2..])?;
        let acceleration = Coords::from_str(&a[2..])?;

        Ok(Self {
            position,
            velocity,
            acceleration,
        })
    }

    fn distance_from_origin(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let mut particles = parse_input(input)?;
    let mut last_particles = particles.clone();

    let destroyed_particles = vec![false; particles.len()];

    loop {
        for particle in &mut particles {
            particle.tick();
        }

        if let Some(solution) =
            check_for_solution(&particles, &last_particles, &destroyed_particles)
        {
            return Ok(solution);
        }

        last_particles = particles.clone();
    }
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let mut particles = parse_input(input)?;
    let mut last_particles = particles.clone();

    let mut destroyed_particles = vec![false; particles.len()];

    loop {
        for particle in &mut particles {
            particle.tick();
        }

        update_destroyed_particles(&particles, &mut destroyed_particles);

        if check_for_solution(&particles, &last_particles, &destroyed_particles).is_some() {
            let destroyed_count = destroyed_particles.iter().filter(|&&b| b).count();
            return Ok(particles.len() - destroyed_count);
        }

        last_particles = particles.clone();
    }
}

fn check_for_solution(
    particles: &[Particle],
    last_particles: &[Particle],
    destroyed_particles: &[bool],
) -> Option<usize> {
    for (i, (particle, last_particle)) in particles.iter().zip(last_particles).enumerate() {
        if destroyed_particles[i] {
            continue;
        }

        if particle.distance_from_origin() <= last_particle.distance_from_origin() {
            return None;
        }

        if particle.position.x.signum() != last_particle.position.x.signum()
            || particle.position.y.signum() != last_particle.position.y.signum()
            || particle.position.z.signum() != last_particle.position.z.signum()
        {
            return None;
        }
    }

    for (i, (particle, last_particle)) in particles.iter().zip(last_particles).enumerate() {
        if destroyed_particles[i] {
            continue;
        }

        for (j, (other_particle, last_other_particle)) in
            particles.iter().zip(last_particles).enumerate().skip(i + 1)
        {
            if destroyed_particles[j] {
                continue;
            }

            if particle.position.distance_to(other_particle.position)
                < last_particle
                    .position
                    .distance_to(last_other_particle.position)
            {
                return None;
            }
        }
    }

    let (min_index, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|(_, particle)| particle.distance_from_origin())
        .unwrap();
    Some(min_index)
}

fn update_destroyed_particles(particles: &[Particle], destroyed_particles: &mut [bool]) {
    let mut position_counts = HashMap::new();
    for (i, particle) in particles.iter().enumerate() {
        if destroyed_particles[i] {
            continue;
        }

        if let Some(count) = position_counts.get_mut(&particle.position) {
            *count += 1;
        } else {
            position_counts.insert(particle.position, 1);
        }
    }

    for (i, particle) in particles.iter().enumerate() {
        if !destroyed_particles[i] && position_counts.get(&particle.position).copied().unwrap() > 1
        {
            destroyed_particles[i] = true;
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Particle>, SimpleError> {
    input.lines().map(Particle::from_line).collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample20.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample20-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(0), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(1), solve_part_2(SAMPLE_INPUT_2));
    }
}
