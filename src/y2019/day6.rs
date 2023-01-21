//! Day 6: Universal Orbit Map
//! https://adventofcode.com/2019/day/6

use crate::SimpleError;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::iter;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct OrbitRelation<'a> {
    orbiting: &'a str,
    orbited: &'a str,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let orbit_relations = parse_input(input)?;

    let orbit_dag = build_orbit_dag(&orbit_relations);

    let mut orbit_counts = HashMap::new();

    for orbiting_object in orbit_dag.keys() {
        compute_orbit_counts(orbiting_object, &orbit_dag, &mut orbit_counts);
    }

    Ok(orbit_counts.values().sum())
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let orbit_relations = parse_input(input)?;

    let orbit_dag = build_orbit_dag(&orbit_relations);
    let bidirectional_map = make_bidirectional(&orbit_dag);

    find_shortest_path_to_santa(&bidirectional_map)
}

fn find_shortest_path_to_santa(
    bidirectional_map: &HashMap<String, Vec<String>>,
) -> Result<usize, SimpleError> {
    if !bidirectional_map.contains_key("YOU") {
        return Err(SimpleError::new(String::from("map does not contain 'YOU'")));
    }

    let mut queue = VecDeque::new();
    let mut visited: HashSet<_> = iter::once(String::from("YOU")).collect();
    for object in bidirectional_map.get("YOU").unwrap() {
        queue.push_back((object.as_str(), 0));
        visited.insert(object.clone());
    }

    while !queue.is_empty() {
        let (object, steps) = queue.pop_front().unwrap();

        if let Some(other_objects) = bidirectional_map.get(object) {
            for other_object in other_objects {
                if other_object == "SAN" {
                    return Ok(steps);
                }

                if !visited.contains(other_object) {
                    visited.insert(other_object.clone());
                    queue.push_back((other_object.as_str(), steps + 1));
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no path found to Santa")))
}

fn make_bidirectional(orbit_dag: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut bidirectional_map: HashMap<String, Vec<String>> = HashMap::new();

    for (orbiting, orbited) in orbit_dag {
        if let Some(values) = bidirectional_map.get_mut(orbiting) {
            values.push(orbited.clone());
        } else {
            bidirectional_map.insert(orbiting.clone(), vec![orbited.clone()]);
        }

        if let Some(values) = bidirectional_map.get_mut(orbited) {
            values.push(orbiting.clone());
        } else {
            bidirectional_map.insert(orbited.clone(), vec![orbiting.clone()]);
        }
    }

    bidirectional_map
}

fn compute_orbit_counts(
    object: &str,
    orbit_dag: &HashMap<String, String>,
    orbit_counts: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&value) = orbit_counts.get(object) {
        return value;
    }

    if !orbit_dag.contains_key(object) {
        orbit_counts.insert(String::from(object), 0);
        return 0;
    }

    let orbiting = orbit_dag.get(object).unwrap();
    let orbit_count = 1 + compute_orbit_counts(orbiting, orbit_dag, orbit_counts);

    orbit_counts.insert(String::from(object), orbit_count);
    orbit_count
}

fn build_orbit_dag(orbit_relations: &[OrbitRelation]) -> HashMap<String, String> {
    orbit_relations
        .iter()
        .map(|orbit_relation| {
            (
                String::from(orbit_relation.orbiting),
                String::from(orbit_relation.orbited),
            )
        })
        .collect()
}

fn parse_input(input: &str) -> Result<Vec<OrbitRelation>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let close_paren_index = line
                .chars()
                .position(|c| c == ')')
                .ok_or_else(|| SimpleError::new(format!("line contains no ')': {line}")))?;

            let orbited = &line[..close_paren_index];
            let orbiting = &line[close_paren_index + 1..];

            Ok(OrbitRelation { orbiting, orbited })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample6.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample6-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(42), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(4), solve_part_2(SAMPLE_INPUT_2));
    }
}
