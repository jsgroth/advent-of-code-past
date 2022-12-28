//! Day 9: All in a Single Night
//! https://adventofcode.com/2015/day/9

use std::collections::{HashMap, HashSet};
use std::error::Error;
use crate::SimpleError;

struct Location {
    distances: HashMap<String, u32>,
}

impl Location {
    fn new() -> Self {
        Self { distances: HashMap::new() }
    }
}

fn solve_part(input: &str, reverse: bool) -> Result<u32, SimpleError> {
    let locations = parse_input(input)?;

    let minimum_distance = locations.keys().map(|start| {
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.clone());
        find_minimum_distance(&locations, visited, start, reverse)
    })
        .min_by_key(|&distance| {
            if reverse { -(distance as i32) } else { distance as i32 }
        })
        .ok_or(SimpleError::new(String::from("input should not be empty")))?;

    Ok(minimum_distance)
}

fn find_minimum_distance(locations: &HashMap<String, Location>, visited: HashSet<String>, current_location: &str, reverse: bool) -> u32 {
    if visited.len() == locations.len() {
        return 0;
    }

    let location = locations.get(current_location).unwrap();

    location.distances.iter()
        .filter(|(name, _)| !visited.contains(*name))
        .map(|(name, distance)| {
            let mut new_visited = visited.clone();
            new_visited.insert(name.clone());
            *distance + find_minimum_distance(locations, new_visited, name, reverse)
        })
        .min_by_key(|&distance| {
            if reverse { -(distance as i32) } else { distance as i32 }
        })
        .unwrap()
}

fn parse_input(input: &str) -> Result<HashMap<String, Location>, SimpleError> {
    let mut locations: HashMap<String, Location> = HashMap::new();

    for line in input.lines() {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            [a, "to", b, "=", distance] => {
                let distance: u32 = distance.parse()?;

                update_location(&mut locations, *a, *b, distance);
                update_location(&mut locations, *b, *a, distance);
            }
            _ => return Err(SimpleError::new(format!("invalid line: {line}")))
        }
    }

    Ok(locations)
}

fn update_location(locations: &mut HashMap<String, Location>, a: &str, b: &str, distance: u32) {
    if let Some(a_loc) = locations.get_mut(a) {
        a_loc.distances.insert(String::from(b), distance);
    } else {
        let mut a_loc = Location::new();
        a_loc.distances.insert(String::from(b), distance);
        locations.insert(String::from(a), a_loc);
    }
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample9.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(605), solve_part(SAMPLE_INPUT, false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(982), solve_part(SAMPLE_INPUT, true));
    }
}