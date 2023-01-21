//! Day 8: Memory Maneuver
//! https://adventofcode.com/2018/day/8

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let numbers = parse_input(input)?;

    let metadata_sum = compute_metadata_sum(&mut numbers.into_iter());

    Ok(metadata_sum)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let numbers = parse_input(input)?;

    let root_node_value = compute_node_value(&mut numbers.into_iter());

    Ok(root_node_value)
}

fn compute_metadata_sum(iter: &mut impl Iterator<Item = u32>) -> u32 {
    let child_nodes = iter.next().unwrap();
    let metadata_entries = iter.next().unwrap();

    let mut metadata_sum = 0;
    for _ in 0..child_nodes {
        metadata_sum += compute_metadata_sum(iter);
    }

    for _ in 0..metadata_entries {
        metadata_sum += iter.next().unwrap();
    }

    metadata_sum
}

fn compute_node_value(iter: &mut impl Iterator<Item = u32>) -> u32 {
    let num_child_nodes = iter.next().unwrap();
    let num_metadata_entries = iter.next().unwrap();

    let mut child_nodes = Vec::new();
    for _ in 0..num_child_nodes {
        child_nodes.push(compute_node_value(iter));
    }

    let mut metadata_entries = Vec::new();
    for _ in 0..num_metadata_entries {
        metadata_entries.push(iter.next().unwrap());
    }

    if num_child_nodes == 0 {
        return metadata_entries.into_iter().sum();
    }

    let mut node_value = 0;
    for &node_index in &metadata_entries {
        if node_index >= 1 && node_index <= num_child_nodes {
            node_value += child_nodes[(node_index - 1) as usize];
        }
    }
    node_value
}

fn parse_input(input: &str) -> Result<Vec<u32>, SimpleError> {
    let line = crate::read_single_line(input)?;
    line.split(' ')
        .map(|word| word.parse::<u32>().map_err(SimpleError::from))
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(138), solve_part_1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(66), solve_part_2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    }
}
