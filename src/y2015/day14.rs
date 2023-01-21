//! Day 14: Reindeer Olympics
//! https://adventofcode.com/2015/day/14

use crate::SimpleError;
use std::cmp;
use std::error::Error;

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

fn solve_part_1(input: &str, target_second: u32) -> Result<u32, SimpleError> {
    let reindeer = parse_input(input)?;

    let max_distance = reindeer
        .into_iter()
        .map(|reindeer| {
            let last_fly_time = cmp::min(
                reindeer.fly_time,
                target_second % (reindeer.fly_time + reindeer.rest_time),
            );

            last_fly_time * reindeer.speed
                + target_second / (reindeer.fly_time + reindeer.rest_time)
                    * reindeer.fly_time
                    * reindeer.speed
        })
        .max()
        .unwrap_or(0);

    Ok(max_distance)
}

fn solve_part_2(input: &str, target_second: u32) -> Result<u32, SimpleError> {
    let reindeer = parse_input(input)?;

    if reindeer.is_empty() {
        return Err(SimpleError::new(String::from("no reindeer in input")));
    }

    let mut distances = vec![0; reindeer.len()];
    let mut scores = vec![0; reindeer.len()];

    for time in 0..target_second {
        for i in 0..reindeer.len() {
            let reindeer = &reindeer[i];
            if time % (reindeer.fly_time + reindeer.rest_time) < reindeer.fly_time {
                distances[i] += reindeer.speed;
            }
        }

        let max_distance = distances.iter().copied().max().unwrap();
        for i in 0..reindeer.len() {
            if distances[i] == max_distance {
                scores[i] += 1;
            }
        }
    }

    Ok(scores.into_iter().max().unwrap())
}

fn parse_input(input: &str) -> Result<Vec<Reindeer>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();

            if split.len() != 15 {
                return Err(SimpleError::new(format!("invalid line: {line}")));
            }

            let speed: u32 = split[3].parse()?;
            let fly_time: u32 = split[6].parse()?;
            let rest_time: u32 = split[13].parse()?;

            Ok(Reindeer {
                speed,
                fly_time,
                rest_time,
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 2503)?;
    let solution2 = solve_part_2(input, 2503)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample14.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(1120), solve_part_1(SAMPLE_INPUT, 1000));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(689), solve_part_2(SAMPLE_INPUT, 1000));
    }
}
